// src-tauri/src/core/file/resume.rs
//
//! 文件传输恢复逻辑

use crate::core::file::transfer::{FileReceiver, FileSender};
use crate::database::handler::transfer_state::TransferStateHandler;
use crate::error::AppResult;
use sea_orm::DbConn;

/// 恢复传输
pub async fn resume_transfers(db: &DbConn) -> AppResult<Vec<ResumeInfo>> {
    let pending = TransferStateHandler::find_pending(db).await?;
    let mut resume_infos = Vec::new();

    for state in pending {
        let info = ResumeInfo {
            tid: state.tid,
            file_id: state.file_id,
            file_path: String::new(), // TODO: 从 file_storage 查询
            offset: state.transferred as u64,
            total: state.file_size as u64,
            target_addr: format!("{}:{}", state.target_ip, state.target_port),
            packet_no: state.packet_no,
            checksum: state.checksum,
            direction: state.direction,
        };
        resume_infos.push(info);
    }

    Ok(resume_infos)
}

/// 恢复信息
pub struct ResumeInfo {
    pub tid: i64,
    pub file_id: i64,
    pub file_path: String,
    pub offset: u64,
    pub total: u64,
    pub target_addr: String,
    pub packet_no: String,
    pub checksum: String,
    pub direction: i8, // 0=下载, 1=上传
}

/// 创建文件传输状态记录
pub async fn create_transfer_state(
    db: &DbConn,
    file_id: i64,
    session_type: i8,
    target_id: i64,
    direction: i8,
    file_size: i64,
    packet_no: &str,
    target_ip: &str,
    target_port: u16,
    checksum: &str,
) -> AppResult<i64> {
    use crate::database::model::transfer_state;
    use sea_orm::ActiveValue;

    let transfer_state = transfer_state::ActiveModel {
        tid: ActiveValue::NotSet,
        file_id: ActiveValue::Set(file_id),
        session_type: ActiveValue::Set(session_type),
        target_id: ActiveValue::Set(target_id),
        direction: ActiveValue::Set(direction),
        transferred: ActiveValue::Set(0),
        file_size: ActiveValue::Set(file_size),
        status: ActiveValue::Set(0), // 等待中
        packet_no: ActiveValue::Set(packet_no.to_string()),
        target_ip: ActiveValue::Set(target_ip.to_string()),
        target_port: ActiveValue::Set(target_port),
        checksum: ActiveValue::Set(checksum.to_string()),
        error_message: ActiveValue::NotSet,
        update_time: ActiveValue::Set(chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string()),
        create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
    };

    let result = TransferStateHandler::create(db, transfer_state).await?;
    Ok(result.tid)
}

/// 更新传输进度
pub async fn update_transfer_progress(db: &DbConn, tid: i64, transferred: i64, status: i8) -> AppResult<()> {
    TransferStateHandler::update_progress(db, tid, transferred, status).await?;
    Ok(())
}

/// 完成传输
pub async fn complete_transfer(db: &DbConn, tid: i64) -> AppResult<()> {
    TransferStateHandler::update_status(db, tid, 2, None).await?;
    Ok(())
}

/// 失败传输
pub async fn fail_transfer(db: &DbConn, tid: i64, error: &str) -> AppResult<()> {
    TransferStateHandler::update_status(db, tid, -1, Some(error.to_string())).await?;
    Ok(())
}

/// 取消传输
pub async fn cancel_transfer(db: &DbConn, tid: i64) -> AppResult<()> {
    TransferStateHandler::update_status(db, tid, -2, Some("Cancelled by user".to_string())).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resume_info_creation() {
        let info = ResumeInfo {
            tid: 1,
            file_id: 100,
            file_path: "/path/to/file.txt".to_string(),
            offset: 1024,
            total: 10240,
            target_addr: "192.168.1.100:2425".to_string(),
            packet_no: "12345".to_string(),
            checksum: "abc123".to_string(),
            direction: 1,
        };

        assert_eq!(info.tid, 1);
        assert_eq!(info.offset, 1024);
        assert_eq!(info.total, 10240);
    }
}
