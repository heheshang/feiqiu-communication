// src-tauri/src/ipc/file.rs
//
/// 文件相关 IPC 接口
use crate::core::file::request::{create_file_attach_request, create_file_data_request, create_file_release};
use crate::core::file::transfer::{FileReceiver, FileSender};
use crate::database::handler::{FileStorageHandler, TransferStateHandler, UserHandler};
use crate::network::feiq::model::FileAttachment;
use crate::network::udp::sender;
use crate::types::{PendingTransfer, TransferStatus};
use sea_orm::DbConn;
use tauri::State;
use tracing::error;

use std::path::Path;

/// 发送文件请求
#[tauri::command]
pub async fn send_file_request_handler(
    file_paths: Vec<String>,
    target_ip: String,
    owner_uid: i64,
    db: State<'_, DbConn>,
) -> Result<i64, String> {
    let db = db.inner();

    // 获取目标用户信息
    let target_user = UserHandler::find_by_ip_port(db, &target_ip, 2425)
        .await
        .map_err(|e| format!("查找目标用户失败: {}", e))?
        .ok_or_else(|| format!("未找到目标用户: {}", target_ip))?;

    // 构建 FileAttachment 列表
    let mut files = Vec::new();
    for path in &file_paths {
        let path_obj = Path::new(path);
        let metadata = path_obj.metadata().map_err(|e| format!("读取文件失败: {}", e))?;

        files.push(FileAttachment {
            file_name: path_obj
                .file_name()
                .ok_or_else(|| "未知文件".to_string())?
                .to_string_lossy()
                .to_string(),
            file_size: metadata.len() as i64,
            mtime: metadata
                .modified()
                .map_err(|e| format!("获取文件时间失败: {}", e))?
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| format!("获取文件时间失败: {}", e))?
                .as_secs() as u64,
            attr: if path_obj.is_dir() { 2 } else { 1 },
        });
    }

    // 创建文件附件包
    let receiver = format!("{}:{}", target_ip, target_user.feiq_port);
    let packet = create_file_attach_request(&files, &target_ip, target_user.feiq_port as u16);
    let packet_str = packet.to_string();

    // 发送 UDP 包
    sender::send_packet_data(&receiver, &packet_str)
        .await
        .map_err(|e| format!("发送文件请求失败: {}", e))?;

    // 保存到数据库 - 创建文件存储记录
    let transfer_id = chrono::Utc::now().timestamp();
    let mut file_ids = Vec::new();

    for file in &files {
        let file_storage = FileStorageHandler::create(
            db,
            file.file_name.clone(),
            file_paths.get(0).unwrap_or(&String::new()).clone(),
            file.file_size as i64,
            "application/octet-stream".to_string(),
            owner_uid,
        )
        .await
        .map_err(|e| format!("保存文件信息失败: {}", e))?;

        file_ids.push(file_storage.fid);
    }

    // 创建传输状态记录 (待对方接受)
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
            file_size: Set(files[index].file_size as i64),
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

    Ok(transfer_id)
}

/// 接收文件请求（响应）
#[tauri::command]
pub async fn accept_file_request_handler(
    packet_no: String,
    file_id: u64,
    offset: u64,
    target_ip: String,
) -> Result<(), String> {
    // 创建文件数据请求包
    let packet = create_file_data_request(&packet_no, file_id, offset);

    // 发送 GETFILEDATA 包
    let addr = format!("{}:{}", target_ip, 2425);
    let packet_str = packet.to_string();

    sender::send_packet_data(&addr, &packet_str)
        .await
        .map_err(|e| format!("发送文件数据请求失败: {}", e))?;

    Ok(())
}

/// 拒绝文件请求
#[tauri::command]
pub async fn reject_file_request_handler(packet_no: String, target_ip: String) -> Result<(), String> {
    // 创建文件释放包
    let packet = create_file_release(&packet_no);

    // 发送 RELEASEFILES 包
    let addr = format!("{}:{}", target_ip, 2425);
    let packet_str = packet.to_string();

    sender::send_packet_data(&addr, &packet_str)
        .await
        .map_err(|e| format!("发送文件拒绝包失败: {}", e))?;

    Ok(())
}

/// 获取文件信息
#[tauri::command]
pub async fn get_file_handler(fid: i64, db: State<'_, DbConn>) -> Result<String, String> {
    let db = db.inner();

    let file_storage = FileStorageHandler::find_by_id(db, fid)
        .await
        .map_err(|e| format!("查找文件失败: {}", e))?;

    Ok(serde_json::to_string(&file_storage).map_err(|e| e.to_string())?)
}

/// 取消文件传输
#[tauri::command]
pub async fn cancel_upload_handler(fid: i64, db: State<'_, DbConn>) -> Result<(), String> {
    // 更新传输状态为已取消
    TransferStateHandler::update_status(db.inner(), fid, -2, Some("已取消".to_string()))
        .await
        .map_err(|e| format!("取消传输失败: {}", e))?;

    Ok(())
}

/// 获取待恢复的传输列表
#[tauri::command]
pub async fn get_pending_transfers_handler(db: State<'_, DbConn>) -> Result<Vec<PendingTransfer>, String> {
    // 查询待恢复的传输 (status = 0 或 1)
    let transfers = TransferStateHandler::find_pending(db.inner())
        .await
        .map_err(|e| format!("查询待恢复传输失败: {}", e))?;

    // 转换为前端类型
    let mut result: Vec<PendingTransfer> = Vec::new();

    for t in transfers {
        // 查找关联的文件信息
        let file_storage = FileStorageHandler::find_by_id(db.inner(), t.file_id)
            .await
            .map_err(|e| format!("查找文件失败: {}", e))?;

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

/// 恢复传输
#[tauri::command]
pub async fn resume_transfer_handler(tid: i64, db: State<'_, DbConn>) -> Result<(), String> {
    // 获取传输信息
    let transfer = TransferStateHandler::find_by_id(db.inner(), tid)
        .await
        .map_err(|e| format!("查询传输失败: {}", e))?
        .ok_or_else(|| format!("传输不存在: {}", tid))?;

    // 检查传输状态
    if transfer.status != 0 && transfer.status != 1 {
        return Err(format!("传输状态无法恢复: {}", transfer.status));
    }

    // 获取目标用户信息
    let _target_user = UserHandler::find_by_id(db.inner(), transfer.target_id)
        .await
        .map_err(|e| format!("查找目标用户失败: {}", e))?;

    // 获取文件存储信息
    let file_storage = FileStorageHandler::find_by_id(db.inner(), transfer.file_id)
        .await
        .map_err(|e| format!("查找文件失败: {}", e))?;

    // 根据传输方向创建相应的处理器
    if transfer.direction == 1 {
        // 上传方向: 创建 FileSender
        let sender = FileSender::new(
            file_storage.file_path.clone(),
            transfer.file_id as u64,
            format!("{}:{}", transfer.target_ip, transfer.target_port),
            transfer.packet_no,
        );

        // 在后台任务中执行发送
        tokio::spawn(async move {
            // TODO: 实现进度回调
            if let Err(e) = sender.send().await {
                error!("文件发送失败: {}", e);
            }
        });
    } else {
        // 接收方向: 创建 FileReceiver
        let _receiver = FileReceiver::new(
            file_storage.file_path.clone(),
            transfer.file_id as u64,
            transfer.file_size as u64,
        );

        // TODO: 实现接收逻辑（需要从网络获取数据块）
        tracing::info!("File receiver created for: {}", file_storage.file_path);
    }

    Ok(())
}
