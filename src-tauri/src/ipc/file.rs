// src-tauri/src/ipc/file.rs
//
/// 文件相关 IPC 接口（薄层 - 只做参数转换和错误映射）
use crate::core::file::service::FileService;
use crate::core::file::transfer::{FileReceiver, FileSender};
use crate::database::handler::{FileStorageHandler, TransferStateHandler, UserHandler};
use crate::types::{MapErrToFrontend, PendingTransfer};
use sea_orm::DbConn;
use tauri::State;
use tracing::error;

/// 发送文件请求
#[tauri::command]
pub async fn send_file_request_handler(
    file_paths: Vec<String>,
    target_ip: String,
    owner_uid: i64,
    db: State<'_, DbConn>,
) -> Result<i64, String> {
    FileService::send_file_request(db.inner(), file_paths, target_ip, owner_uid)
        .await
        .map_err_to_frontend()
}

/// 接收文件请求（响应）
#[tauri::command]
pub async fn accept_file_request_handler(
    packet_no: String,
    file_id: u64,
    offset: u64,
    target_ip: String,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    FileService::accept_file(db.inner(), packet_no, file_id, offset, target_ip)
        .await
        .map_err_to_frontend()
}

/// 拒绝文件请求
#[tauri::command]
pub async fn reject_file_request_handler(
    packet_no: String,
    target_ip: String,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    FileService::reject_file(db.inner(), packet_no, target_ip)
        .await
        .map_err_to_frontend()
}

/// 获取文件信息
#[tauri::command]
pub async fn get_file_handler(fid: i64, db: State<'_, DbConn>) -> Result<String, String> {
    let db = db.inner();
    let file_storage = FileStorageHandler::find_by_id(db, fid).await.map_err_to_frontend()?;
    // serde_json::Error doesn't implement Into<AppError>, use standard error handling
    Ok(serde_json::to_string(&file_storage).map_err(|e| e.to_string())?)
}

/// 取消文件传输
#[tauri::command]
pub async fn cancel_upload_handler(fid: i64, db: State<'_, DbConn>) -> Result<(), String> {
    FileService::cancel_transfer(db.inner(), fid).await.map_err_to_frontend()
}

/// 获取待恢复的传输列表
#[tauri::command]
pub async fn get_pending_transfers_handler(db: State<'_, DbConn>) -> Result<Vec<PendingTransfer>, String> {
    FileService::get_pending_transfers(db.inner()).await.map_err_to_frontend()
}

/// 恢复传输
#[tauri::command]
pub async fn resume_transfer_handler(tid: i64, db: State<'_, DbConn>) -> Result<(), String> {
    // 获取传输信息
    let transfer = TransferStateHandler::find_by_id(db.inner(), tid)
        .await
        .map_err_to_frontend()?
        .ok_or_else(|| {
            // 手动构造 NotFound 错误
            use crate::types::ErrorCode;
            use crate::types::FrontendError;
            FrontendError {
                code: ErrorCode::NotFound,
                message: format!("传输不存在: {}", tid),
                details: None,
            }
            .to_json()
        })?;

    // 检查传输状态
    if transfer.status != 0 && transfer.status != 1 {
        return Err({
            use crate::types::ErrorCode;
            use crate::types::FrontendError;
            FrontendError {
                code: ErrorCode::Business,
                message: "传输状态无法恢复".to_string(),
                details: Some(format!("当前状态: {}", transfer.status)),
            }
            .to_json()
        });
    }

    // 获取目标用户信息
    let _target_user = UserHandler::find_by_id(db.inner(), transfer.target_id)
        .await
        .map_err_to_frontend()?;

    // 获取文件存储信息
    let file_storage = FileStorageHandler::find_by_id(db.inner(), transfer.file_id)
        .await
        .map_err_to_frontend()?;

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
