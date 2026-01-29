// src-tauri/src/network/udp/sender.rs
//
/// UDP 发送器 - 使用全局共享的 UDP 套接字
///
/// ## 日志说明
///
/// 发送器会记录详细的日志信息，包括：
/// - 目标地址
/// - 数据包长度和内容
/// - 数据包详情（命令字、发送者、接收者等）
/// - 十六进制格式的原始数据（debug 级别）
///
/// 日志在 `socket.rs` 的 `send_packet_data()` 和 `send_packet()` 函数中输出。
use crate::error::AppResult;
use crate::network::feiq::model::FeiqPacket;

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
///
/// # 日志
/// 此函数会记录详细的发送日志，包括：
/// - 目标地址
/// - 协议类型、版本、命令字
/// - 发送者、接收者、消息编号
/// - 完整数据包内容
pub async fn send_packet(addr: &str, packet: &FeiqPacket) -> AppResult<()> {
    super::socket::send_packet(addr, packet).await
}

/// 广播 UDP 数据包
///
/// # 参数
/// * `packet` - 要广播的 FeiQ 数据包
///
/// # 说明
/// 此函数会将数据包广播到 255.255.255.255:2425，用于在线通知等场景。
///
/// # 日志
/// 详细的发送日志会通过 `send_packet()` 记录
pub async fn broadcast_packet(packet: &FeiqPacket) -> AppResult<()> {
    super::socket::broadcast_packet(packet).await
}
