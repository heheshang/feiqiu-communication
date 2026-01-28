// src-tauri/src/core/chat/manager.rs
//
/// 聊天会话管理器
///
/// 负责管理会话状态和未读计数：
/// - 获取会话列表
/// - 更新未读计数
/// - 归档会话
/// - 删除会话
/// - 会话排序（按最后消息时间）
use crate::database::handler::{ChatMessageHandler, ChatSessionHandler};
use crate::error::{AppError, AppResult};
use sea_orm::DbConn;
use tracing::{info, warn};

/// 聊天会话管理器
pub struct ChatManager;

impl ChatManager {
    /// 获取用户的会话列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 用户 ID
    ///
    /// # 返回
    /// - 会话列表，按最后消息时间降序排列
    pub async fn get_session_list(
        db: &DbConn,
        owner_uid: i64,
    ) -> AppResult<Vec<crate::database::model::chat_session::Model>> {
        info!("获取用户会话列表: owner_uid={}", owner_uid);

        let sessions = ChatSessionHandler::list_by_owner(db, owner_uid).await?;

        info!("找到 {} 个会话", sessions.len());
        Ok(sessions)
    }

    /// 更新会话未读计数
    ///
    /// 当收到新消息时调用
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 会话所有者 ID
    /// - `session_type`: 会话类型
    /// - `target_id`: 目标 ID
    ///
    /// # 返回
    /// - 更新后的未读计数
    pub async fn increment_unread_count(
        db: &DbConn,
        owner_uid: i64,
        session_type: i8,
        target_id: i64,
    ) -> AppResult<i32> {
        info!(
            "增加未读计数: owner_uid={}, session_type={}, target_id={}",
            owner_uid, session_type, target_id
        );

        // 获取或创建会话
        let session = ChatSessionHandler::get_or_create(db, owner_uid, session_type, target_id).await?;

        // 增加未读计数
        ChatSessionHandler::increment_unread(db, session.sid).await?;

        // 重新获取会话以获取更新后的计数
        let session = ChatSessionHandler::find_by_id(db, session.sid).await?;

        info!("未读计数已更新: sid={}, count={}", session.sid, session.unread_count);

        // 触发 UI 事件
        let _ = crate::event::bus::EVENT_SENDER.send(crate::event::model::AppEvent::Ui(
            crate::event::model::UiEvent::UpdateUnreadCount {
                session_type,
                target_id,
                count: session.unread_count,
            },
        ));

        Ok(session.unread_count)
    }

    /// 清空会话未读计数
    ///
    /// 用户打开会话时调用
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 会话所有者 ID
    /// - `session_type`: 会话类型
    /// - `target_id`: 目标 ID
    pub async fn clear_unread_count(db: &DbConn, owner_uid: i64, session_type: i8, target_id: i64) -> AppResult<()> {
        info!(
            "清空未读计数: owner_uid={}, session_type={}, target_id={}",
            owner_uid, session_type, target_id
        );

        // 查找会话
        let session = ChatSessionHandler::find_by_owner_and_target(db, owner_uid, session_type, target_id)
            .await?
            .ok_or_else(|| AppError::NotFound("会话不存在".to_string()))?;

        // 清空未读计数
        ChatSessionHandler::clear_unread(db, session.sid).await?;

        info!("未读计数已清空: sid={}", session.sid);

        // 触发 UI 事件
        let _ = crate::event::bus::EVENT_SENDER.send(crate::event::model::AppEvent::Ui(
            crate::event::model::UiEvent::UpdateUnreadCount {
                session_type,
                target_id,
                count: 0,
            },
        ));

        Ok(())
    }

    /// 删除会话
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `sid`: 会话 ID
    ///
    /// # 注意
    /// - 仅删除会话记录，不删除消息
    /// - 消息保留在数据库中用于历史记录查询
    pub async fn delete_session(db: &DbConn, sid: i64) -> AppResult<()> {
        info!("删除会话: sid={}", sid);

        ChatSessionHandler::delete(db, sid).await?;

        info!("会话已删除: sid={}", sid);

        // 触发 UI 事件
        let _ = crate::event::bus::EVENT_SENDER.send(crate::event::model::AppEvent::Chat(
            crate::event::model::ChatEvent::SessionDeleted { session_id: sid },
        ));

        Ok(())
    }

    /// 获取会话的最后一条消息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `session_type`: 会话类型
    /// - `target_id`: 目标 ID
    ///
    /// # 返回
    /// - 最后一条消息，如果不存在则返回 None
    pub async fn get_last_message(
        db: &DbConn,
        session_type: i8,
        target_id: i64,
    ) -> AppResult<Option<crate::database::model::chat_message::Model>> {
        let messages = ChatMessageHandler::find_by_session(db, session_type, target_id, 1).await?;

        Ok(messages.into_iter().next())
    }

    /// 获取会话统计信息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 用户 ID
    ///
    /// # 返回
    /// - 会话统计信息
    pub async fn get_session_stats(db: &DbConn, owner_uid: i64) -> AppResult<SessionStats> {
        info!("获取会话统计: owner_uid={}", owner_uid);

        let sessions = ChatSessionHandler::list_by_owner(db, owner_uid).await?;

        let total_sessions = sessions.len();
        let total_unread = sessions.iter().map(|s| s.unread_count as usize).sum();

        // 统计单聊和群聊数量
        let single_chats = sessions.iter().filter(|s| s.session_type == 0).count();
        let group_chats = sessions.iter().filter(|s| s.session_type == 1).count();

        let stats = SessionStats {
            total_sessions,
            single_chats,
            group_chats,
            total_unread,
        };

        info!("会话统计: {:?}", stats);
        Ok(stats)
    }

    /// 归档旧会话
    ///
    /// 将超过指定天数没有消息的会话标记为归档状态
    ///
    /// # 参数
    /// - `_db`: 数据库连接
    /// - `_owner_uid`: 用户 ID
    /// - `_days`: 天数阈值
    ///
    /// # 返回
    /// - 归档的会话数量
    pub async fn archive_old_sessions(_db: &DbConn, _owner_uid: i64, _days: i64) -> AppResult<usize> {
        info!("归档旧会话");

        // TODO: 实现归档逻辑
        // 这需要：
        // 1. 查询超过指定天数没有活动的会话
        // 2. 更新会话的归档状态（需要在会话表中添加 archived 字段）

        warn!("归档功能尚未实现");
        Ok(0)
    }

    /// 搜索会话
    ///
    /// 根据关键词搜索会话（按目标用户的昵称或群组名称）
    ///
    /// # 参数
    /// - `_db`: 数据库连接
    /// - `_owner_uid`: 用户 ID
    /// - `_keyword`: 搜索关键词
    ///
    /// # 返回
    /// - 匹配的会话列表
    pub async fn search_sessions(
        _db: &DbConn,
        _owner_uid: i64,
        _keyword: &str,
    ) -> AppResult<Vec<crate::database::model::chat_session::Model>> {
        info!("搜索会话");

        // TODO: 实现搜索逻辑
        // 这需要：
        // 1. 查询用户的所有会话
        // 2. 根据 target_id 关联查询用户或群组名称
        // 3. 过滤匹配的会话

        warn!("搜索功能尚未实现");
        Ok(vec![])
    }
}

/// 会话统计信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionStats {
    /// 总会话数
    pub total_sessions: usize,

    /// 单聊数量
    pub single_chats: usize,

    /// 群聊数量
    pub group_chats: usize,

    /// 总未读消息数
    pub total_unread: usize,
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_manager_module_exists() {
        // 这是一个简单的存在性测试
        assert_eq!(std::mem::size_of::<ChatManager>(), 0);
    }

    #[test]
    fn test_session_stats_struct() {
        let stats = SessionStats {
            total_sessions: 10,
            single_chats: 7,
            group_chats: 3,
            total_unread: 15,
        };

        assert_eq!(stats.total_sessions, 10);
        assert_eq!(stats.single_chats, 7);
        assert_eq!(stats.group_chats, 3);
        assert_eq!(stats.total_unread, 15);
    }
}
