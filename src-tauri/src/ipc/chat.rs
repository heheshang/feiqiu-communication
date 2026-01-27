// src-tauri/src/ipc/chat.rs
//
/// 聊天相关 IPC 接口
/// TODO: Phase 4 时实现完整功能

/// 获取聊天历史记录
#[tauri::command]
pub async fn get_chat_history_handler(
    _session_type: i8,
    _target_id: i64,
    _page: i32,
    _page_size: i32,
) -> Result<Vec<crate::types::ChatMessage>, String> {
    // TODO: 实现获取聊天历史
    Ok(vec![])
}

/// 发送文本消息
#[tauri::command]
pub async fn send_text_message_handler(
    _session_type: i8,
    _target_id: i64,
    _content: String,
) -> Result<i64, String> {
    // TODO: 实现发送消息
    Ok(0)
}

/// 获取会话列表
#[tauri::command]
pub async fn get_session_list_handler(
    _owner_uid: i64,
) -> Result<Vec<crate::types::ChatSession>, String> {
    // TODO: 实现获取会话列表
    Ok(vec![])
}

/// 标记消息已读
#[tauri::command]
pub async fn mark_messages_read_handler(
    _session_type: i8,
    _target_id: i64,
    _owner_uid: i64,
) -> Result<(), String> {
    // TODO: 实现标记已读
    Ok(())
}
