// src-tauri/src/ipc/chat.rs
//
use crate::core::ChatService;
use crate::database::handler::{ChatMessageHandler, UserHandler};
use crate::types::{ChatMessage, ChatSession, MapErrToFrontend};
use sea_orm::DbConn;
use tauri::State;

/// 聊天相关 IPC 接口（薄层 - 只做参数转换和错误映射）
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
    ChatService::get_messages(db, session_type, target_id, page, page_size)
        .await
        .map_err_to_frontend()
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
    ChatService::send_message(db, session_type, target_id, owner_uid, content, 0)
        .await
        .map_err_to_frontend()
}

/// 获取会话列表
#[tauri::command]
pub async fn get_session_list_handler(owner_uid: i64, state: State<'_, DbConn>) -> Result<Vec<ChatSession>, String> {
    let db = state.inner();
    ChatService::get_sessions(db, owner_uid).await.map_err_to_frontend()
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
    ChatService::mark_as_read(db, session_type, target_id, owner_uid)
        .await
        .map_err_to_frontend()
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
    ChatMessageHandler::update_status(db, mid, 2).await.map_err_to_frontend()?;

    // Send read receipt via UDP
    use crate::network::feiq::model::FeiQPacket;
    use crate::network::udp::sender;

    let read_packet = FeiQPacket::make_feiq_read_packet(&msg_no);

    // Send to the original sender
    let addr = format!("{}:{}", target_ip, 2425);
    sender::send_packet(&addr, &read_packet).await.map_err_to_frontend()?;

    Ok(())
}

/// 处理接收到的已读回执（由网络层调用）
#[allow(dead_code)]
pub async fn handle_read_receipt(db: &DbConn, msg_no: &str) -> Result<(), String> {
    // 通过 msg_no 查找消息并更新状态
    if let Ok(Some(message)) = ChatMessageHandler::find_by_msg_no(db, msg_no).await {
        // 更新消息状态为已读
        let _ = ChatMessageHandler::update_status(db, message.mid, 2).await;
        Ok(())
    } else {
        // 如果找不到消息，可能是旧数据（msg_no 为空）
        // 尝试将 msg_no 解析为消息 ID（向后兼容）
        if let Ok(mid) = msg_no.parse::<i64>() {
            let _ = ChatMessageHandler::update_status(db, mid, 2).await;
        }
        Ok(())
    }
}

/// 重试发送失败的消息
#[tauri::command]
pub async fn retry_send_message(
    mid: i64,
    _session_type: i8,
    target_id: i64,
    _owner_uid: i64,
    state: State<'_, DbConn>,
) -> Result<(), String> {
    let db = state.inner();

    // 获取消息详情
    let message = ChatMessageHandler::find_by_id(db, mid).await.map_err_to_frontend()?;

    // 重置状态为发送中
    ChatMessageHandler::update_status(db, mid, 0).await.map_err_to_frontend()?;

    // 重新发送消息
    use crate::network::feiq::model::FeiQPacket;
    use crate::network::udp::sender;

    // Get target user's IP from database
    let target_user = UserHandler::find_by_id(db, target_id).await.map_err_to_frontend()?;

    let packet = FeiQPacket::make_feiq_message_packet(&message.content, None);
    let addr = format!("{}:{}", target_user.feiq_ip, target_user.feiq_port);

    // 发送消息
    sender::send_packet(&addr, &packet).await.map_err_to_frontend()?;

    // 发送成功后更新状态
    ChatMessageHandler::update_status(db, mid, 1).await.map_err_to_frontend()?;

    Ok(())
}
