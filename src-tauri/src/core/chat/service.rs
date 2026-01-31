//! 聊天业务逻辑服务层
//!
//! ChatService 提供聊天相关的业务逻辑操作，包括：
//! - 发送消息
//! - 获取消息列表
//! - 标记消息已读
//! - 删除消息
//! - 管理聊天会话

use crate::database::handler::{ChatMessageHandler, ChatSessionHandler, UserHandler};
use crate::error::{AppError, AppResult};
use crate::network::feiq::model::FeiQPacket;
use crate::network::udp::sender;
use crate::types::{ChatMessage, ChatSession, MessageStatus, MessageType, SessionType};
use sea_orm::DbConn;
use tracing::{error, info};

/// 聊天服务
pub struct ChatService;

impl ChatService {
    /// 发送消息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `session_type`: 会话类型（0=单聊, 1=群聊）
    /// - `target_id`: 目标ID（用户ID或群组ID）
    /// - `sender_uid`: 发送者用户ID
    /// - `content`: 消息内容
    /// - `msg_type`: 消息类型（0=文本, 1=文件, 2=Emoji）
    ///
    /// # 返回
    /// 返回新创建的消息ID
    pub async fn send_message(
        db: &DbConn,
        session_type: i8,
        target_id: i64,
        sender_uid: i64,
        content: String,
        msg_type: i8,
    ) -> AppResult<i64> {
        // 1. 创建消息记录（状态：发送中 = 0）
        let message = ChatMessageHandler::create(db, session_type, target_id, sender_uid, content.clone(), msg_type)
            .await
            .map_err(|e| {
                error!("创建消息记录失败: {}", e);
                e
            })?;

        let mid = message.mid;
        info!("消息记录已创建: mid={}", mid);

        // 2. 获取或创建会话
        let session = ChatSessionHandler::get_or_create(db, sender_uid, session_type, target_id)
            .await
            .map_err(|e| {
                error!("获取或创建会话失败: {}", e);
                e
            })?;

        // 3. 更新会话的最后消息
        ChatSessionHandler::update_last_message(db, session.sid, mid)
            .await
            .map_err(|e| {
                error!("更新会话最后消息失败: {}", e);
                e
            })?;

        // 4. 根据会话类型发送消息
        if session_type == 0 {
            // 单聊
            let target_user = UserHandler::find_by_id(db, target_id).await.map_err(|e| {
                error!("查找目标用户失败: {}", e);
                AppError::NotFound(format!("目标用户 {} 不存在", target_id))
            })?;

            // 检查用户是否在线
            if target_user.status != 1 {
                error!("目标用户不在线，标记消息为失败");
                ChatMessageHandler::update_status(db, mid, -1).await?;
                return Err(AppError::Business("目标用户不在线".to_string()));
            }

            // 构造消息包
            let packet = FeiQPacket::make_feiq_message_packet(&content, None);
            let addr = format!("{}:{}", target_user.feiq_ip, target_user.feiq_port);

            // 发送 UDP 消息
            match sender::send_packet(&addr, &packet).await {
                Ok(_) => {
                    info!("消息已发送: mid={}, addr={}", mid, addr);
                    // 更新消息状态为已发送（1）
                    ChatMessageHandler::update_status(db, mid, 1).await?;
                    Ok(mid)
                }
                Err(e) => {
                    error!("发送消息失败: {}", e);
                    // 更新消息状态为失败（-1）
                    let _ = ChatMessageHandler::update_status(db, mid, -1).await;
                    Err(AppError::Network(format!("发送消息失败: {}", e)))
                }
            }
        } else {
            // 群聊
            let packet = FeiQPacket::make_feiq_message_packet(&content, None);

            use crate::core::group::GroupBroadcaster;
            let sent_count = GroupBroadcaster::broadcast_message(db, target_id, &packet, sender_uid).await?;

            info!("群消息已广播到 {} 个成员", sent_count);

            // 更新消息状态为已发送（1）
            ChatMessageHandler::update_status(db, mid, 1).await?;

            Ok(mid)
        }
    }

    /// 获取会话的消息列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `session_type`: 会话类型（0=单聊, 1=群聊）
    /// - `target_id`: 目标ID（用户ID或群组ID）
    /// - `page`: 页码（从1开始）
    /// - `page_size`: 每页消息数量
    ///
    /// # 返回
    /// 返回消息列表（前端类型）
    pub async fn get_messages(
        db: &DbConn,
        session_type: i8,
        target_id: i64,
        page: i32,
        page_size: i32,
    ) -> AppResult<Vec<ChatMessage>> {
        let messages = ChatMessageHandler::find_by_session_paged(db, session_type, target_id, page, page_size).await?;

        // 转换为前端类型
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

    /// 标记消息已读
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `session_type`: 会话类型
    /// - `target_id`: 目标ID
    /// - `owner_uid`: 用户ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn mark_as_read(db: &DbConn, session_type: i8, target_id: i64, owner_uid: i64) -> AppResult<()> {
        // 查找会话
        let session = ChatSessionHandler::find_by_owner_and_target(db, owner_uid, session_type, target_id)
            .await?
            .ok_or_else(|| AppError::NotFound("会话不存在".to_string()))?;

        // 清空未读计数
        ChatSessionHandler::clear_unread(db, session.sid).await?;

        Ok(())
    }

    /// 删除消息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `msg_id`: 消息ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn delete_message(db: &DbConn, msg_id: i64) -> AppResult<()> {
        ChatMessageHandler::delete(db, msg_id).await
    }

    /// 获取用户的聊天会话列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 用户ID
    ///
    /// # 返回
    /// 返回会话列表（前端类型）
    pub async fn get_sessions(db: &DbConn, owner_uid: i64) -> AppResult<Vec<ChatSession>> {
        let sessions = ChatSessionHandler::list_by_owner(db, owner_uid).await?;

        // 转换为前端类型
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

    /// 删除聊天会话
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `session_id`: 会话ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn delete_session(db: &DbConn, session_id: i64) -> AppResult<()> {
        ChatSessionHandler::delete(db, session_id).await
    }

    /// 清空会话未读计数
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `session_id`: 会话ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn clear_unread(db: &DbConn, session_id: i64) -> AppResult<()> {
        ChatSessionHandler::clear_unread(db, session_id).await
    }
}
