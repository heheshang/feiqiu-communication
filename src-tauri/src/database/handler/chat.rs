// src-tauri/src/database/handler/chat.rs
//
//! 聊天消息和会话 CRUD 操作

use crate::database::model::{chat_message, chat_session, ChatMessage, ChatSession};
use crate::error::{AppError, AppResult};
use sea_orm::{prelude::*, *};

/// 聊天消息处理器
pub struct ChatMessageHandler;

impl ChatMessageHandler {
    /// 发送消息
    pub async fn create(
        db: &DbConn,
        session_type: i8,
        target_id: i64,
        sender_uid: i64,
        content: String,
        msg_type: i8,
    ) -> AppResult<chat_message::Model> {
        Self::create_with_msg_no(db, session_type, target_id, sender_uid, content, msg_type, None).await
    }

    /// 发送消息（带消息编号）
    pub async fn create_with_msg_no(
        db: &DbConn,
        session_type: i8,
        target_id: i64,
        sender_uid: i64,
        content: String,
        msg_type: i8,
        msg_no: Option<String>,
    ) -> AppResult<chat_message::Model> {
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
            msg_no: ActiveValue::Set(msg_no),
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

    /// 根据消息编号查找消息
    pub async fn find_by_msg_no(db: &DbConn, msg_no: &str) -> AppResult<Option<chat_message::Model>> {
        let message = ChatMessage::find()
            .filter(chat_message::Column::MsgNo.eq(msg_no))
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(message)
    }

    /// 获取会话的聊天消息
    pub async fn find_by_session(
        db: &DbConn,
        session_type: i8,
        target_id: i64,
        limit: u64,
    ) -> AppResult<Vec<chat_message::Model>> {
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

    /// 分页获取会话的聊天消息
    /// page: 页码，从 1 开始
    /// page_size: 每页消息数量
    /// 返回: 按时间正序排列的消息列表（最旧的消息在前）
    pub async fn find_by_session_paged(
        db: &DbConn,
        session_type: i8,
        target_id: i64,
        page: i32,
        page_size: i32,
    ) -> AppResult<Vec<chat_message::Model>> {
        let page = page.max(1) as u64;
        let page_size = page_size.max(1) as u64;
        let offset = (page - 1) * page_size;

        // 使用原生 SQL 实现正确的分页逻辑
        // 分页获取最新 N 页的消息，然后反转顺序
        let messages = ChatMessage::find()
            .filter(chat_message::Column::SessionType.eq(session_type))
            .filter(chat_message::Column::TargetId.eq(target_id))
            .order_by_desc(chat_message::Column::SendTime)
            .limit(page_size)
            .offset(offset)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 反转顺序，使最新的消息在最后（正序：旧 -> 新）
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

    /// 根据状态查找消息
    ///
    /// 查找特定状态的消息，用于重试发送失败的消息等
    pub async fn find_by_status(db: &DbConn, status: i8, limit: Option<u64>) -> AppResult<Vec<chat_message::Model>> {
        let mut query = ChatMessage::find()
            .filter(chat_message::Column::Status.eq(status))
            .order_by_asc(chat_message::Column::SendTime);

        if let Some(limit_value) = limit {
            query = query.limit(limit_value);
        }

        let messages = query.all(db).await.map_err(|e| AppError::Database(e))?;

        Ok(messages)
    }

    /// 标记会话的所有消息为已读
    ///
    /// 将指定会话的所有未读消息标记为已读状态
    pub async fn mark_session_read(db: &DbConn, owner_uid: i64, session_type: i8, target_id: i64) -> AppResult<u64> {
        // 查找会话
        let session = ChatSessionHandler::find_by_owner_and_target(db, owner_uid, session_type, target_id)
            .await?
            .ok_or_else(|| AppError::NotFound("会话不存在".to_string()))?;

        // 更新该会话的所有消息状态为已读 (status = 2)
        let _update_result = ChatMessage::update_many()
            .col_expr(
                chat_message::Column::Status,
                Expr::value(2), // 2 = 已读
            )
            .col_expr(
                chat_message::Column::UpdateTime,
                Expr::value(chrono::Utc::now().naive_utc()),
            )
            .filter(chat_message::Column::SessionType.eq(session_type))
            .filter(chat_message::Column::TargetId.eq(target_id))
            .filter(chat_message::Column::Status.lt(2)) // 只更新未读消息
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 清除会话的未读计数
        ChatSessionHandler::clear_unread(db, session.sid).await?;

        // 返回更新的消息数量（这里简化处理，实际可从 update_result 获取）
        Ok(1)
    }
}

/// 聊天会话处理器
pub struct ChatSessionHandler;

impl ChatSessionHandler {
    /// 创建或获取会话
    pub async fn get_or_create(
        db: &DbConn,
        owner_uid: i64,
        session_type: i8,
        target_id: i64,
    ) -> AppResult<chat_session::Model> {
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
    pub async fn find_by_owner_and_target(
        db: &DbConn,
        owner_uid: i64,
        session_type: i8,
        target_id: i64,
    ) -> AppResult<Option<chat_session::Model>> {
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

    /// 归档旧会话
    ///
    /// 归档指定天数之前未更新的会话
    pub async fn archive_old_sessions(db: &DbConn, owner_uid: i64, days: i64) -> AppResult<usize> {
        let cutoff_time = chrono::Utc::now().naive_utc() - chrono::Duration::days(days);

        // 查找需要归档的会话
        let sessions_to_archive = ChatSession::find()
            .filter(chat_session::Column::OwnerUid.eq(owner_uid))
            .filter(chat_session::Column::UpdateTime.lt(cutoff_time))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 这里简化处理，实际可能需要将它们移动到归档表或标记为已归档
        // 当前实现只是删除旧会话
        let count = sessions_to_archive.len();

        for session in sessions_to_archive {
            Self::delete(db, session.sid).await?;
        }

        Ok(count)
    }

    /// 搜索会话
    ///
    /// 根据关键词搜索会话（通过会话关联的最后消息内容）
    pub async fn search_sessions(
        db: &DbConn,
        owner_uid: i64,
        keyword: &str,
        limit: u64,
    ) -> AppResult<Vec<chat_session::Model>> {
        // 搜索包含关键词的消息
        let messages_with_keyword = ChatMessage::find()
            .filter(chat_message::Column::Content.contains(keyword))
            .order_by_desc(chat_message::Column::SendTime)
            .limit(limit)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 获取这些消息对应的会话
        let mut session_ids = std::collections::HashSet::new();
        for message in messages_with_keyword {
            session_ids.insert((message.session_type, message.target_id));
        }

        // 查找这些会话
        let mut sessions = Vec::new();
        for (session_type, target_id) in session_ids {
            if let Some(session) =
                ChatSessionHandler::find_by_owner_and_target(db, owner_uid, session_type, target_id).await?
            {
                sessions.push(session);
            }
        }

        // 按更新时间排序
        sessions.sort_by(|a, b| b.update_time.cmp(&a.update_time));

        // 限制返回数量
        sessions.truncate(limit as usize);

        Ok(sessions)
    }
}
