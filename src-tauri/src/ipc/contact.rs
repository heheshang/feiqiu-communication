// src-tauri/src/ipc/contact.rs
//
/// 通讯录相关 IPC 接口
/// TODO: Phase 4 时实现完整功能

/// 获取通讯录列表
#[tauri::command]
pub async fn get_contact_list_handler(
    _owner_uid: i64,
) -> Result<Vec<crate::types::Contact>, String> {
    // TODO: 实现获取通讯录
    Ok(vec![])
}

/// 获取在线用户列表
#[tauri::command]
pub async fn get_online_users_handler(
) -> Result<Vec<crate::types::UserInfo>, String> {
    // TODO: 实现获取在线用户
    Ok(vec![])
}
