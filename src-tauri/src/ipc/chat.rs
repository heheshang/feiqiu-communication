// src-tauri/src/ipc/chat.rs
//
/// 聊天相关 IPC 接口

use tauri::State;
use sea_orm::DbConn;
use crate::database::handler::{ChatMessageHandler, ChatSessionHandler};
use crate::types::{SessionType, ChatMessage, ChatSession};

/// 获取聊天历史记录
#[tauri::command]
pub async fn get_chat_history_handler(
    _session_type: i8,
    _target_id: i64,
    _page: i32,
    _page_size: i32,
    _state: State<'_, DbConn>,
) -> Result<Vec<ChatMessage>, String> {
    // TODO: Implement proper pagination
    // For now, return empty vec as we need to join with user table to get sender info
    // This will be implemented after we have proper data models
    Ok(vec![])
}

/// 发送文本消息
#[tauri::command]
pub async fn send_text_message_handler(
    session_type: i8,
    target_id: i64,
    content: String,
    owner_uid: i64,
    state: State<'_, DbConn>,
) -> Result<i64, String> {
    let db = state.inner();

    // Create the message in database
    let message = ChatMessageHandler::create(
        db,
        session_type,
        target_id,
        owner_uid,
        content,
        0, // Text message type
    )
    .await
    .map_err(|e| e.to_string())?;

    // Get or create session
    let session = ChatSessionHandler::get_or_create(
        db,
        owner_uid,
        session_type,
        target_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    // Update session last message
    ChatSessionHandler::update_last_message(db, session.sid, message.mid)
        .await
        .map_err(|e| e.to_string())?;

    // TODO: Send via UDP network

    Ok(message.mid)
}

/// 获取会话列表
#[tauri::command]
pub async fn get_session_list_handler(
    owner_uid: i64,
    state: State<'_, DbConn>,
) -> Result<Vec<ChatSession>, String> {
    let db = state.inner();

    let sessions = ChatSessionHandler::list_by_owner(db, owner_uid)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to frontend type
    let result: Vec<ChatSession> = sessions
        .into_iter()
        .map(|s| {
            ChatSession {
                sid: s.sid,
                owner_uid: s.owner_uid,
                session_type: if s.session_type == 0 {
                    SessionType::Single
                } else {
                    SessionType::Group
                },
                target_id: s.target_id,
                last_msg_id: s.last_msg_id,
                unread_count: s.unread_count,
                update_time: s.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            }
        })
        .collect();

    Ok(result)
}

/// 标记消息已读
#[tauri::command]
pub async fn mark_messages_read_handler(
    session_type: i8,
    target_id: i64,
    owner_uid: i64,
    state: State<'_, DbConn>,
) -> Result<(), String> {
    let db = state.inner();

    // Find the session
    let session = ChatSessionHandler::find_by_owner_and_target(
        db,
        owner_uid,
        session_type,
        target_id,
    )
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| format!("会话不存在"))?;

    // Clear unread count
    ChatSessionHandler::clear_unread(db, session.sid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
