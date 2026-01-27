// src-tauri/src/database/handler/chat.rs
//
//! 聊天消息和会话 CRUD 操作

use sea_orm::*;
use crate::database::model::{chat_message, chat_session, ChatMessage, ChatSession};
use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};

/// 聊天消息处理器
pub struct ChatMessageHandler;

impl ChatMessageHandler {
    /// 发送消息
    pub async fn create(db: &DbConn, session_type: i8, target_id: i64, sender_uid: i64, content: String, msg_type: i8) -> AppResult<chat_message::Model> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let new_message = chat_message::ActiveModel {
            mid: ActiveValue::NotSet,
            session_type: ActiveValue::Set(session_type),
            target_id: ActiveValue::Set(target_id),
            sender_uid: ActiveValue::Set(sender_uid),
            msg_type: ActiveValue::Set(msg_type),
            content: ActiveValue::Set(content),
            send_time: ActiveValue::Set(now),
            status: ActiveValue::Set(0), // 0 = 发送中
            create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let result = ChatMessage::insert(new_message)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找消息
    pub async fn find_by_id(db: &DbConn, mid: i64) -> AppResult<chat_message::Model> {
        let message = ChatMessage::find_by_id(mid)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?
            .ok_or_else(|| AppError::NotFound(format!("消息 {} 不存在", mid)))?;

        Ok(message)
    }

    /// 获取会话的聊天消息
    pub async fn find_by_session(db: &DbConn, session_type: i8, target_id: i64, limit: u64) -> AppResult<Vec<chat_message::Model>> {
        let messages = ChatMessage::find()
            .filter(chat_message::Column::SessionType.eq(session_type))
            .filter(chat_message::Column::TargetId.eq(target_id))
            .order_by_desc(chat_message::Column::SendTime)
            .limit(limit)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 反转顺序，使最新的消息在最后
        Ok(messages.into_iter().rev().collect())
    }

    /// 更新消息状态
    pub async fn update_status(db: &DbConn, mid: i64, status: i8) -> AppResult<()> {
        let message = Self::find_by_id(db, mid).await?;

        let mut message_update: chat_message::ActiveModel = message.into();
        message_update.status = ActiveValue::Set(status);
        message_update.update_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        message_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 删除消息
    pub async fn delete(db: &DbConn, mid: i64) -> AppResult<()> {
        ChatMessage::delete_by_id(mid)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;
        Ok(())
    }
}

/// 聊天会话处理器
pub struct ChatSessionHandler;

impl ChatSessionHandler {
    /// 创建或获取会话
    pub async fn get_or_create(db: &DbConn, owner_uid: i64, session_type: i8, target_id: i64) -> AppResult<chat_session::Model> {
        // 尝试查找现有会话
        if let Some(session) = Self::find_by_owner_and_target(db, owner_uid, session_type, target_id).await? {
            return Ok(session);
        }

        // 创建新会话
        let new_session = chat_session::ActiveModel {
            sid: ActiveValue::NotSet,
            owner_uid: ActiveValue::Set(owner_uid),
            session_type: ActiveValue::Set(session_type),
            target_id: ActiveValue::Set(target_id),
            last_msg_id: ActiveValue::Set(None),
            unread_count: ActiveValue::Set(0),
            update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let result = ChatSession::insert(new_session)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找会话
    pub async fn find_by_id(db: &DbConn, sid: i64) -> AppResult<chat_session::Model> {
        let session = ChatSession::find_by_id(sid)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?
            .ok_or_else(|| AppError::NotFound(format!("会话 {} 不存在", sid)))?;

        Ok(session)
    }

    /// 根据所有者、类型和目标 ID 查找会话
    pub async fn find_by_owner_and_target(db: &DbConn, owner_uid: i64, session_type: i8, target_id: i64) -> AppResult<Option<chat_session::Model>> {
        let session = ChatSession::find()
            .filter(chat_session::Column::OwnerUid.eq(owner_uid))
            .filter(chat_session::Column::SessionType.eq(session_type))
            .filter(chat_session::Column::TargetId.eq(target_id))
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(session)
    }

    /// 获取用户的所有会话
    pub async fn list_by_owner(db: &DbConn, owner_uid: i64) -> AppResult<Vec<chat_session::Model>> {
        let sessions = ChatSession::find()
            .filter(chat_session::Column::OwnerUid.eq(owner_uid))
            .order_by_desc(chat_session::Column::UpdateTime)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(sessions)
    }

    /// 更新会话的最后消息
    pub async fn update_last_message(db: &DbConn, sid: i64, message_id: i64) -> AppResult<()> {
        let session = Self::find_by_id(db, sid).await?;

        let mut session_update: chat_session::ActiveModel = session.into();
        session_update.last_msg_id = ActiveValue::Set(Some(message_id));
        session_update.update_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        session_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 增加未读消息数
    pub async fn increment_unread(db: &DbConn, sid: i64) -> AppResult<()> {
        let session = Self::find_by_id(db, sid).await?;
        let unread_count = session.unread_count;

        let mut session_update: chat_session::ActiveModel = session.into();
        session_update.unread_count = ActiveValue::Set(unread_count + 1);
        session_update.update_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        session_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 清空未读消息数
    pub async fn clear_unread(db: &DbConn, sid: i64) -> AppResult<()> {
        let session = Self::find_by_id(db, sid).await?;

        let mut session_update: chat_session::ActiveModel = session.into();
        session_update.unread_count = ActiveValue::Set(0);
        session_update.update_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        session_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 删除会话
    pub async fn delete(db: &DbConn, sid: i64) -> AppResult<()> {
        ChatSession::delete_by_id(sid)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;
        Ok(())
    }
}
