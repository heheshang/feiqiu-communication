// src-tauri/src/network/feiq/parser.rs
//
/// 飞秋协议解析器（简化版，暂不使用 combine）
/// TODO: Phase 2 时替换为 combine 解析器组合子

use crate::network::feiq::model::FeiqPacket;

/// 解析错误类型
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid packet format: {0}")]
    InvalidFormat(String),

    #[error("Invalid command number: {0}")]
    InvalidCommand(String),

    #[allow(dead_code)]
    #[error("Missing required field")]
    MissingField,
}

/// 从字符串解析飞秋数据包
///
/// 协议格式: `版本号:命令字:发送者:接收者:消息编号:附加信息`
///
/// # 示例
///
/// ```rust
/// use crate::network::feiq::parser::parse_feiq_packet;
///
/// let input = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
/// let packet = parse_feiq_packet(input).unwrap();
/// assert_eq!(packet.command, 1); // BR_ENTRY
/// ```
pub fn parse_feiq_packet(s: &str) -> Result<FeiqPacket, ParseError> {
    let parts: Vec<&str> = s.split(':').collect();

    if parts.len() < 6 {
        return Err(ParseError::InvalidFormat(
            "Packet must have at least 6 fields".to_string()
        ));
    }

    let version = parts[0].to_string();
    let command = parts[1]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidCommand(parts[1].to_string()))?;
    let sender = parts[2].to_string();
    let receiver = if parts.len() > 3 { parts[3].to_string() } else { String::new() };
    let msg_no = parts[4].to_string();

    // 附加信息（可能包含冒号，所以需要特殊处理）
    let extension = if parts.len() > 6 {
        Some(parts[6..].join(":"))
    } else if parts.len() > 5 && !parts[5].is_empty() {
        Some(parts[5].to_string())
    } else {
        None
    };

    Ok(FeiqPacket {
        version,
        command,
        sender,
        receiver,
        msg_no,
        extension,
        ip: String::new(),
    })
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry_packet() {
        let input = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, 1);
        assert_eq!(packet.sender, "admin@PC-001/192.168.1.100");
        assert_eq!(packet.receiver, "2425|AA:BB:CC:DD:EE:FF");
        assert_eq!(packet.msg_no, "12345");
        assert_eq!(packet.extension, None);
    }

    #[test]
    fn test_parse_sendmsg_packet() {
        let input = "1.0:32:sender:host:receiver:12345:Hello World";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.command, 32);
        assert_eq!(packet.sender, "sender");
        assert_eq!(packet.receiver, "host");
        assert_eq!(packet.msg_no, "12345");
        assert_eq!(packet.extension, Some("Hello World".to_string()));
    }

    #[test]
    fn test_parse_with_colons_in_extension() {
        let input = "1.0:20:sender:host:receiver:12345:Message:with:colons";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.extension, Some("Message:with:colons".to_string()));
    }
}
