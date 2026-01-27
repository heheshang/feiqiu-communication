// src-tauri/src/ipc/group.rs
//
/// 群组相关 IPC 接口
/// TODO: Phase 7 时实现完整功能

/// 创建群组
#[tauri::command]
pub async fn create_group_handler(
    _group_name: String,
    _creator_uid: i64,
    _member_uids: Vec<i64>,
) -> Result<i64, String> {
    // TODO: 实现创建群组
    Ok(0)
}

/// 获取群组信息
#[tauri::command]
pub async fn get_group_info_handler(
    _gid: i64,
) -> Result<crate::types::GroupInfo, String> {
    // TODO: 实现获取群组信息
    Err("Not implemented".to_string())
}

/// 获取群成员列表
#[tauri::command]
pub async fn get_group_members_handler(
    _gid: i64,
) -> Result<Vec<crate::types::GroupMember>, String> {
    // TODO: 实现获取群成员
    Ok(vec![])
}
