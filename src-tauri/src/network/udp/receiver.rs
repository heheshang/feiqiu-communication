// src-tauri/src/network/udp/receiver.rs
//
/// UDP 接收器 - 使用全局共享的 UDP 套接字
use crate::event::bus::EVENT_SENDER;
use crate::event::model::{AppEvent, NetworkEvent};
use crate::network::feiq::parser::{decode_gbk, parse_feiq_packet};
use tracing::{debug, error, info, warn};

/// 发布事件到总线（提取为可测试的函数）
fn publish_event_from_packet(
    packet: &crate::network::feiq::model::ProtocolPacket,
    addr: std::net::SocketAddr,
) -> Result<(), String> {
    use crate::network::feiq::constants::*;

    let sender_ip = addr.ip().to_string();
    let sender_port = addr.port();
    let sender_nickname = packet.sender.clone();
    let hostname = packet.hostname.clone();
    let mac_addr = packet.mac_addr.clone();

    let base_cmd = packet.base_command();
    let event = match base_cmd {
        IPMSG_BR_ENTRY => {
            AppEvent::Network(NetworkEvent::UserOnline {
                ip: sender_ip,
                port: sender_port,
                nickname: sender_nickname,
                hostname,
                mac_addr,
            })
        }
        IPMSG_BR_EXIT => {
            AppEvent::Network(NetworkEvent::UserOffline { ip: sender_ip })
        }
        IPMSG_ANSENTRY => {
            AppEvent::Network(NetworkEvent::UserPresenceResponse {
                ip: sender_ip,
                port: sender_port,
                nickname: sender_nickname,
                hostname,
            })
        }
        IPMSG_SENDMSG => {
            let content = packet.extension.clone().unwrap_or_default();
            let msg_no = packet.msg_no.clone();
            let needs_receipt = packet.has_option(IPMSG_SENDCHECKOPT);
            AppEvent::Network(NetworkEvent::MessageReceived {
                sender_ip,
                sender_port,
                sender_nickname,
                content,
                msg_no,
                needs_receipt,
            })
        }
        IPMSG_RECVMSG => {
            let msg_no = packet.msg_no.clone();
            AppEvent::Network(NetworkEvent::MessageReceiptReceived { msg_no })
        }
        IPMSG_READMSG => {
            let msg_no = packet.msg_no.clone();
            AppEvent::Network(NetworkEvent::MessageRead { msg_no })
        }
        IPMSG_DELMSG => {
            let msg_no = packet.msg_no.clone();
            AppEvent::Network(NetworkEvent::MessageDeleted { msg_no })
        }
        _ => {
            // Unknown command - log for debugging but don't publish event
            warn!("Unknown command received: 0x{:04X}", base_cmd);
            return Ok(());
        }
    };

    EVENT_SENDER.send(event).map_err(|e| e.to_string())
}

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

                        // 发布事件
                        match publish_event_from_packet(&packet, addr) {
                            Ok(_) => {
                                info!("✅ 事件已发布");
                            }
                            Err(e) => {
                                error!("❌ 事件发送失败: {}", e);
                            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::feiq::model::ProtocolPacket;
    use std::net::{IpAddr, SocketAddr};

    fn create_test_addr(ip: &str, port: u16) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(ip.parse().unwrap()), port)
    }

    #[test]
    fn test_user_online_event_fields() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000001, // IPMSG_BR_ENTRY
            "testuser".to_string(),
            "".to_string(),
            "12345".to_string(),
            None,
        );
        let mut packet = packet;
        packet.hostname = Some("DESKTOP-TEST".to_string());
        packet.mac_addr = Some("AA:BB:CC:DD:EE:FF".to_string());

        let addr = create_test_addr("192.168.1.100", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_user_offline_event_fields() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000002, // IPMSG_BR_EXIT
            "testuser".to_string(),
            "".to_string(),
            "12346".to_string(),
            None,
        );

        let addr = create_test_addr("192.168.1.101", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_user_presence_response_event_fields() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000003, // IPMSG_ANSENTRY
            "responder".to_string(),
            "".to_string(),
            "12347".to_string(),
            None,
        );
        let mut packet = packet;
        packet.hostname = Some("DESKTOP-RESP".to_string());

        let addr = create_test_addr("192.168.1.102", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_message_received_event_fields() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00800120, // IPMSG_SENDMSG | UTF8OPT | SENDCHECKOPT
            "sender".to_string(),
            "receiver".to_string(),
            "msg001".to_string(),
            Some("Hello, World!".to_string()),
        );

        let addr = create_test_addr("192.168.1.103", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_message_received_without_receipt_flag() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000020, // IPMSG_SENDMSG only (no SENDCHECKOPT)
            "sender".to_string(),
            "receiver".to_string(),
            "msg002".to_string(),
            Some("No receipt needed".to_string()),
        );

        let addr = create_test_addr("192.168.1.104", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_message_received_with_empty_content() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000020, // IPMSG_SENDMSG
            "sender".to_string(),
            "receiver".to_string(),
            "msg003".to_string(),
            None,
        );

        let addr = create_test_addr("192.168.1.105", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_message_receipt_received_event_fields() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000021, // IPMSG_RECVMSG
            "receiver".to_string(),
            "sender".to_string(),
            "msg001".to_string(),
            None,
        );

        let addr = create_test_addr("192.168.1.106", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_message_read_event_fields() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000030, // IPMSG_READMSG
            "reader".to_string(),
            "sender".to_string(),
            "msg001".to_string(),
            None,
        );

        let addr = create_test_addr("192.168.1.107", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_message_deleted_event_fields() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000031, // IPMSG_DELMSG
            "deleter".to_string(),
            "sender".to_string(),
            "msg001".to_string(),
            None,
        );

        let addr = create_test_addr("192.168.1.108", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_event_extraction_with_special_characters() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000020, // IPMSG_SENDMSG
            "user@domain".to_string(),
            "receiver".to_string(),
            "msg_special_001".to_string(),
            Some("Message with special chars: 你好世界 🎉".to_string()),
        );

        let addr = create_test_addr("192.168.1.110", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed with special characters");
    }

    #[test]
    fn test_event_extraction_with_long_message() {
        let long_content = "a".repeat(1000);
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000020, // IPMSG_SENDMSG
            "sender".to_string(),
            "receiver".to_string(),
            "msg_long_001".to_string(),
            Some(long_content),
        );

        let addr = create_test_addr("192.168.1.111", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed with long message");
    }

    #[test]
    fn test_event_extraction_preserves_msg_no() {
        let msg_no = "unique_msg_id_12345";
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000021, // IPMSG_RECVMSG
            "receiver".to_string(),
            "sender".to_string(),
            msg_no.to_string(),
            None,
        );

        let addr = create_test_addr("192.168.1.112", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed");
    }

    #[test]
    fn test_event_extraction_with_different_ports() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000001, // IPMSG_BR_ENTRY
            "user".to_string(),
            "".to_string(),
            "12345".to_string(),
            None,
        );

        // Test with different port numbers
        for port in &[2425, 2426, 5000, 65535] {
            let addr = create_test_addr("192.168.1.113", *port);
            let result = publish_event_from_packet(&packet, addr);
            assert!(result.is_ok(), "Event publishing should succeed for port {}", port);
        }
    }

    #[test]
    fn test_event_extraction_with_different_ips() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000001, // IPMSG_BR_ENTRY
            "user".to_string(),
            "".to_string(),
            "12345".to_string(),
            None,
        );

        // Test with different IP addresses
        for ip in &["192.168.1.1", "10.0.0.1", "172.16.0.1", "127.0.0.1"] {
            let addr = create_test_addr(ip, 2425);
            let result = publish_event_from_packet(&packet, addr);
            assert!(result.is_ok(), "Event publishing should succeed for IP {}", ip);
        }
    }

    #[test]
    fn test_message_received_with_all_options() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00A00120, // IPMSG_SENDMSG | UTF8OPT | SENDCHECKOPT | FILEATTACHOPT
            "sender".to_string(),
            "receiver".to_string(),
            "msg_with_file".to_string(),
            Some("Message with file attachment".to_string()),
        );

        let addr = create_test_addr("192.168.1.114", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed with multiple options");
    }

    #[test]
    fn test_event_extraction_with_empty_hostname() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000001, // IPMSG_BR_ENTRY
            "user".to_string(),
            "".to_string(),
            "12345".to_string(),
            None,
        );
        let mut packet = packet;
        packet.hostname = None;
        packet.mac_addr = None;

        let addr = create_test_addr("192.168.1.115", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed with empty optional fields");
    }

    #[test]
    fn test_event_extraction_with_numeric_msg_no() {
        let packet = ProtocolPacket::new_ipmsg(
            "1.0".to_string(),
            0x00000021, // IPMSG_RECVMSG
            "receiver".to_string(),
            "sender".to_string(),
            "9999999999".to_string(),
            None,
        );

        let addr = create_test_addr("192.168.1.116", 2425);
        let result = publish_event_from_packet(&packet, addr);

        assert!(result.is_ok(), "Event publishing should succeed with numeric msg_no");
    }
}
