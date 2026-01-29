// src-tauri/src/network/udp/socket.rs
//
/// 全局 UDP 套接字管理器
///
/// 负责管理单个 UDP 套接字，用于发送和接收数据
use crate::error::{AppError, AppResult};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{debug, info};

/// 全局 UDP 套接字（使用 OnceCell 支持运行时初始化）
static UDP_SOCKET: once_cell::sync::OnceCell<Arc<UdpSocket>> = once_cell::sync::OnceCell::new();

/// 初始化全局 UDP 套接字
///
/// 必须在应用启动时调用一次，绑定到 FeiQ 标准端口 2425
pub async fn init_udp_socket() -> AppResult<()> {
    // 检查是否已经初始化
    if UDP_SOCKET.get().is_some() {
        tracing::warn!("UDP socket 已经初始化，跳过重复初始化");
        return Ok(());
    }

    // 绑定到 0.0.0.0:2425，监听所有网络接口
    let socket = UdpSocket::bind("0.0.0.0:2425")
        .await
        .map_err(|e| AppError::Network(format!("Failed to bind UDP socket to 0.0.0.0:2425: {}", e)))?;

    // 设置广播选项 (Windows 必须显式设置才能广播到 255.255.255.255)
    socket
        .set_broadcast(true)
        .map_err(|e| AppError::Network(format!("Failed to set SO_BROADCAST option: {}", e)))?;

    // 设置为全局变量
    UDP_SOCKET
        .set(Arc::new(socket))
        .map_err(|_| "UDP socket already initialized".to_string())
        .unwrap();

    tracing::info!("UDP socket 已绑定到 0.0.0.0:2425 (broadcast enabled)");
    Ok(())
}

/// 获取全局 UDP 套接字
pub fn get_udp_socket() -> Arc<UdpSocket> {
    UDP_SOCKET
        .get()
        .expect("UDP socket not initialized. Call init_udp_socket() first.")
        .clone()
}

/// 发送 UDP 数据包
///
/// # 参数
/// * `addr` - 目标地址 (格式: "IP:PORT")
/// * `data` - 要发送的数据字符串
pub async fn send_packet_data(addr: &str, data: &str) -> AppResult<()> {
    let socket = get_udp_socket();
    let bytes = data.as_bytes();

    // 记录发送日志
    info!("========================================");
    info!("📤 [UDP SEND] 目标: {}", addr);
    info!("📦 [DATA BYTES] 长度: {} bytes", bytes.len());
    info!("📄 [DATA CONTENT] {}", data);
    debug!("🔢 [DATA HEX] {:02X?}", bytes);

    socket
        .send_to(bytes, addr)
        .await
        .map_err(|e| AppError::Network(format!("Failed to send UDP data to {}: {}", addr, e)))?;

    info!("✅ [SEND SUCCESS] 已发送 {} bytes 到 {}", bytes.len(), addr);
    Ok(())
}

/// 发送 FeiQ 数据包
///
/// # 参数
/// * `addr` - 目标地址
/// * `packet` - FeiQ 数据包
pub async fn send_packet(addr: &str, packet: &crate::network::feiq::model::FeiqPacket) -> AppResult<()> {
    let data = packet.to_string();

    // 记录数据包详情
    info!("┌────────────────────────────────────────");
    info!("│ [PACKET INFO]");
    info!("│ ├─ 目标地址: {}", addr);
    info!("│ ├─ 协议类型: {:?}", packet.protocol_type);
    info!("│ ├─ 版本: {}", packet.version);
    info!("│ ├─ 命令字: 0x{:04X} ({})", packet.command, packet.command);
    info!("│ ├─ 发送者: {}", packet.sender);
    info!("│ ├─ 接收者: {}", packet.receiver);
    info!("│ ├─ 消息编号: {}", packet.msg_no);
    if let Some(ext) = &packet.extension {
        info!("│ ├─ 附加信息: {}", ext);
    }
    info!("│ ├─ 完整数据包: {}", data);
    info!("└────────────────────────────────────────");

    send_packet_data(addr, &data).await
}

/// 广播 FeiQ 数据包
///
/// # 参数
/// * `packet` - 要广播的 FeiQ 数据包
pub async fn broadcast_packet(packet: &crate::network::feiq::model::FeiqPacket) -> AppResult<()> {
    send_packet("255.255.255.255:2425", packet).await
}
