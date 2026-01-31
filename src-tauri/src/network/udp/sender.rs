// src-tauri/src/network/udp/sender.rs
//
/// UDP 发送器 - 使用全局共享的 UDP 套接字
use crate::error::AppResult;
use crate::network::feiq::model::FeiQPacket;

/// 发送 UDP 数据包到指定地址（字符串形式）
///
/// # 参数
/// * `addr` - 目标地址，格式为 "IP:PORT"
/// * `data` - 要发送的数据字符串
///
/// # 日志
/// 此函数会记录发送日志，包括目标地址、数据长度和内容
pub async fn send_packet_data(addr: &str, data: &str) -> AppResult<()> {
    super::socket::send_packet_data(addr, data).await
}

/// 发送 UDP 数据包到指定地址
///
/// # 参数
/// * `addr` - 目标地址，格式为 "IP:PORT"
/// * `packet` - FeiQ 数据包
pub async fn send_packet(addr: &str, packet: &FeiQPacket) -> AppResult<()> {
    super::socket::send_packet(addr, packet).await
}

/// 广播 UDP 数据包到子网广播地址
///
/// # 参数
/// * `packet` - 要广播的 FeiQ 数据包
pub async fn broadcast_packet(packet: &FeiQPacket) -> AppResult<()> {
    super::socket::broadcast_packet(packet).await
}
