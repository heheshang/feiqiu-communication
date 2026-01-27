// src-tauri/src/network/feiq/parser.rs
//
/// 飞秋协议解析器（使用 combine 解析器组合子）

use crate::network::feiq::model::FeiqPacket;
use std::str::Utf8Error;

/// 解析错误类型
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid packet format: {0}")]
    InvalidFormat(String),

    #[error("Invalid command number: {0}")]
    InvalidCommand(String),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Missing required field")]
    MissingField,
}

impl From<ParseError> for String {
    fn from(err: ParseError) -> Self {
        err.to_string()
    }
}

// ============================================================
// Helper Functions
// ============================================================

/// 解析单个字段（直到遇到冒号）
fn parse_field(s: &str) -> Result<(&str, &str), ParseError> {
    match s.find(':') {
        Some(pos) => Ok((&s[..pos], &s[pos + 1..])),
        None => Err(ParseError::InvalidFormat("Missing colon separator".to_string())),
    }
}

/// 解析版本号
fn parse_version(s: &str) -> Result<(String, &str), ParseError> {
    let (version, rest) = parse_field(s)?;
    Ok((version.to_string(), rest))
}

/// 解析命令字
fn parse_command(s: &str) -> Result<(u32, &str), ParseError> {
    let (cmd_str, rest) = parse_field(s)?;
    let command = u32::from_str_radix(cmd_str, 16)
        .or_else(|_| u32::from_str_radix(cmd_str, 10))
        .map_err(|_| ParseError::InvalidCommand(cmd_str.to_string()))?;
    Ok((command, rest))
}

/// 解析发送者信息
fn parse_sender(s: &str) -> Result<(String, &str), ParseError> {
    let (sender, rest) = parse_field(s)?;
    Ok((sender.to_string(), rest))
}

/// 解析接收者信息
fn parse_receiver(s: &str) -> Result<(String, &str), ParseError> {
    let (receiver, rest) = parse_field(s)?;
    Ok((receiver.to_string(), rest))
}

/// 解析消息编号
fn parse_msg_no(s: &str) -> Result<(String, &str), ParseError> {
    let (msg_no, rest) = parse_field(s)?;
    Ok((msg_no.to_string(), rest))
}

/// 解析附加信息（可选）
fn parse_extension(s: &str) -> Result<(Option<String>,), ParseError> {
    if s.is_empty() {
        return Ok((None,));
    }

    // 检查是否以冒号开头
    if s.starts_with(':') {
        let ext = &s[1..];
        Ok((Some(ext.to_string()),))
    } else {
        Ok((Some(s.to_string()),))
    }
}

// ============================================================
// Public API
// ============================================================

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
    let (version, rest) = parse_version(s)?;
    let (command, rest) = parse_command(rest)?;
    let (sender, rest) = parse_sender(rest)?;
    let (receiver, rest) = parse_receiver(rest)?;
    let (msg_no, rest) = parse_msg_no(rest)?;
    let (extension,) = parse_extension(rest)?;

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

/// 从字节数组解析飞秋数据包
///
/// 用于处理从 UDP socket 接收的原始字节数据
#[allow(dead_code)]
pub fn parse_feiq_packet_bytes(bytes: &[u8]) -> Result<FeiqPacket, ParseError> {
    // 转换为 UTF-8 字符串
    let s = std::str::from_utf8(bytes)?;
    parse_feiq_packet(s)
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::feiq::constants::*;

    #[test]
    fn test_parse_entry_packet() {
        let input = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, IPMSG_BR_ENTRY);
        assert_eq!(packet.sender, "admin@PC-001/192.168.1.100");
        assert_eq!(packet.receiver, "2425|AA:BB:CC:DD:EE:FF");
        assert_eq!(packet.msg_no, "12345");
        assert_eq!(packet.extension, None);
    }

    #[test]
    fn test_parse_sendmsg_packet() {
        let input = "1.0:32:sender:host:receiver:12345:Hello World";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.command, IPMSG_SENDMSG);
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

    #[test]
    fn test_parse_with_options() {
        // SENDMSG | UTF8OPT | SENDCHECKOPT = 0x00800020 = 8388648
        let input = "1.0:8388648:sender:host:receiver:12345:Hello";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.command, 0x00800020);
        assert!(packet.has_option(IPMSG_UTF8OPT));
        assert!(packet.has_option(IPMSG_SENDCHECKOPT));
    }

    #[test]
    fn test_parse_empty_extension() {
        let input = "1.0:1:sender:host:receiver:12345:";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.extension, None);
    }

    #[test]
    fn test_parse_invalid_command() {
        let input = "1.0:abc:sender:host:receiver:12345:";
        let result = parse_feiq_packet(input);

        // 应该解析成功但命令为0（因为abc无法解析为数字）
        assert!(result.is_ok());
        let packet = result.unwrap();
        assert_eq!(packet.command, 0);
    }

    #[test]
    fn test_parse_too_short() {
        let input = "1.0:1:sender";
        let result = parse_feiq_packet(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_chinese_characters() {
        let input = "1.0:8388648:张三@PC-001/192.168.1.100:2425::12345:你好世界";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.sender, "张三@PC-001/192.168.1.100");
        assert_eq!(packet.extension, Some("你好世界".to_string()));
    }

    #[test]
    fn test_parse_br_exit() {
        let input = "1.0:2:user@PC/192.168.1.1:2425::12345:";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.command, IPMSG_BR_EXIT);
    }

    #[test]
    fn test_parse_ansentry() {
        let input = "1.0:3:user@PC/192.168.1.1:2425::12345:";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.command, IPMSG_ANSENTRY);
    }

    #[test]
    fn test_parse_multiple_packets() {
        // Test parsing multiple packets from a single string
        let input1 = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
        let input2 = "1.0:32:sender:host:receiver:12345:Hello";

        let packet1 = parse_feiq_packet(input1).unwrap();
        let packet2 = parse_feiq_packet(input2).unwrap();

        assert_eq!(packet1.command, IPMSG_BR_ENTRY);
        assert_eq!(packet2.command, IPMSG_SENDMSG);
        assert_eq!(packet2.extension, Some("Hello".to_string()));
    }
}
