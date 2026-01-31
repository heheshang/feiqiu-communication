//! 文件业务逻辑服务层
//!
//! FileService 提供文件传输相关的业务逻辑操作，包括：
//! - 发送文件请求
//! - 接受文件传输
//! - 拒绝文件传输
//! - 取消文件传输

use crate::core::file::request::{create_file_attach_request, create_file_data_request, create_file_release};
use crate::database::handler::{FileStorageHandler, TransferStateHandler, UserHandler};
use crate::error::{AppError, AppResult};
use crate::network::udp::sender;
use crate::types::{PendingTransfer, TransferStatus};
use sea_orm::DbConn;
use tracing::info;

/// 文件服务
pub struct FileService;

impl FileService {
    /// 发送文件请求
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `file_paths`: 文件路径列表
    /// - `target_ip`: 目标IP地址
    /// - `owner_uid`: 发送者用户ID
    ///
    /// # 返回
    /// 返回文件传输ID
    pub async fn send_file_request(
        db: &DbConn,
        file_paths: Vec<String>,
        target_ip: String,
        owner_uid: i64,
    ) -> AppResult<i64> {
        // 获取目标用户信息
        let target_user = UserHandler::find_by_ip_port(db, &target_ip, 2425)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("未找到目标用户: {}", target_ip)))?;

        // 构建文件附件列表
        let mut files = Vec::new();
        for path in &file_paths {
            use std::path::Path;
            let path_obj = Path::new(path);
            let metadata = path_obj.metadata().map_err(AppError::Io)?;

            files.push(crate::network::feiq::model::FileAttachment {
                file_name: path_obj
                    .file_name()
                    .ok_or_else(|| AppError::Business("未知文件".to_string()))?
                    .to_string_lossy()
                    .to_string(),
                file_size: metadata.len() as i64,
                mtime: metadata
                    .modified()
                    .map_err(AppError::Io)?
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| AppError::Business(e.to_string()))?
                    .as_secs(),
                attr: if path_obj.is_dir() { 2 } else { 1 },
            });
        }

        // 创建文件附件包
        let receiver = format!("{}:{}", target_ip, target_user.feiq_port);
        let packet = create_file_attach_request(&files, &target_ip, target_user.feiq_port as u16);
        let packet_str = packet.to_feiq_string();

        // 发送 UDP 包
        sender::send_packet_data(&receiver, &packet_str)
            .await
            .map_err(|e| AppError::Network(format!("发送文件请求失败: {}", e)))?;

        // 保存到数据库 - 创建文件存储记录
        let transfer_id = chrono::Utc::now().timestamp();
        let mut file_ids = Vec::new();

        for file in &files {
            let file_storage = FileStorageHandler::create(
                db,
                file.file_name.clone(),
                file_paths.first().unwrap_or(&String::new()).clone(),
                file.file_size,
                "application/octet-stream".to_string(),
                owner_uid,
            )
            .await?;

            file_ids.push(file_storage.fid);
        }

        // 创建传输状态记录
        for (index, file_storage) in file_ids.iter().enumerate() {
            use crate::database::model::transfer_state;
            use sea_orm::ActiveValue::*;

            let transfer_model = transfer_state::ActiveModel {
                tid: NotSet,
                file_id: Set(*file_storage),
                session_type: Set(0), // 单聊
                target_id: Set(target_user.uid),
                direction: Set(1), // 1=上传
                transferred: Set(0),
                file_size: Set(files[index].file_size),
                status: Set(0), // 0=等待对方接受
                packet_no: Set(transfer_id.to_string()),
                target_ip: Set(target_ip.clone()),
                target_port: Set(target_user.feiq_port as u16),
                checksum: Set(String::new()),
                error_message: NotSet,
                update_time: Set(chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string()),
                create_time: Set(chrono::Utc::now().naive_utc()),
            };

            let _ = TransferStateHandler::create(db, transfer_model).await;
        }

        info!(
            "文件请求已发送: transfer_id={}, files_count={}",
            transfer_id,
            file_paths.len()
        );

        Ok(transfer_id)
    }

    /// 接受文件传输
    ///
    /// # 参数
    /// - `packet_no`: 包编号
    /// - `file_id`: 文件ID
    /// - `offset`: 偏移量
    /// - `target_ip`: 目标IP
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn accept_file(
        _db: &DbConn,
        packet_no: String,
        file_id: u64,
        offset: u64,
        target_ip: String,
    ) -> AppResult<()> {
        // 创建文件数据请求包
        let packet = create_file_data_request(&packet_no, file_id, offset);

        // 发送 GETFILEDATA 包
        let addr = format!("{}:{}", target_ip, 2425);
        let packet_str = packet.to_feiq_string();

        sender::send_packet_data(&addr, &packet_str)
            .await
            .map_err(|e| AppError::Network(format!("发送文件数据请求失败: {}", e)))?;

        info!("文件传输已接受: packet_no={}, file_id={}", packet_no, file_id);

        Ok(())
    }

    /// 拒绝文件传输
    ///
    /// # 参数
    /// - `packet_no`: 包编号
    /// - `target_ip`: 目标IP
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn reject_file(_db: &DbConn, packet_no: String, target_ip: String) -> AppResult<()> {
        // 创建文件释放包
        let packet = create_file_release(&packet_no);

        // 发送 RELEASEFILES 包
        let addr = format!("{}:{}", target_ip, 2425);
        let packet_str = packet.to_feiq_string();

        sender::send_packet_data(&addr, &packet_str)
            .await
            .map_err(|e| AppError::Network(format!("发送文件拒绝包失败: {}", e)))?;

        info!("文件传输已拒绝: packet_no={}", packet_no);

        Ok(())
    }

    /// 取消文件传输
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `transfer_id`: 传输ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn cancel_transfer(db: &DbConn, transfer_id: i64) -> AppResult<()> {
        // 更新传输状态为已取消
        TransferStateHandler::update_status(db, transfer_id, -2, Some("已取消".to_string())).await?;

        info!("文件传输已取消: transfer_id={}", transfer_id);

        Ok(())
    }

    /// 获取待传输的文件列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    ///
    /// # 返回
    /// 返回待传输文件列表
    pub async fn get_pending_transfers(db: &DbConn) -> AppResult<Vec<PendingTransfer>> {
        // 查询待恢复的传输 (status = 0 或 1)
        let transfers = TransferStateHandler::find_pending(db).await?;

        // 转换为前端类型
        let mut result = Vec::new();

        for t in transfers {
            // 查找关联的文件信息
            let file_storage = FileStorageHandler::find_by_id(db, t.file_id).await?;

            result.push(PendingTransfer {
                tid: t.tid,
                file_id: t.file_id,
                file_name: file_storage.file_name,
                file_path: file_storage.file_path,
                transferred: t.transferred,
                file_size: t.file_size,
                status: if t.status == 0 {
                    TransferStatus::Pending
                } else if t.status == 1 {
                    TransferStatus::Transferring
                } else if t.status == 2 {
                    TransferStatus::Completed
                } else if t.status == -1 {
                    TransferStatus::Failed
                } else {
                    TransferStatus::Cancelled
                },
                target_ip: t.target_ip,
                direction: t.direction,
            });
        }

        Ok(result)
    }

    /// 更新文件传输进度
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `transfer_id`: 传输ID
    /// - `transferred`: 已传输字节数
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn update_transfer_progress(db: &DbConn, transfer_id: i64, _transferred: i64) -> AppResult<()> {
        // 直接使用handler的update_status方法
        TransferStateHandler::update_status(db, transfer_id, 1, None).await?;
        Ok(())
    }
}
