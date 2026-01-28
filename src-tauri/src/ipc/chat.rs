// src-tauri/src/ipc/chat.rs
//
use crate::database::handler::{ChatMessageHandler, ChatSessionHandler};
use crate::types::{ChatMessage, ChatSession, MessageStatus, MessageType, SessionType};
use sea_orm::DbConn;
/// 聊天相关 IPC 接口
use tauri::State;

/// 获取聊天历史记录（分页）
#[tauri::command]
pub async fn get_chat_history_handler(
    session_type: i8,
    target_id: i64,
    page: i32,
    page_size: i32,
    state: State<'_, DbConn>,
) -> Result<Vec<ChatMessage>, String> {
    let db = state.inner();

    let messages = ChatMessageHandler::find_by_session_paged(db, session_type, target_id, page, page_size)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to frontend type
    let result: Vec<ChatMessage> = messages
        .into_iter()
        .map(|m| ChatMessage {
            mid: m.mid,
            session_type: if m.session_type == 0 {
                SessionType::Single
            } else {
                SessionType::Group
            },
            target_id: m.target_id,
            sender_uid: m.sender_uid,
            msg_type: match m.msg_type {
                0 => MessageType::Text,
                1 => MessageType::File,
                2 => MessageType::Emoji,
                _ => MessageType::Text,
            },
            content: m.content,
            send_time: m.send_time,
            status: match m.status {
                -1 => MessageStatus::Failed,
                0 => MessageStatus::Sending,
                1 => MessageStatus::Sent,
                2 => MessageStatus::Read,
                _ => MessageStatus::Sending,
            },
        })
        .collect();

    Ok(result)
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
    let session = ChatSessionHandler::get_or_create(db, owner_uid, session_type, target_id)
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
pub async fn get_session_list_handler(owner_uid: i64, state: State<'_, DbConn>) -> Result<Vec<ChatSession>, String> {
    let db = state.inner();

    let sessions = ChatSessionHandler::list_by_owner(db, owner_uid)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to frontend type
    let result: Vec<ChatSession> = sessions
        .into_iter()
        .map(|s| ChatSession {
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
    let session = ChatSessionHandler::find_by_owner_and_target(db, owner_uid, session_type, target_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("会话不存在"))?;

    // Clear unread count
    ChatSessionHandler::clear_unread(db, session.sid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 标记单条消息已读并发送已读回执
#[tauri::command]
pub async fn mark_message_read_and_send_receipt(
    mid: i64,
    msg_no: String,
    target_ip: String,
    state: State<'_, DbConn>,
) -> Result<(), String> {
    let db = state.inner();

    // Update message status to read
    ChatMessageHandler::update_status(db, mid, 2).await.map_err(|e| e.to_string())?;

    // Send read receipt via UDP
    use crate::network::feiq::model::FeiqPacket;
    use crate::network::udp::sender;

    let read_packet = FeiqPacket::make_read_packet(&msg_no);

    // Send to the original sender
    let addr = format!("{}:{}", target_ip, 2425);
    sender::send_packet(&addr, &read_packet)
        .await
        .map_err(|e| format!("发送已读回执失败: {}", e))?;

    Ok(())
}

/// 处理接收到的已读回执（由网络层调用）
#[allow(dead_code)]
pub async fn handle_read_receipt(db: &DbConn, msg_no: &str) -> Result<(), String> {
    // TODO: 需要在 chat_message 表中添加 msg_no 字段
    // 目前先使用消息ID作为临时方案
    // 当收到已读回执时，通过 msg_no 查找消息

    // 临时实现：尝试解析 msg_no 为数字
    if let Ok(mid) = msg_no.parse::<i64>() {
        let _ = ChatMessageHandler::update_status(db, mid, 2).await;
    }

    Ok(())
}

/// 重试发送失败的消息
#[tauri::command]
pub async fn retry_send_message(
    mid: i64,
    _session_type: i8,
    _target_id: i64,
    _owner_uid: i64,
    state: State<'_, DbConn>,
) -> Result<(), String> {
    let db = state.inner();

    // 获取消息详情
    let message = ChatMessageHandler::find_by_id(db, mid).await.map_err(|e| e.to_string())?;

    // 重置状态为发送中
    ChatMessageHandler::update_status(db, mid, 0).await.map_err(|e| e.to_string())?;

    // 重新发送消息
    use crate::network::feiq::model::FeiqPacket;
    use crate::network::udp::sender;

    let packet = FeiqPacket::make_message_packet(&message.content, true);
    let addr = format!("0.0.0.0:2425"); // TODO: 获取实际的目标地址

    // 发送消息
    sender::send_packet(&addr, &packet)
        .await
        .map_err(|e| format!("重试发送失败: {}", e))?;

    // 发送成功后更新状态
    ChatMessageHandler::update_status(db, mid, 1).await.map_err(|e| e.to_string())?;

    Ok(())
}
