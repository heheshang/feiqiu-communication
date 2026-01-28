// src-tauri/src/network/udp/sender.rs
//
use crate::network::feiq::model::FeiqPacket;
/// UDP 发送器
use tokio::net::UdpSocket;

/// 发送 UDP 数据包到指定地址（字符串形式）
#[allow(dead_code)]
pub async fn send_packet_data(addr: &str, data: &str) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    tracing::debug!("发送 UDP 数据到 {}: {}", addr, data);

    socket.send_to(data.as_bytes(), addr).await?;
    Ok(())
}

/// 发送 UDP 数据包到指定地址
#[allow(dead_code)]
pub async fn send_packet(addr: &str, packet: &FeiqPacket) -> anyhow::Result<()> {
    let data = packet.to_string();
    send_packet_data(addr, &data).await
}

/// 广播 UDP 数据包
#[allow(dead_code)]
pub async fn broadcast_packet(packet: &FeiqPacket) -> anyhow::Result<()> {
    send_packet("255.255.255.255:2425", packet).await
}
