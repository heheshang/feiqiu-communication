// src-tauri/src/core/contact/discovery.rs
//
/// 用户在线发现模块
///
/// 功能:
/// - 启动时广播 BR_ENTRY 包
/// - 监听其他用户的 BR_ENTRY 并回复 ANSENTRY
/// - 维护在线用户列表
/// - 处理用户离线事件
use crate::event::bus::EVENT_RECEIVER;
use crate::event::model::{AppEvent, NetworkEvent, UiEvent};
use crate::network::feiq::{constants::*, model::FeiqPacket};
use crate::network::udp::sender::send_packet_data;
use crate::types::UserInfo;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};

/// 在线用户列表
///
/// Key: machine_id (格式: IP:port)
/// Value: 用户信息
type OnlineUsers = Arc<Mutex<HashMap<String, UserInfo>>>;

/// 全局在线用户列表
static ONLINE_USERS: OnceCell<OnlineUsers> = OnceCell::new();

use once_cell::sync::OnceCell;

/// 获取全局在线用户列表
pub fn get_online_users() -> &'static OnlineUsers {
    ONLINE_USERS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

/// 获取在线用户列表的副本
pub fn get_online_users_list() -> Vec<UserInfo> {
    let users = get_online_users();
    users.lock().unwrap().values().cloned().collect()
}

/// 添加或更新在线用户
pub fn add_online_user(user: UserInfo) {
    let users = get_online_users();
    let machine_id = format!("{}:{}", user.feiq_ip, user.feiq_port);

    let mut users_guard = users.lock().unwrap();
    let is_new = !users_guard.contains_key(&machine_id);

    users_guard.insert(machine_id.clone(), user.clone());

    if is_new {
        info!("新用户上线: {} ({})", user.nickname, machine_id);
    } else {
        info!("用户信息更新: {} ({})", user.nickname, machine_id);
    }
}

/// 移除在线用户
pub fn remove_online_user(ip: &str) {
    let users = get_online_users();
    let mut users_guard = users.lock().unwrap();

    // 查找并移除匹配 IP 的用户
    let keys_to_remove: Vec<String> = users_guard
        .keys()
        .filter(|k| k.starts_with(&format!("{}:", ip)))
        .cloned()
        .collect();

    for key in keys_to_remove {
        if let Some(user) = users_guard.remove(&key) {
            info!("用户离线: {} ({})", user.nickname, key);
        }
    }
}

/// 根据 IP 查找用户
pub fn find_user_by_ip(ip: &str) -> Option<UserInfo> {
    let users = get_online_users();
    let users_guard = users.lock().unwrap();

    for (machine_id, user) in users_guard.iter() {
        if machine_id.starts_with(&format!("{}:", ip)) {
            return Some(user.clone());
        }
    }

    None
}

/// 启动用户发现服务
///
/// 流程:
/// 1. 广播 BR_ENTRY 包（上线通知）
/// 2. 监听事件总线，处理网络事件
/// 3. 收到 BR_ENTRY 时回复 ANSENTRY
/// 4. 收到 ANSENTRY 时添加到在线列表
/// 5. 收到 BR_EXIT 时从在线列表移除
pub async fn start_discovery() -> anyhow::Result<()> {
    info!("用户发现服务启动中...");

    // 1. 广播上线
    broadcast_entry().await?;

    // 2. 监听事件总线
    let receiver = EVENT_RECEIVER.clone();
    tokio::spawn(async move {
        discovery_event_loop(receiver).await;
    });

    info!("用户发现服务已启动");
    Ok(())
}

/// 广播上线通知
async fn broadcast_entry() -> anyhow::Result<()> {
    info!("广播上线通知...");

    let packet = FeiqPacket::make_entry_packet();
    let packet_str = packet.to_string();

    // 广播到 255.255.255.255:2425
    send_packet_data(&format!("{}:{}", FEIQ_BROADCAST_ADDR, FEIQ_DEFAULT_PORT), &packet_str).await?;

    info!("上线通知已广播");
    Ok(())
}

/// 发送在线响应
async fn send_ansentry(addr: &str) -> anyhow::Result<()> {
    info!("回复 ANSENTRY to {}", addr);

    let packet = FeiqPacket::make_ansentry_packet();
    let packet_str = packet.to_string();

    send_packet_data(addr, &packet_str).await?;

    info!("ANSENTRY 已发送到 {}", addr);
    Ok(())
}

/// 用户发现事件循环
async fn discovery_event_loop(receiver: crossbeam_channel::Receiver<AppEvent>) {
    info!("用户发现事件循环启动");

    loop {
        match receiver.recv() {
            Ok(event) => {
                if let AppEvent::Network(net_event) = event {
                    handle_network_event(net_event).await;
                }
            }
            Err(e) => {
                error!("事件接收失败: {}", e);
            }
        }
    }
}

/// 处理网络事件
async fn handle_network_event(event: NetworkEvent) {
    match event {
        NetworkEvent::PacketReceived { packet, addr } => {
            handle_packet_received(packet, addr).await;
        }
        _ => {}
    }
}

/// 处理收到的数据包
async fn handle_packet_received(packet_json: String, addr: String) {
    // 反序列化数据包
    let packet: FeiqPacket = match serde_json::from_str(&packet_json) {
        Ok(p) => p,
        Err(e) => {
            error!("数据包反序列化失败: {}", e);
            return;
        }
    };

    // 解析发送者信息
    let sender_info = match parse_sender_info(&packet.sender, &addr) {
        Ok(info) => info,
        Err(e) => {
            warn!("无法解析发送者信息: {} - {}", packet.sender, e);
            return;
        }
    };

    let (nickname, ip, port, machine_id) = sender_info;

    // 根据命令字处理
    let base_cmd = packet.base_command();

    match base_cmd {
        IPMSG_BR_ENTRY => {
            info!("收到 BR_ENTRY from {}", addr);

            // 创建用户信息
            let user = UserInfo {
                uid: 0, // TODO: 生成唯一 ID
                nickname: nickname.clone(),
                feiq_ip: ip.clone(),
                feiq_port: port,
                feiq_machine_id: machine_id.clone(),
                avatar: None,
                status: 1, // 在线
            };

            // 添加到在线列表
            add_online_user(user);

            // 回复 ANSENTRY
            if let Err(e) = send_ansentry(&addr).await {
                error!("发送 ANSENTRY 失败: {}", e);
            }
        }

        IPMSG_ANSENTRY => {
            info!("收到 ANSENTRY from {}", addr);

            // 创建用户信息
            let user = UserInfo {
                uid: 0, // TODO: 生成唯一 ID
                nickname: nickname.clone(),
                feiq_ip: ip.clone(),
                feiq_port: port,
                feiq_machine_id: machine_id.clone(),
                avatar: None,
                status: 1, // 在线
            };

            // 添加到在线列表
            add_online_user(user);
        }

        IPMSG_BR_EXIT => {
            info!("收到 BR_EXIT from {}", addr);

            // 从在线列表移除
            remove_online_user(&ip);
        }

        IPMSG_SENDMSG => {
            // 消息处理在聊天模块
            info!("收到 SENDMSG from {}", addr);
        }

        _ => {
            info!("收到未处理的命令字: {} from {}", base_cmd, addr);
        }
    }
}

/// 解析发送者信息
///
/// 根据 IPMsg/FeiQ 协议格式解析发送者信息
///
/// # IPMsg 协议格式
/// ```text
/// 版本号:命令字:发送者:接收者:消息编号:附加信息
/// Example: 1.0:32:sender:host:12345:Hello
/// ```
/// - `sender` 字段: 发送者标识符（如 "sender" 或 "user@hostname"）
/// - IP:port 从 UDP 包的 addr 参数获取
///
/// # FeiQ 协议格式
/// ```text
/// Header: 版本号#长度#MAC地址#端口#标志1#标志2#命令#类型
/// Data: 时间戳:包ID:主机名:用户ID:内容
/// Example: 1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:SHIKUN-SH:6291459:ssk
/// ```
/// - `sender` 字段: `hostname@mac_addr` 格式
/// - IP:port 从 UDP 包的 addr 参数获取
///
/// # 参数
/// - `sender`: 数据包中的 sender 字段
/// - `addr`: UDP 包的源地址（格式: "IP:port"）
///
/// # 返回
/// - `Ok((nickname, ip, port, machine_id))`: 解析成功
/// - `Err`: 解析失败
fn parse_sender_info(sender: &str, addr: &str) -> Result<(String, String, u16, String), String> {
    // 从 addr 解析 IP 和端口
    let addr_parts: Vec<&str> = addr.split(':').collect();
    let ip = addr_parts
        .first()
        .ok_or_else(|| format!("Invalid addr format: {}", addr))?
        .to_string();
    let port: u16 = addr_parts
        .get(1)
        .and_then(|p| p.parse().ok())
        .ok_or_else(|| format!("Invalid port in addr: {}", addr))?;

    // 解析 nickname
    // IPMsg: sender 可能是 "nickname" 或 "nickname@hostname"
    // FeiQ: sender 是 "hostname@mac_addr"
    let nickname = if let Some(at_pos) = sender.find('@') {
        // 包含 @，提取 @ 之前的部分
        sender[..at_pos].to_string()
    } else {
        // 不包含 @，整个 sender 就是 nickname
        sender.to_string()
    };

    // 生成机器 ID
    let machine_id = format!("{}:{}", ip, port);

    Ok((nickname, ip, port, machine_id))
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sender_info_ipmsg_simple() {
        // IPMsg 协议: sender 字段是简单标识符
        // 协议格式: 版本号:命令字:发送者:接收者:消息编号:附加信息
        // Example: 1.0:32:sender:host:12345:Hello
        let sender = "sender";
        let addr = "192.168.1.100:2425";

        let result = parse_sender_info(sender, addr);

        assert!(result.is_ok());
        let (nickname, ip, port, machine_id) = result.unwrap();
        assert_eq!(nickname, "sender");
        assert_eq!(ip, "192.168.1.100");
        assert_eq!(port, 2425);
        assert_eq!(machine_id, "192.168.1.100:2425");
    }

    #[test]
    fn test_parse_sender_info_ipmsg_with_host() {
        // IPMsg 协议: sender 字段包含用户名@主机名
        let sender = "user@hostname";
        let addr = "192.168.1.100:2425";

        let result = parse_sender_info(sender, addr);

        assert!(result.is_ok());
        let (nickname, ip, port, machine_id) = result.unwrap();
        assert_eq!(nickname, "user");
        assert_eq!(ip, "192.168.1.100");
        assert_eq!(port, 2425);
        assert_eq!(machine_id, "192.168.1.100:2425");
    }

    #[test]
    fn test_parse_sender_info_feiq() {
        // FeiQ 协议: sender 字段是 hostname@mac_addr
        // Header: 版本号#长度#MAC地址#端口#标志1#标志2#命令#类型
        // Data: 时间戳:包ID:主机名:用户ID:内容
        let sender = "SHIKUN-SH@5C60BA7361C6";
        let addr = "192.168.1.100:6452";

        let result = parse_sender_info(sender, addr);

        assert!(result.is_ok());
        let (nickname, ip, port, machine_id) = result.unwrap();
        assert_eq!(nickname, "SHIKUN-SH");
        assert_eq!(ip, "192.168.1.100");
        assert_eq!(port, 6452);
        assert_eq!(machine_id, "192.168.1.100:6452");
    }

    #[test]
    fn test_parse_sender_info_invalid_addr() {
        // 测试无效的 addr 格式
        let sender = "user";
        let addr = "invalid-addr";

        let result = parse_sender_info(sender, addr);

        assert!(result.is_err());
    }

    #[test]
    fn test_add_remove_online_user() {
        let user = UserInfo {
            uid: 1,
            nickname: "Test User".to_string(),
            feiq_ip: "192.168.1.100".to_string(),
            feiq_port: 2425,
            feiq_machine_id: "192.168.1.100:2425".to_string(),
            avatar: None,
            status: 1,
        };

        add_online_user(user.clone());

        let users = get_online_users_list();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].nickname, "Test User");

        remove_online_user("192.168.1.100");

        let users = get_online_users_list();
        assert_eq!(users.len(), 0);
    }
}
