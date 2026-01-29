// src-tauri/src/network/udp/sender.rs
//
/// UDP 发送器
use crate::error::{AppError, AppResult};
use crate::network::feiq::model::FeiqPacket;
use tokio::net::UdpSocket;

/// 发送 UDP 数据包到指定地址（字符串形式）
#[allow(dead_code)]
pub async fn send_packet_data(addr: &str, data: &str) -> AppResult<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(|e| AppError::Network(format!("Failed to bind UDP socket: {}", e)))?;

    tracing::debug!("发送 UDP 数据到 {}: {}", addr, data);

    socket
        .send_to(data.as_bytes(), addr)
        .await
        .map_err(|e| AppError::Network(format!("Failed to send UDP data to {}: {}", addr, e)))?;
    Ok(())
}

/// 发送 UDP 数据包到指定地址
#[allow(dead_code)]
pub async fn send_packet(addr: &str, packet: &FeiqPacket) -> AppResult<()> {
    let data = packet.to_string();
    send_packet_data(addr, &data).await
}

/// 广播 UDP 数据包
#[allow(dead_code)]
pub async fn broadcast_packet(packet: &FeiqPacket) -> AppResult<()> {
    send_packet("255.255.255.255:2425", packet).await
}
