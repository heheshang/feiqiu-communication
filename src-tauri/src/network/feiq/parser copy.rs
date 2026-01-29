// src-tauri/src/network/feiq/parser.rs
//
/// 飞秋协议解析器 (使用 combine)
use crate::network::feiq::model::{ProtocolPacket, ProtocolType};
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

/// 解析飞秋数据包（自动检测协议类型）
///
/// # 支持的协议格式
///
/// ## IPMsg 格式
/// ```text
/// 版本号:命令字:发送者:接收者:消息编号:附加信息
/// ```
///
/// ## FeiQ 格式
/// ```text
/// 版本号#长度#MAC地址#端口#标志1#标志2#命令#类型:时间戳:包ID:主机名:用户ID:内容
/// ```
///
/// # 示例
///
/// ```rust
/// use crate::network::feiq::parser::parse_feiq_packet;
///
/// // IPMsg 格式
/// let ipmsg = "1.0:32:sender:host:receiver:6291459:Hello World";
/// let packet = parse_feiq_packet(ipmsg).unwrap();
///
/// // FeiQ 格式
/// let feiq = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:SHIKUN-SH:6291459:ssk";
/// let packet = parse_feiq_packet(feiq).unwrap();
/// ```
pub fn parse_feiq_packet(s: &str) -> Result<ProtocolPacket, ParseError> {
    let protocol_type = ProtocolPacket::detect_protocol(s);

    match protocol_type {
        ProtocolType::FeiQ => parse_feiq_packet_feiq(s),
        ProtocolType::IPMsg => parse_feiq_packet_ipmsg(s),
    }
}

/// 解析 FeiQ 格式数据包
///
/// 格式: `版本号#长度#MAC地址#端口#标志1#标志2#命令#类型:时间戳:包ID:主机名:用户ID:内容`
fn parse_feiq_packet_feiq(s: &str) -> Result<ProtocolPacket, ParseError> {
    // 分割头部和数据部分
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidFormat(
            "FeiQ packet must have header and data sections separated by ':'".to_string(),
        ));
    }

    let header = parts[0];
    let data = parts[1];

    // 解析头部 (以 # 分隔)
    let header_fields: Vec<&str> = header.split('#').collect();
    if header_fields.len() < 8 {
        return Err(ParseError::InvalidFormat(format!(
            "FeiQ header must have at least 8 fields, found {}",
            header_fields.len()
        )));
    }

    let version = header_fields[0].to_string();
    let _length = header_fields[1]; // 可以用于验证
    let mac_addr = header_fields[2].to_string();

    // 端口号是十六进制
    let port = u16::from_str_radix(header_fields[3], 16)
        .or_else(|_| u16::from_str_radix(header_fields[3], 10))
        .unwrap_or(2425);

    let _flag1 = header_fields[4]; // 保留字段
    let _flag2 = header_fields[5]; // 保留字段

    // 命令字
    let command = u32::from_str_radix(header_fields[6], 16)
        .or_else(|_| u32::from_str_radix(header_fields[6], 10))
        .map_err(|_| ParseError::InvalidCommand(header_fields[6].to_string()))?;

    // 消息类型
    let msg_type = u32::from_str_radix(header_fields[7], 16)
        .or_else(|_| u32::from_str_radix(header_fields[7], 10))
        .unwrap_or(0);

    // 解析数据部分 (以 : 分隔)
    let data_fields: Vec<&str> = data.split(':').collect();
    if data_fields.len() < 5 {
        return Err(ParseError::InvalidFormat(format!(
            "FeiQ data must have at least 5 fields, found {}",
            data_fields.len()
        )));
    }

    let timestamp = data_fields[0]
        .parse::<u64>()
        .map_err(|_| ParseError::ParseError("Invalid timestamp".to_string()))?;

    let msg_no = data_fields[1].to_string();
    let hostname = data_fields[2].to_string();
    let user_id = data_fields[3].to_string();

    // 剩余部分合并为扩展信息
    let extension = if data_fields.len() > 5 {
        Some(data_fields[4..].join(":"))
    } else if !data_fields[4].is_empty() {
        Some(data_fields[4].to_string())
    } else {
        None
    };

    Ok(ProtocolPacket {
        protocol_type: ProtocolType::FeiQ,
        version,
        command,
        msg_type: Some(msg_type),
        sender: format!("{}@{}", hostname, mac_addr),
        hostname: Some(hostname),
        mac_addr: Some(mac_addr),
        receiver: String::new(),
        msg_no,
        timestamp: Some(timestamp),
        user_id: Some(user_id),
        extension,
        ip: String::new(),
        port: Some(port),
    })
}

/// 解析 IPMsg 格式数据包
///
/// 格式: `版本号:命令字:发送者:接收者:消息编号:附加信息`
///
/// 注意: 发送者字段可能包含 IP:port，如 "user@host/192.168.1.1:2425"
/// 当接收者为空时，格式为: version:command:sender:receiver:msg_no:extension
fn parse_feiq_packet_ipmsg(s: &str) -> Result<ProtocolPacket, ParseError> {
    // 找到所有冒号的位置
    let mut colon_positions: Vec<usize> = vec![];
    for (i, c) in s.char_indices() {
        if c == ':' {
            colon_positions.push(i);
        }
    }

    if colon_positions.len() < 5 {
        return Err(ParseError::InvalidFormat(format!(
            "IPMsg packet must have at least 5 colons separating fields, found {}",
            colon_positions.len()
        )));
    }

    // 提取前 5 个基本字段
    let version = s[..colon_positions[0]].to_string();
    let command = u32::from_str_radix(&s[colon_positions[0] + 1..colon_positions[1]], 10)
        .or_else(|_| u32::from_str_radix(&s[colon_positions[0] + 1..colon_positions[1]], 16))
        .unwrap_or(0);

    // 发送者字段 (field 2)
    let sender = s[colon_positions[1] + 1..colon_positions[2]].to_string();

    // 接收者字段 (field 3)
    let receiver = s[colon_positions[2] + 1..colon_positions[3]].to_string();

    // 消息编号字段 (field 4)
    let msg_no = s[colon_positions[3] + 1..colon_positions[4]].to_string();

    // 扩展字段 (field 5+) - 第 5 个冒号之后的所有内容
    let extension = if colon_positions[4] + 1 < s.len() {
        let ext = &s[colon_positions[4] + 1..];
        if ext.is_empty() {
            None
        } else {
            Some(ext.to_string())
        }
    } else {
        None
    };

    Ok(ProtocolPacket {
        protocol_type: ProtocolType::IPMsg,
        version,
        command,
        msg_type: None,
        sender,
        hostname: None,
        mac_addr: None,
        receiver,
        msg_no,
        timestamp: None,
        user_id: None,
        extension,
        ip: String::new(),
        port: None,
    })
}

/// 从字节数组解析飞秋数据包
///
/// 用于处理从 UDP socket 接收的原始字节数据
#[allow(dead_code)]
pub fn parse_feiq_packet_bytes(bytes: &[u8]) -> Result<ProtocolPacket, ParseError> {
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

    // ==================== IPMsg 格式测试 ====================

    #[test]
    fn test_parse_ipmsg_sendmsg() {
        // Format: version:command:sender:receiver:msg_no:extension
        let input = "1.0:32:sender:host:12345:Hello World";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.protocol_type, ProtocolType::IPMsg);
        assert_eq!(packet.command, IPMSG_SENDMSG);
        assert_eq!(packet.sender, "sender");
        assert_eq!(packet.receiver, "host");
        assert_eq!(packet.msg_no, "12345");
        assert_eq!(packet.extension, Some("Hello World".to_string()));
    }

    #[test]
    fn test_parse_ipmsg_with_colons_in_extension() {
        let input = "1.0:20:sender:host:12345:Message:with:colons";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.extension, Some("Message:with:colons".to_string()));
    }

    #[test]
    fn test_parse_ipmsg_empty_extension() {
        let input = "1.0:1:sender:host:12345:";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.extension, None);
    }

    #[test]
    fn test_parse_ipmsg_too_short() {
        let input = "1.0:1:sender";
        let result = parse_feiq_packet(input);

        assert!(result.is_err());
    }

    // ==================== FeiQ 格式测试 ====================

    #[test]
    fn test_parse_feiq_basic() {
        let input = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:SHIKUN-SH:6291459:ssk";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.protocol_type, ProtocolType::FeiQ);
        assert_eq!(packet.version, "1_lbt6_0");
        assert_eq!(packet.command, 0x4001);
        assert_eq!(packet.msg_type, Some(9));
        assert_eq!(packet.mac_addr, Some("5C60BA7361C6".to_string()));
        assert_eq!(packet.hostname, Some("SHIKUN-SH".to_string()));
        assert_eq!(packet.timestamp, Some(1765442982));
        assert_eq!(packet.user_id, Some("6291459".to_string()));
        assert_eq!(packet.msg_no, "T0220165");
        assert_eq!(packet.extension, Some("ssk".to_string()));
    }

    #[test]
    fn test_parse_feiq_with_port() {
        let input = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:HOSTNAME:12345:Hello";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.protocol_type, ProtocolType::FeiQ);
        assert_eq!(packet.port, Some(6468)); // 0x1944 = 6468 decimal
        assert_eq!(packet.extension, Some("Hello".to_string()));
    }

    #[test]
    fn test_detect_protocol_ipmsg() {
        assert_eq!(
            ProtocolPacket::detect_protocol("1.0:32:sender:host:receiver:12345:Hello"),
            ProtocolType::IPMsg
        );
    }

    #[test]
    fn test_detect_protocol_feiq() {
        assert_eq!(
            ProtocolPacket::detect_protocol(
                "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:HOST:123:msg"
            ),
            ProtocolType::FeiQ
        );
    }

    #[test]
    fn test_parse_ipmsg_br_entry() {
        // IPMsg format: sender field includes port, receiver can be empty
        let input = "1.0:1:user@PC-001.port2425::12345:";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.protocol_type, ProtocolType::IPMsg);
        assert_eq!(packet.command, IPMSG_BR_ENTRY);
        assert_eq!(packet.sender, "user@PC-001.port2425");
        assert_eq!(packet.receiver, "");
    }
}
