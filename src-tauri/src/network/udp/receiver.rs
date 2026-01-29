// src-tauri/src/network/udp/receiver.rs
//
/// UDP 接收器 - 使用全局共享的 UDP 套接字
use crate::event::bus::EVENT_SENDER;
use crate::event::model::{AppEvent, NetworkEvent};
use crate::network::feiq::model::ProtocolPacket;
use crate::network::feiq::parser::{decode_gbk, parse_feiq_packet};
use tracing::{debug, error, info, warn};

/// 启动 UDP 接收器
///
/// 使用全局共享的 UDP 套接字接收飞秋协议数据包
/// 注意：必须先调用 init_udp_socket() 初始化全局套接字
pub async fn start_udp_receiver() -> Result<(), Box<dyn std::error::Error>> {
    // 获取全局 UDP 套接字
    let socket = super::socket::get_udp_socket();
    info!("UDP 接收器已启动，使用全局共享套接字监听端口 2425");

    let mut buf = [0u8; 2048];

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                // 记录原始接收日志
                info!("========================================");
                info!("📥 [UDP RECV] 来自: {}", addr);
                info!("📦 [RAW BYTES] 长度: {} bytes", len);
                debug!("🔢 [RAW HEX] {:02X?}", &buf[..len]);

                // 尝试使用 GBK 解码 (飞秋协议使用 GBK 编码)
                let decoded = match decode_gbk(&buf[..len]) {
                    Ok(s) => {
                        info!("📝 [GBK DECODE] 成功解码 (GBK -> UTF-8)");
                        s
                    }
                    Err(e) => {
                        // GBK 解码失败，回退到 UTF-8 lossy 解码
                        warn!("⚠️  GBK 解码失败: {}, 回退到 UTF-8 lossy", e);
                        String::from_utf8_lossy(&buf[..len]).to_string()
                    }
                };

                // 记录解码后的字符串内容
                info!("📄 [DECODED MSG] {}", decoded);

                // 解析数据包
                match parse_feiq_packet(&decoded) {
                    Ok(packet) => {
                        info!("✅ [PARSE SUCCESS]");
                        info!("  ├─ 协议类型: {:?}", packet.protocol_type);
                        info!("  ├─ 命令字: 0x{:04X} ({})", packet.command, packet.command);
                        info!("  ├─ 发送者: {}", packet.sender);
                        info!("  ├─ 接收者: {}", packet.receiver);
                        info!("  ├─ 消息编号: {}", packet.msg_no);
                        if let Some(ext) = &packet.extension {
                            info!("  ├─ 附加信息: {}", ext);
                        }
                        if let Some(host) = &packet.hostname {
                            info!("  ├─ 主机名: {}", host);
                        }
                        if let Some(mac) = &packet.mac_addr {
                            info!("  ├─ MAC 地址: {}", mac);
                        }

                        // 发送到事件总线
                        let event = AppEvent::Network(NetworkEvent::PacketReceived {
                            packet: serde_json::to_string(&packet).unwrap_or_default(),
                            addr: addr.to_string(),
                        });

                        if let Err(e) = EVENT_SENDER.send(event) {
                            error!("❌ 事件发送失败: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("❌ [PARSE ERROR] {}", e);
                        error!("   原始数据: {}", decoded);
                    }
                }
            }
            Err(e) => {
                error!("❌ [UDP RECV ERROR] {}", e);
            }
        }
    }
}
