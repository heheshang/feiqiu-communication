// src-tauri/src/core/chat/receipt.rs
//
/// 已读回执处理器
///
/// 负责处理消息已读回执功能：
/// - 接收已读回执（ANSREADMSG）
/// - 发送已读回执
/// - 更新消息已读状态
/// - 处理 READMSG 命令
use crate::database::handler::ChatMessageHandler;
use crate::event::bus::EVENT_RECEIVER;
use crate::event::model::{AppEvent, NetworkEvent, UiEvent};
use crate::network::feiq::{constants::*, model::ProtocolPacket};
use crate::network::udp::sender;
use sea_orm::DbConn;
use std::sync::Arc;
use tracing::{error, info, warn};

/// 已读回执处理器
pub struct ReceiptHandler {
    db: Arc<DbConn>,
}

impl ReceiptHandler {
    /// 创建新的已读回执处理器
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }

    /// 启动已读回执处理
    ///
    /// 在后台任务中监听事件总线，处理已读回执相关的网络事件
    pub fn start(&self) {
        let db = self.db.clone();
        let receiver = EVENT_RECEIVER.clone();

        tokio::spawn(async move {
            info!("已读回执处理器已启动");
            Self::event_loop(db, receiver).await;
        });
    }

    /// 事件循环
    async fn event_loop(db: Arc<DbConn>, receiver: crossbeam_channel::Receiver<AppEvent>) {
        loop {
            match receiver.recv() {
                Ok(event) => {
                    // 处理网络事件中的数据包接收
                    if let AppEvent::Network(NetworkEvent::PacketReceived { packet, addr }) = event {
                        Self::handle_packet_received(db.clone(), packet, addr).await;
                    }
                }
                Err(e) => {
                    error!("事件接收失败: {}", e);
                }
            }
        }
    }

    /// 处理接收到的数据包
    async fn handle_packet_received(db: Arc<DbConn>, packet_json: String, addr: String) {
        // 反序列化数据包
        let packet: ProtocolPacket = match serde_json::from_str(&packet_json) {
            Ok(p) => p,
            Err(e) => {
                error!("数据包反序列化失败: {}", e);
                return;
            }
        };

        let base_cmd = packet.base_command();

        match base_cmd {
            // 处理已读回执（ANSREADMSG）
            IPMSG_ANSREADMSG => {
                Self::handle_ansreadmsg(db, packet, addr).await;
            }
            // 处理消息已读请求（READMSG）
            IPMSG_READMSG => {
                Self::handle_readmsg(db, packet, addr).await;
            }
            _ => {}
        }
    }

    /// 处理已读回执（ANSREADMSG）
    ///
    /// 当对方阅读了我们发送的消息后，会发送 ANSREADMSG 回执
    async fn handle_ansreadmsg(db: Arc<DbConn>, packet: ProtocolPacket, _addr: String) {
        info!("收到已读回执");

        // 从附加信息中提取消息编号
        let msg_no = packet.msg_no.clone();

        if msg_no.is_empty() {
            warn!("已读回执缺少消息编号");
            return;
        }

        // 通过 msg_no 查找消息并更新状态
        match ChatMessageHandler::find_by_msg_no(&db, &msg_no).await {
            Ok(Some(message)) => {
                // 更新消息状态为已读（2）
                if let Err(e) = ChatMessageHandler::update_status(&db, message.mid, 2).await {
                    error!("更新消息已读状态失败: {}", e);
                } else {
                    info!("消息已标记为已读: mid={}", message.mid);

                    // 触发 UI 事件：更新消息状态
                    let _ = crate::event::bus::EVENT_SENDER.send(AppEvent::Ui(UiEvent::UpdateMessageStatus {
                        msg_id: message.mid,
                        status: 2,
                    }));
                }
            }
            Ok(None) => {
                // 如果找不到消息，可能是旧数据（msg_no 为空）
                // 尝试将 msg_no 解析为消息 ID（向后兼容）
                if let Ok(mid) = msg_no.parse::<i64>() {
                    warn!("使用兼容模式查找消息: mid={}", mid);
                    if let Err(e) = ChatMessageHandler::update_status(&db, mid, 2).await {
                        error!("更新消息已读状态失败: {}", e);
                    }
                } else {
                    warn!("找不到对应的消息: msg_no={}", msg_no);
                }
            }
            Err(e) => {
                error!("查找消息失败: {}", e);
            }
        }
    }

    /// 处理消息已读请求（READMSG）
    ///
    /// 对方阅读了我们发送的消息后，会发送 READMSG 请求
    /// 我们需要回复 ANSREADMSG 确认
    async fn handle_readmsg(db: Arc<DbConn>, packet: ProtocolPacket, addr: String) {
        info!("收到消息已读请求");

        // 从附加信息中提取消息编号
        let msg_no = packet.msg_no.clone();

        if msg_no.is_empty() {
            warn!("已读请求缺少消息编号");
            return;
        }

        // 通过 msg_no 查找消息并更新状态
        match ChatMessageHandler::find_by_msg_no(&db, &msg_no).await {
            Ok(Some(message)) => {
                // 更新消息状态为已读（2）
                if let Err(e) = ChatMessageHandler::update_status(&db, message.mid, 2).await {
                    error!("更新消息已读状态失败: {}", e);
                } else {
                    info!("消息已标记为已读: mid={}", message.mid);

                    // 触发 UI 事件：更新消息状态
                    let _ = crate::event::bus::EVENT_SENDER.send(AppEvent::Ui(UiEvent::UpdateMessageStatus {
                        msg_id: message.mid,
                        status: 2,
                    }));
                }

                // 发送 ANSREADMSG 确认
                Self::send_ansreadmsg(&addr, &msg_no).await;
            }
            Ok(None) => {
                warn!("找不到对应的消息: msg_no={}", msg_no);
            }
            Err(e) => {
                error!("查找消息失败: {}", e);
            }
        }
    }

    /// 发送已读回执（ANSREADMSG）
    ///
    /// 当我们阅读了对方发送的消息后，调用此方法发送回执
    pub async fn send_read_receipt(db: &DbConn, mid: i64, target_ip: &str) -> Result<(), String> {
        info!("发送已读回执: mid={}, target_ip={}", mid, target_ip);

        // 获取消息详情
        let message = ChatMessageHandler::find_by_id(db, mid)
            .await
            .map_err(|e| format!("查找消息失败: {}", e))?;

        // 检查消息是否有 msg_no
        let msg_no = message.msg_no.ok_or_else(|| {
            warn!("消息没有 msg_no，无法发送已读回执");
            "消息没有 msg_no".to_string()
        })?;

        // 构造 ANSREADMSG 包
        let packet = ProtocolPacket::make_read_packet(&msg_no);
        let packet_str = packet.to_string();

        // 发送到目标地址
        let addr = format!("{}:{}", target_ip, 2425);
        sender::send_packet_data(&addr, &packet_str)
            .await
            .map_err(|e| format!("发送已读回执失败: {}", e))?;

        info!("已读回执已发送: addr={}", addr);
        Ok(())
    }

    /// 发送已读确认（ANSREADMSG）
    async fn send_ansreadmsg(addr: &str, msg_no: &str) {
        let packet = ProtocolPacket::make_ansread_packet(msg_no);
        let packet_str = packet.to_string();

        if let Err(e) = sender::send_packet_data(addr, &packet_str).await {
            error!("发送 ANSREADMSG 确认失败: {}", e);
        } else {
            info!("已发送 ANSREADMSG 确认 to {}", addr);
        }
    }

    /// 标记消息已读（本地操作）
    ///
    /// 用户打开聊天窗口查看消息时调用
    /// 不发送网络回执，仅更新本地状态
    pub async fn mark_message_read_locally(db: &DbConn, mid: i64) -> Result<(), String> {
        info!("标记消息已读（本地）: mid={}", mid);

        ChatMessageHandler::update_status(db, mid, 2)
            .await
            .map_err(|e| format!("更新消息状态失败: {}", e))?;

        // 触发 UI 事件：更新消息状态
        let _ =
            crate::event::bus::EVENT_SENDER.send(AppEvent::Ui(UiEvent::UpdateMessageStatus { msg_id: mid, status: 2 }));

        Ok(())
    }

    /// 批量标记会话的消息已读
    ///
    /// 用户打开聊天窗口时，批量标记该会话的所有未读消息为已读
    pub async fn mark_session_messages_read(
        _db: &DbConn,
        _owner_uid: i64,
        _session_type: i8,
        _target_id: i64,
    ) -> Result<usize, String> {
        info!("批量标记会话消息已读");

        // TODO: 实现 ChatMessageHandler::mark_session_read 方法
        // 这需要查询该会话的所有未读消息并批量更新状态

        Ok(0)
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receipt_handler_module_exists() {
        // 这是一个简单的存在性测试
        assert_eq!(std::mem::size_of::<ReceiptHandler>(), 8); // Arc<DbConn> 的大小
    }
}
