// src-tauri/src/ipc/file.rs
//
/// 文件相关 IPC 接口

/// 发送文件请求
#[tauri::command]
pub async fn send_file_request_handler(
    file_paths: Vec<String>,
    target_ip: String,
    owner_uid: i64,
) -> Result<i64, String> {
    // TODO: Phase 6.2 - 实现完整的文件请求发送
    // 1. 构建 FileAttachment 列表
    // 2. 创建 FeiqPacket::make_file_attach_packet
    // 3. 通过 UDP 发送
    // 4. 保存到数据库
    Ok(0)
}

/// 接收文件请求（响应）
#[tauri::command]
pub async fn accept_file_request_handler(
    packet_no: String,
    file_id: u64,
    offset: u64,
    target_ip: String,
) -> Result<(), String> {
    // TODO: Phase 6.2 - 发送 IPMSG_GETFILEDATA 包
    Ok(())
}

/// 拒绝文件请求
#[tauri::command]
pub async fn reject_file_request_handler(packet_no: String, target_ip: String) -> Result<(), String> {
    // TODO: Phase 6.2 - 发送 IPMSG_RELEASEFILES 包
    Ok(())
}

/// 获取文件信息
#[tauri::command]
pub async fn get_file_handler(fid: i64) -> Result<String, String> {
    Err("Not implemented".to_string())
}

/// 取消文件传输
#[tauri::command]
pub async fn cancel_upload_handler(fid: i64) -> Result<(), String> {
    // TODO: Phase 6.2 - 实现取消传输逻辑
    Ok(())
}

/// 获取待恢复的传输列表
#[tauri::command]
pub async fn get_pending_transfers_handler() -> Result<Vec<crate::types::PendingTransfer>, String> {
    // TODO: Phase 6.4 - 从数据库获取待恢复的传输
    // 1. 查询 transfer_state 表中 status = 0 或 1 的记录
    // 2. 关联 file_storage 表获取文件信息
    Ok(vec![])
}

/// 恢复传输
#[tauri::command]
pub async fn resume_transfer_handler(tid: i64) -> Result<(), String> {
    // TODO: Phase 6.4 - 恢复指定的传输
    // 1. 从 transfer_state 获取传输信息
    // 2. 根据方向创建 FileSender 或 FileReceiver
    // 3. 从保存的 offset 开始传输
    Ok(())
}
