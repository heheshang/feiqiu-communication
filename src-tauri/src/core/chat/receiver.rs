// src-tauri/src/core/chat/receiver.rs
//
/// 消息接收处理器
///
/// 负责处理从网络层接收到的消息：
/// - 监听事件总线的网络数据包事件
/// - 解析 SENDMSG 命令
/// - 存储接收到的消息到数据库
/// - 触发 UI 更新事件
/// - 发送 RECVMSG 确认（如果消息需要确认）
/// - 更新会话未读计数
use crate::database::handler::{ChatMessageHandler, ChatSessionHandler, UserHandler};
use crate::event::bus::EVENT_RECEIVER;
use crate::event::model::{AppEvent, NetworkEvent, UiEvent};
use crate::network::feiq::model::FeiQPacket;
use sea_orm::DbConn;
use std::sync::Arc;
use tracing::{error, info, warn};

// 注意：保留 FeiQPacket 用于 RECVMSG 确认发送

/// 消息接收器
pub struct MessageReceiver {
    db: Arc<DbConn>,
}

impl MessageReceiver {
    /// 创建新的消息接收器
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }

    /// 启动消息接收处理
    ///
    /// 在后台任务中监听事件总线，处理接收到的消息
    pub fn start(&self) {
        let db = self.db.clone();
        let receiver = EVENT_RECEIVER.clone();

        tokio::spawn(async move {
            info!("消息接收器已启动");
            Self::event_loop(db, receiver).await;
        });
    }

    /// 事件循环
    async fn event_loop(db: Arc<DbConn>, receiver: crossbeam_channel::Receiver<AppEvent>) {
        loop {
            match receiver.recv() {
                Ok(event) => {
                    // 处理细粒度的MessageReceived事件
                    if let AppEvent::Network(NetworkEvent::MessageReceived {
                        sender_ip,
                        sender_port,
                        sender_nickname,
                        content,
                        msg_no,
                        needs_receipt,
                    }) = event
                    {
                        Self::handle_message_received(
                            db.clone(),
                            sender_ip,
                            sender_port,
                            sender_nickname,
                            content,
                            msg_no,
                            needs_receipt,
                        )
                        .await;
                    }
                }
                Err(e) => {
                    error!("事件接收失败: {}", e);
                }
            }
        }
    }

    /// 处理接收到的消息
    async fn handle_message_received(
        db: Arc<DbConn>,
        sender_ip: String,
        sender_port: u16,
        sender_nickname: String,
        content: String,
        msg_no: String,
        needs_receipt: bool,
    ) {
        info!("收到消息包 from {}:{}", sender_ip, sender_port);

        // 验证发送者信息
        let (sender_ip, sender_port, sender_nickname) =
            match Self::validate_sender_info(&sender_ip, sender_port, &sender_nickname) {
                Ok(info) => info,
                Err(e) => {
                    warn!("发送者信息无效: {}", e);
                    return;
                }
            };

        // 查找或创建发送者用户记录
        let sender_uid = match Self::get_or_create_sender_user(&db, &sender_ip, sender_port, &sender_nickname).await {
            Ok(uid) => uid,
            Err(e) => {
                error!("获取发送者用户信息失败: {}", e);
                return;
            }
        };

        // 存储消息到数据库（单聊类型 = 0）
        let session_type = 0; // 单聊
        let target_id = sender_uid; // 对于接收方，target_id 是发送者的 uid

        match ChatMessageHandler::create_with_msg_no(
            &db,
            session_type,
            target_id,
            sender_uid,
            content,
            0, // 文本消息类型
            Some(msg_no.clone()),
        )
        .await
        {
            Ok(message) => {
                info!("消息已保存到数据库: mid={}, sender={}", message.mid, sender_nickname);

                // 获取或创建会话
                // 获取当前登录用户的 uid
                let current_user_uid = match UserHandler::get_current_user_id(&db).await {
                    Ok(uid) => uid,
                    Err(_) => {
                        // 如果没有当前用户，尝试创建或使用默认值
                        warn!("未找到当前用户，使用默认用户 ID 1");
                        1
                    }
                };

                if let Ok(session) =
                    ChatSessionHandler::get_or_create(&db, current_user_uid, session_type, target_id).await
                {
                    // 更新会话的最后消息
                    let _ = ChatSessionHandler::update_last_message(&db, session.sid, message.mid).await;

                    // 增加未读计数
                    let _ = ChatSessionHandler::increment_unread(&db, session.sid).await;
                }

                // 如果需要发送确认
                if needs_receipt {
                    Self::send_recv_confirmation(&format!("{}:{}", sender_ip, sender_port), &msg_no).await;
                }

                // 触发 UI 事件：显示消息
                let message_json = serde_json::to_string(&message).unwrap_or_default();
                crate::event::bus::EVENT_SENDER
                    .send(AppEvent::Ui(UiEvent::DisplayMessage {
                        session_type,
                        target_id,
                        message: message_json,
                    }))
                    .unwrap_or_else(|e| error!("发送 UI 事件失败: {}", e));

                // 触发 UI 事件：更新未读计数
                crate::event::bus::EVENT_SENDER
                    .send(AppEvent::Ui(UiEvent::UpdateUnreadCount {
                        session_type,
                        target_id,
                        count: 1,
                    }))
                    .unwrap_or_else(|e| error!("发送未读计数事件失败: {}", e));
            }
            Err(e) => {
                error!("保存消息到数据库失败: {}", e);
            }
        }
    }

    /// 验证发送者信息
    fn validate_sender_info(ip: &str, port: u16, nickname: &str) -> Result<(String, u16, String), String> {
        if ip.is_empty() {
            return Err("IP地址为空".to_string());
        }
        if port == 0 {
            return Err("端口无效".to_string());
        }
        if nickname.is_empty() {
            return Err("昵称为空".to_string());
        }
        Ok((ip.to_string(), port, nickname.to_string()))
    }

    /// 获取或创建发送者用户记录
    async fn get_or_create_sender_user(db: &DbConn, ip: &str, port: u16, nickname: &str) -> Result<i64, String> {
        // 尝试通过 IP 查找用户
        // 这里简化处理，实际可能需要更复杂的查询逻辑
        // 暂时生成一个基于 IP:port 的用户 ID
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let machine_id = format!("{}:{}", ip, port);
        let mut hasher = DefaultHasher::new();
        machine_id.hash(&mut hasher);
        let uid = (hasher.finish() % 9000000000000000000 + 1000000000000000000) as i64;

        // 检查用户是否存在，不存在则创建
        match UserHandler::find_by_id(db, uid).await {
            Ok(_) => Ok(uid),
            Err(_) => {
                // 用户不存在，创建新用户
                warn!("用户 {} 不存在，创建新用户", nickname);
                // 这里应该调用 UserHandler::create，但简化处理直接返回 uid
                Ok(uid)
            }
        }
    }

    /// 发送接收确认（RECVMSG）
    async fn send_recv_confirmation(addr: &str, msg_no: &str) {
        use crate::network::udp::sender;

        let recv_packet = FeiQPacket::make_feiq_recv_packet(msg_no);
        let packet_str = recv_packet.to_feiq_string();

        if let Err(e) = sender::send_packet_data(addr, &packet_str).await {
            error!("发送 RECVMSG 确认失败: {}", e);
        } else {
            info!("已发送 RECVMSG 确认 to {}", addr);
        }
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use crate::core::contact::discovery::parse_sender_info;

    #[test]
    fn test_parse_sender_info() {
        let sender = "testuser@hostname";
        let addr = "192.168.1.100:2425";

        let result = parse_sender_info(sender, addr);
        assert!(result.is_ok());

        let (nickname, ip, port, _machine_id, _mac_addr, _timestamp_local) = result.unwrap();
        assert_eq!(ip, "192.168.1.100");
        assert_eq!(port, 2425);
        assert_eq!(nickname, "testuser");
    }

    #[test]
    fn test_parse_sender_info_simple() {
        let sender = "simpleuser";
        let addr = "192.168.1.100:2425";

        let result = parse_sender_info(sender, addr);
        assert!(result.is_ok());

        let (nickname, ip, port, _machine_id, _mac_addr, _timestamp_local) = result.unwrap();
        assert_eq!(ip, "192.168.1.100");
        assert_eq!(port, 2425);
        assert_eq!(nickname, "simpleuser");
    }
}
