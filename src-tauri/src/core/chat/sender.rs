// src-tauri/src/core/chat/sender.rs
//
/// 消息发送处理器
///
/// 负责处理消息发送逻辑：
/// - 创建消息记录（状态：发送中）
/// - 通过 UDP 发送消息包
/// - 更新消息发送状态
/// - 处理发送失败重试
/// - 与群组广播集成
use crate::database::handler::{ChatMessageHandler, ChatSessionHandler, UserHandler};
use crate::error::{AppError, AppResult};
use crate::network::feiq::model::ProtocolPacket;
use crate::network::udp::sender;
use sea_orm::DbConn;
use tracing::{error, info, warn};

/// 消息发送器
pub struct MessageSender;

impl MessageSender {
    /// 发送单聊消息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `sender_uid`: 发送者用户 ID
    /// - `target_id`: 目标用户 ID
    /// - `content`: 消息内容
    ///
    /// # 返回
    /// - `Ok(mid)`: 消息 ID
    /// - `Err`: 发送失败
    pub async fn send_single_message(db: &DbConn, sender_uid: i64, target_id: i64, content: String) -> AppResult<i64> {
        info!(
            "发送单聊消息: sender={}, target={}, content={}",
            sender_uid, target_id, content
        );

        // 1. 创建消息记录（状态：发送中 = 0）
        let session_type = 0; // 单聊
        let message = ChatMessageHandler::create(db, session_type, target_id, sender_uid, content.clone(), 0)
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

        // 4. 获取目标用户的网络信息
        let target_user = UserHandler::find_by_id(db, target_id).await.map_err(|e| {
            error!("查找目标用户失败: {}", e);
            AppError::NotFound(format!("目标用户 {} 不存在", target_id))
        })?;

        // 检查用户是否在线
        if target_user.status != 1 {
            warn!("目标用户不在线，标记消息为失败");
            ChatMessageHandler::update_status(db, mid, -1).await?;
            return Err(AppError::Business("目标用户不在线".to_string()));
        }

        // 5. 构造消息包
        let packet = ProtocolPacket::make_message_packet(&content, true);
        let addr = format!("{}:{}", target_user.feiq_ip, target_user.feiq_port);

        // 6. 发送 UDP 消息
        match sender::send_packet(&addr, &packet).await {
            Ok(_) => {
                info!("消息已发送: mid={}, addr={}", mid, addr);

                // 7. 更新消息状态为已发送（1）
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
    }

    /// 发送群聊消息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `sender_uid`: 发送者用户 ID
    /// - `group_id`: 群组 ID
    /// - `content`: 消息内容
    ///
    /// # 返回
    /// - `Ok((mid, sent_count))`: 消息 ID 和成功发送数量
    /// - `Err`: 发送失败
    pub async fn send_group_message(
        db: &DbConn,
        sender_uid: i64,
        group_id: i64,
        content: String,
    ) -> AppResult<(i64, usize)> {
        info!(
            "发送群聊消息: sender={}, group={}, content={}",
            sender_uid, group_id, content
        );

        // 1. 创建消息记录（状态：发送中 = 0）
        let session_type = 1; // 群聊
        let message = ChatMessageHandler::create(db, session_type, group_id, sender_uid, content.clone(), 0).await?;

        let mid = message.mid;
        info!("群消息记录已创建: mid={}", mid);

        // 2. 获取或创建会话
        let session = ChatSessionHandler::get_or_create(db, sender_uid, session_type, group_id).await?;

        // 3. 更新会话的最后消息
        ChatSessionHandler::update_last_message(db, session.sid, mid).await?;

        // 4. 构造消息包
        let packet = ProtocolPacket::make_message_packet(&content, true);

        // 5. 使用 GroupBroadcaster 广播消息
        use crate::core::group::GroupBroadcaster;
        let sent_count = GroupBroadcaster::broadcast_message(db, group_id, &packet, sender_uid).await?;

        info!("群消息已广播到 {} 个成员", sent_count);

        // 6. 更新消息状态为已发送（1）
        ChatMessageHandler::update_status(db, mid, 1).await?;

        Ok((mid, sent_count))
    }

    /// 重试发送失败的消息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `mid`: 消息 ID
    ///
    /// # 返回
    /// - `Ok(())`: 重试成功
    /// - `Err`: 重试失败
    pub async fn retry_send_message(db: &DbConn, mid: i64) -> AppResult<()> {
        info!("重试发送消息: mid={}", mid);

        // 1. 获取消息详情
        let message = ChatMessageHandler::find_by_id(db, mid).await?;

        // 只重试状态为失败的消息
        if message.status != -1 {
            return Err(AppError::Business("消息状态不是失败，无需重试".to_string()));
        }

        // 2. 重置状态为发送中
        ChatMessageHandler::update_status(db, mid, 0).await?;

        // 3. 根据会话类型发送
        if message.session_type == 0 {
            // 单聊
            let target_user = UserHandler::find_by_id(db, message.target_id).await?;
            let packet = ProtocolPacket::make_message_packet(&message.content, true);
            let addr = format!("{}:{}", target_user.feiq_ip, target_user.feiq_port);

            sender::send_packet(&addr, &packet)
                .await
                .map_err(|e| AppError::Network(format!("发送消息失败: {}", e)))?;

            // 更新状态为已发送
            ChatMessageHandler::update_status(db, mid, 1).await?;
        } else {
            // 群聊
            let packet = ProtocolPacket::make_message_packet(&message.content, true);
            use crate::core::group::GroupBroadcaster;
            GroupBroadcaster::broadcast_message(db, message.target_id, &packet, message.sender_uid).await?;

            // 更新状态为已发送
            ChatMessageHandler::update_status(db, mid, 1).await?;
        }

        info!("消息重试成功: mid={}", mid);
        Ok(())
    }

    /// 批量重试发送失败的消息
    ///
    /// # 参数
    /// - `_db`: 数据库连接
    /// - `_session_type`: 会话类型
    /// - `_target_id`: 目标 ID
    ///
    /// # 返回
    /// - `Ok(success_count)`: 成功重试的数量
    /// - `Err`: 重试失败
    pub async fn retry_failed_messages(_db: &DbConn, _session_type: i8, _target_id: i64) -> AppResult<usize> {
        info!("批量重试失败消息");

        // 查询失败状态的消息
        // 这里需要添加数据库查询方法，暂时返回成功数量 0
        // TODO: 实现 ChatMessageHandler::find_by_status 方法

        Ok(0)
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // 注意：这些测试需要数据库连接，实际运行时需要集成测试环境

    #[test]
    fn test_message_sender_module_exists() {
        // 这是一个简单的存在性测试
        // 实际的功能测试需要完整的测试环境
        assert_eq!(std::mem::size_of::<MessageSender>(), 0);
    }
}
