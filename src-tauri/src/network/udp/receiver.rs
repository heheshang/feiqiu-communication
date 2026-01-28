// src-tauri/src/network/udp/receiver.rs
//
use crate::event::bus::EVENT_SENDER;
use crate::event::model::{AppEvent, NetworkEvent};
use crate::network::feiq::parser::parse_feiq_packet;
/// UDP 接收器
use tokio::net::UdpSocket;
use tracing::{error, info};

/// 启动 UDP 接收器
///
/// 绑定 0.0.0.0:2425 端口，接收飞秋协议数据包
pub async fn start_udp_receiver() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2425").await?;
    info!("UDP 接收器已启动，监听端口 2425");

    let mut buf = [0u8; 2048];

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let data = String::from_utf8_lossy(&buf[..len]);

                info!("收到 UDP 数据: {} bytes from {}", len, addr);

                // 解析数据包
                match parse_feiq_packet(&data) {
                    Ok(packet) => {
                        info!("解析成功: command={}, sender={}", packet.command, packet.sender);

                        // 发送到事件总线
                        let event = AppEvent::Network(NetworkEvent::PacketReceived {
                            packet: serde_json::to_string(&packet).unwrap_or_default(),
                            addr: addr.to_string(),
                        });

                        if let Err(e) = EVENT_SENDER.send(event) {
                            error!("事件发送失败: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("数据包解析失败: {} - 数据: {}", e, data);
                    }
                }
            }
            Err(e) => {
                error!("UDP 接收错误: {}", e);
            }
        }
    }
}
