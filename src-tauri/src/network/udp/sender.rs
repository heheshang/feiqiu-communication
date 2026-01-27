// src-tauri/src/network/udp/sender.rs
//
/// UDP 发送器

use tokio::net::UdpSocket;
use crate::network::feiq::model::FeiqPacket;

/// 发送 UDP 数据包到指定地址
#[allow(dead_code)]
pub async fn send_packet(addr: &str, packet: &FeiqPacket) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let data = packet.to_string();

    tracing::debug!("发送 UDP 数据到 {}: {}", addr, data);

    socket.send_to(data.as_bytes(), addr).await?;
    Ok(())
}

/// 广播 UDP 数据包
#[allow(dead_code)]
pub async fn broadcast_packet(packet: &FeiqPacket) -> anyhow::Result<()> {
    send_packet("255.255.255.255:2425", packet).await
}
