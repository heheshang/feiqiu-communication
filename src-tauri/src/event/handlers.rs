use sea_orm::DbConn;
use tracing::{error, info};

use crate::core::file::FileTransferHandler;
use crate::database::handler::{ContactHandler, UserHandler};
use crate::event::model::{NetworkEvent, UiEvent};

pub async fn handle_network_event(event: NetworkEvent, db: &DbConn) {
    match event {
        NetworkEvent::UserOnline {
            ip,
            port,
            nickname,
            hostname,
            mac_addr,
        } => handle_user_online_with_db(db, ip, port, nickname, hostname, mac_addr).await,
        NetworkEvent::UserOffline { ip } => handle_user_offline(ip).await,
        NetworkEvent::UserPresenceResponse {
            ip,
            port,
            nickname,
            hostname,
        } => handle_user_presence_with_db(db, ip, port, nickname, hostname).await,
        NetworkEvent::MessageReceived {
            sender_ip,
            sender_port,
            sender_nickname,
            content,
            msg_no,
            needs_receipt,
        } => {
            handle_message_received(
                sender_ip,
                sender_port,
                sender_nickname,
                content,
                msg_no,
                needs_receipt,
            )
            .await
        }
        NetworkEvent::MessageReceiptReceived { msg_no } => {
            handle_message_receipt(msg_no).await
        }
        NetworkEvent::MessageRead { msg_no } => handle_message_read(msg_no).await,
        NetworkEvent::MessageDeleted { msg_no } => handle_message_deleted(msg_no).await,
        NetworkEvent::FileRequestReceived { from_ip, files } => {
            handle_file_request(from_ip, files).await
        }
        NetworkEvent::UserUpdated { user } => handle_user_updated(user).await,
        NetworkEvent::FileDataRequest {
            from_ip,
            packet_no,
            file_id,
            offset,
        } => handle_file_data_request(db, from_ip, packet_no, file_id, offset).await,
        NetworkEvent::FileDataReceived {
            from_ip,
            packet_no,
            file_id,
            offset,
            data,
        } => handle_file_data_received(db, from_ip, packet_no, file_id, offset, data).await,
        NetworkEvent::FileRelease { from_ip, packet_no } => {
            handle_file_release(db, from_ip, packet_no).await
        }
        _ => {
            info!("收到未处理的事件类型");
        }
    }
}

async fn handle_user_online(
    ip: String,
    port: u16,
    nickname: String,
    hostname: Option<String>,
    mac_addr: Option<String>,
) {
    info!("用户上线事件: {} ({}:{})", nickname, ip, port);
    if let Some(h) = hostname {
        info!("  主机名: {}", h);
    }
    if let Some(m) = mac_addr {
        info!("  MAC: {}", m);
    }
}

/// 处理用户上线事件（带数据库操作）
///
/// 收到 BR_ENTRY 消息后：
/// 1. 更新或创建 user 表记录
/// 2. 确保 contact 表中存在与当前用户的联系人关系
async fn handle_user_online_with_db(
    db: &DbConn,
    ip: String,
    port: u16,
    nickname: String,
    hostname: Option<String>,
    mac_addr: Option<String>,
) {
    info!("用户上线事件: {} ({}:{})", nickname, ip, port);
    if let Some(h) = hostname {
        info!("  主机名: {}", h);
    }
    if let Some(m) = mac_addr {
        info!("  MAC: {}", m);
    }

    let machine_id = format!("{}:{}", ip, port);

    // 1. 更新或创建用户记录（状态：在线）
    match UserHandler::upsert_by_machine_id(db, &machine_id, &ip, port, &nickname, 1).await {
        Ok(user) => {
            info!("✅ 用户已更新/创建: uid={}, nickname={}", user.uid, user.nickname);

            // 2. 获取当前用户 ID
            if let Ok(current_user) = UserHandler::get_current_user(db).await {
                // 不为自己创建联系人关系
                if user.uid != current_user.uid {
                    // 3. 确保联系人关系存在
                    match ContactHandler::ensure_contact(db, current_user.uid, user.uid).await {
                        Ok(_) => {
                            info!("✅ 联系人关系已确保: {} -> {}", current_user.nickname, user.nickname);
                        }
                        Err(e) => {
                            error!("❌ 创建联系人关系失败: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("❌ 更新/创建用户失败: {}", e);
        }
    }
}

async fn handle_user_offline(ip: String) {
    info!("用户离线事件: {}", ip);
}

async fn handle_user_presence(
    ip: String,
    port: u16,
    nickname: String,
    hostname: Option<String>,
) {
    info!("用户在线应答: {} ({}:{})", nickname, ip, port);
    if let Some(h) = hostname {
        info!("  主机名: {}", h);
    }
}

/// 处理用户在线应答（带数据库操作）
///
/// 收到 ANSENTRY 消息后：
/// 1. 更新或创建 user 表记录
/// 2. 确保 contact 表中存在与当前用户的联系人关系
pub async fn handle_user_presence_with_db(
    db: &DbConn,
    ip: String,
    port: u16,
    nickname: String,
    hostname: Option<String>,
) {
    info!("用户在线应答: {} ({}:{})", nickname, ip, port);
    if let Some(h) = hostname {
        info!("  主机名: {}", h);
    }

    let machine_id = format!("{}:{}", ip, port);

    // 1. 更新或创建用户记录（状态：在线）
    match UserHandler::upsert_by_machine_id(db, &machine_id, &ip, port, &nickname, 1).await {
        Ok(user) => {
            info!("✅ 用户已更新/创建: uid={}, nickname={}", user.uid, user.nickname);

            // 2. 获取当前用户 ID
            if let Ok(current_user) = UserHandler::get_current_user(db).await {
                // 不为自己创建联系人关系
                if user.uid != current_user.uid {
                    // 3. 确保联系人关系存在
                    match ContactHandler::ensure_contact(db, current_user.uid, user.uid).await {
                        Ok(_) => {
                            info!("✅ 联系人关系已确保: {} -> {}", current_user.nickname, user.nickname);
                        }
                        Err(e) => {
                            error!("❌ 创建联系人关系失败: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("❌ 更新/创建用户失败: {}", e);
        }
    }
}

async fn handle_message_received(
    sender_ip: String,
    sender_port: u16,
    sender_nickname: String,
    content: String,
    msg_no: String,
    needs_receipt: bool,
) {
    info!("收到消息: {} from {}:{}", msg_no, sender_ip, sender_port);
    info!("  发送者: {}", sender_nickname);
    info!("  内容: {}", content);
    info!("  需要确认: {}", needs_receipt);
}

async fn handle_message_receipt(msg_no: String) {
    info!("收到消息确认: {}", msg_no);
}

async fn handle_message_read(msg_no: String) {
    info!("消息已读: {}", msg_no);
}

async fn handle_message_deleted(msg_no: String) {
    info!("消息已删除: {}", msg_no);
}

async fn handle_file_request(from_ip: String, files: String) {
    info!("收到文件请求: from {}", from_ip);
    info!("  文件信息: {}", files);
}

async fn handle_user_updated(user: String) {
    info!("用户更新事件: {}", user);
}

async fn handle_file_data_request(
    db: &DbConn,
    from_ip: String,
    packet_no: String,
    file_id: u64,
    offset: u64,
) {
    if let Err(e) =
        FileTransferHandler::handle_file_data_request(db, &from_ip, &packet_no, file_id, offset)
            .await
    {
        error!("处理文件数据请求失败: {}", e);
    }
}

async fn handle_file_data_received(
    db: &DbConn,
    from_ip: String,
    packet_no: String,
    file_id: u64,
    offset: u64,
    data: String,
) {
    if let Err(e) = FileTransferHandler::handle_file_data_received(
        db, &from_ip, &packet_no, file_id, offset, &data,
    )
    .await
    {
        error!("处理文件数据接收失败: {}", e);
    }
}

async fn handle_file_release(db: &DbConn, from_ip: String, packet_no: String) {
    if let Err(e) = FileTransferHandler::handle_file_release(db, &from_ip, &packet_no).await {
        error!("处理文件释放失败: {}", e);
    }
}

pub async fn handle_ui_event(_event: UiEvent) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_user_online() {
        handle_user_online(
            "192.168.1.100".to_string(),
            2425,
            "测试用户".to_string(),
            Some("test-host".to_string()),
            Some("00:11:22:33:44:55".to_string()),
        )
        .await;
    }

    #[tokio::test]
    async fn test_handle_user_offline() {
        handle_user_offline("192.168.1.100".to_string()).await;
    }
}
