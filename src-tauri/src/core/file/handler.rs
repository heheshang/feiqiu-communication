// src-tauri/src/core/file/handler.rs
//
//! 文件传输事件处理器
//!
//! 处理来自网络层的文件传输事件：
//! - FileDataRequest: 对方请求文件数据块
//! - FileDataReceived: 接收到文件数据块
//! - FileRelease: 文件传输释放/取消

use crate::core::file::transfer::{FileReceiver, FileSender};
use crate::database::handler::{FileStorageHandler, TransferStateHandler};
use crate::error::{AppError, AppResult};
use crate::network::feiq::packer::FeiQPacket;
use crate::network::udp::sender;
use sea_orm::DbConn;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tracing::{error, info, warn};

/// 文件传输事件处理器
pub struct FileTransferHandler;

/// 文件接收器缓存 (用于分块接收)
///
/// key: (packet_no, file_id)
/// value: FileReceiver
lazy_static::lazy_static! {
    static ref FILE_RECEIVERS: Mutex<HashMap<(String, u64), FileReceiver>> = Mutex::new(HashMap::new());
}

impl FileTransferHandler {
    /// 处理文件数据请求事件
    ///
    /// 当远程用户请求文件数据块时，读取本地文件并发送
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `from_ip`: 请求者IP
    /// - `packet_no`: 数据包编号
    /// - `file_id`: 文件ID
    /// - `offset`: 偏移量
    pub async fn handle_file_data_request(
        db: &DbConn,
        from_ip: &str,
        packet_no: &str,
        file_id: u64,
        offset: u64,
    ) -> AppResult<()> {
        info!(
            "收到文件数据请求: from_ip={}, packet_no={}, file_id={}, offset={}",
            from_ip, packet_no, file_id, offset
        );

        // 查询传输状态记录
        let transfer_states = TransferStateHandler::find_by_packet_no(db, packet_no).await?;

        // 找到对应的文件传输记录
        let transfer_state = transfer_states
            .iter()
            .find(|t| t.file_id as u64 == file_id)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "找不到传输记录: packet_no={}, file_id={}",
                    packet_no, file_id
                ))
            })?;

        // 获取文件路径
        let file_storage = FileStorageHandler::find_by_id(db, transfer_state.file_id).await?;

        // 读取文件数据块 (4KB)
        let chunk = Self::read_file_chunk(&file_storage.file_path, offset).await?;

        // 构建并发送 FeiQ 文件数据包
        let packet = FeiQPacket::make_feiq_file_data_packet(packet_no, file_id, offset, &chunk, None);
        let packet_str = packet.to_feiq_string();

        let addr = format!("{}:{}", from_ip, 2425);
        sender::send_packet_data(&addr, &packet_str)
            .await
            .map_err(|e| AppError::Network(format!("发送文件数据块失败: {}", e)))?;

        info!(
            "已发送文件数据块: file_id={}, offset={}, size={}",
            file_id,
            offset,
            chunk.len()
        );

        Ok(())
    }

    /// 处理文件数据接收事件
    ///
    /// 当接收到文件数据块时，写入本地文件并更新进度
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `from_ip`: 发送者IP
    /// - `packet_no`: 数据包编号
    /// - `file_id`: 文件ID
    /// - `offset`: 偏移量
    /// - `data`: Base64编码的文件数据
    pub async fn handle_file_data_received(
        db: &DbConn,
        from_ip: &str,
        packet_no: &str,
        file_id: u64,
        offset: u64,
        data: &str,
    ) -> AppResult<()> {
        info!(
            "收到文件数据: from_ip={}, packet_no={}, file_id={}, offset={}, size={}",
            from_ip,
            packet_no,
            file_id,
            offset,
            data.len()
        );

        // 解码 Base64 数据
        use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
        let chunk = BASE64
            .decode(data)
            .map_err(|e| AppError::Protocol(format!("Base64 解码失败: {}", e)))?;

        // 查询传输状态记录
        let transfer_states = TransferStateHandler::find_by_packet_no(db, packet_no).await?;

        let transfer_state = transfer_states
            .iter()
            .find(|t| t.file_id as u64 == file_id)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "找不到传输记录: packet_no={}, file_id={}",
                    packet_no, file_id
                ))
            })?;

        // 获取或创建 FileReceiver
        let key = (packet_no.to_string(), file_id);
        let mut receivers = FILE_RECEIVERS.lock().map_err(|e| {
            AppError::Business(format!("获取文件接收器缓存失败: {}", e))
        })?;

        let receiver = receivers.entry(key.clone()).or_insert_with(|| {
            // 创建新的 FileReceiver
            FileReceiver::new(
                transfer_state.get_save_path(),
                file_id,
                transfer_state.file_size as u64,
            )
        });

        // 写入数据块
        receiver.receive_chunk(offset, &chunk)?;

        // 更新传输进度
        let new_offset = offset + chunk.len() as u64;
        TransferStateHandler::update_progress(db, transfer_state.tid, new_offset as i64, 1).await?;

        // 检查是否完成
        if receiver.current_size()? >= transfer_state.file_size as u64 {
            info!(
                "文件接收完成: file_id={}, size={}",
                file_id, transfer_state.file_size
            );

            // 标记传输完成
            TransferStateHandler::update_status(db, transfer_state.tid, 2, None).await?;

            // 移除接收器缓存
            receivers.remove(&key);
        }

        Ok(())
    }

    /// 处理文件释放事件
    ///
    /// 当文件传输被取消或释放时，清理相关资源
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `from_ip`: 发送者IP
    /// - `packet_no`: 数据包编号
    pub async fn handle_file_release(db: &DbConn, from_ip: &str, packet_no: &str) -> AppResult<()> {
        info!(
            "收到文件释放: from_ip={}, packet_no={}",
            from_ip, packet_no
        );

        // 查询传输状态记录
        let transfer_states = TransferStateHandler::find_by_packet_no(db, packet_no).await?;

        for transfer_state in transfer_states {
            // 标记传输为已取消
            TransferStateHandler::update_status(db, transfer_state.tid, -2, Some("对方取消传输".to_string())).await?;

            // 移除文件接收器缓存
            let mut receivers = FILE_RECEIVERS.lock().map_err(|e| {
                AppError::Business(format!("获取文件接收器缓存失败: {}", e))
            })?;

            // 移除所有与该 packet_no 相关的接收器
            receivers.retain(|(p, _)| p != packet_no);
        }

        info!("文件传输已清理: packet_no={}", packet_no);

        Ok(())
    }

    /// 读取文件数据块
    ///
    /// # 参数
    /// - `file_path`: 文件路径
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// 返回文件数据块 (最大 4KB)
    async fn read_file_chunk(file_path: &str, offset: u64) -> AppResult<Vec<u8>> {
        use tokio::fs::File;
        use tokio::io::{AsyncReadExt, AsyncSeekExt};

        let mut file = File::open(file_path)
            .await
            .map_err(|e| AppError::Io(e))?;

        // 定位到指定偏移量
        file.seek(io::SeekFrom::Start(offset))
            .await
            .map_err(|e| AppError::Io(e))?;

        // 读取最多 4KB 数据
        let mut buffer = vec![0u8; 4096];
        let n = file
            .read(&mut buffer)
            .await
            .map_err(|e| AppError::Io(e))?;

        buffer.truncate(n);
        Ok(buffer)
    }
}

/// 为 TransferState 添加辅助方法
trait TransferStateExt {
    fn get_save_path(&self) -> String;
}

impl TransferStateExt for crate::database::model::transfer_state::Model {
    fn get_save_path(&self) -> String {
        // 使用下载目录 + 文件名
        // TODO: 从配置获取下载目录
        format!("/tmp/feiqiu_download_{}", self.file_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_structure() {
        // 测试模块结构是否正确
        assert!(true);
    }
}
