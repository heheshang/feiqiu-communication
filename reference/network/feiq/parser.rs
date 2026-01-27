// src/network/feiq/parser.rs
///
/// 飞秋协议解析器（使用 combine 解析器组合子）
///
/// 参考: langzime/ipmsg-rs (src/util.rs)
/// 设计模式: 解析器组合子 (Parser Combinator)
///

use combine::{combine_parser_impl, many, many1, token, satisfy, digit, Parser, Stream};
use crate::network::feiq::model::FeiqPacket;

// ============================================================
// 主解析器
// ============================================================

/// 飞秋协议数据包解析器
///
/// 协议格式:
/// ```text
/// 版本号:命令字:发送者信息:接收者信息:消息编号:附加信息
/// ```
///
/// # 示例
///
/// ```rust
/// use crate::network::feiq::parser::feiq_packet_parser;
///
/// let input = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
/// let result = feiq_packet_parser().parse(input);
/// assert!(result.is_ok());
/// ```
pub fn feiq_packet_parser<Input>() -> impl Parser<Input, Output = FeiqPacket>
where
    Input: Stream<Token = char>,
{
    (
        // 1. 版本号 (1.0)
        many1(satisfy(|c| c != ':')),
        token(':'),

        // 2. 命令字 (数字)
        many1(digit()),
        token(':'),

        // 3. 发送者信息 (可能包含冒号)
        many1(satisfy(|c| c != ':')),
        token(':'),

        // 4. 接收者信息 (可能为空)
        many(satisfy(|c| c != ':')),
        token(':'),

        // 5. 消息编号 (数字)
        many1(digit()),
        token(':'),

        // 6. 附加信息 (可选，可能为空)
        many(satisfy(|c| true)),
    )
        .map(
            |(ver, _, cmd_str, _, sender, _, receiver, _, msg_no_str, _, extension)| {
                let version: String = ver.into_iter().collect();

                // 解析命令字
                let command: u32 = cmd_str
                    .into_iter()
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);

                // 解析发送者信息
                let sender: String = sender.into_iter().collect();

                // 解析接收者信息
                let receiver: String = if receiver.is_empty() {
                    String::new()
                } else {
                    receiver.into_iter().collect()
                };

                // 解析消息编号
                let msg_no: String = msg_no_str.into_iter().collect();

                // 解析附加信息
                let ext: Option<String> = if extension.is_empty() {
                    None
                } else {
                    Some(extension.into_iter().collect())
                };

                FeiqPacket {
                    version,
                    command,
                    sender,
                    receiver,
                    msg_no,
                    extension: ext,
                    ip: String::new(), // 由外部填充
                }
            },
        )
}

// ============================================================
// 辅助解析器
// ============================================================

/// 解析发送者信息
///
/// 格式: `用户名@机器名/IP:端口|MAC地址`
pub fn sender_info_parser<Input>() -> impl Parser<Input, Output = SenderInfo>
where
    Input: Stream<Token = char>,
{
    (
        // 用户名
        many1(satisfy(|c| c != '@')),
        token('@'),

        // 机器名
        many1(satisfy(|c| c != '/')),
        token('/'),

        // IP:端口
        many1(satisfy(|c| c != '|')),
        token('|'),

        // MAC 地址
        many1(satisfy(|c| true)),
    )
        .map(
            |(username, _, hostname, _, ip_port, _, mac)| {
                SenderInfo {
                    username: username.into_iter().collect(),
                    hostname: hostname.into_iter().collect(),
                    ip_port: ip_port.into_iter().collect(),
                    mac: mac.into_iter().collect(),
                }
            },
        )
}

/// 发送者信息结构
#[derive(Debug, Clone, PartialEq)]
pub struct SenderInfo {
    pub username: String,
    pub hostname: String,
    pub ip_port: String,
    pub mac: String,
}

// ============================================================
// 反序列化器（从字符串解析）
// ============================================================

impl FeiqPacket {
    /// 从字符串解析数据包
    ///
    /// # 示例
    ///
    /// ```rust
    /// use crate::network::feiq::model::FeiqPacket;
    ///
    /// let packet_str = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
    /// let packet = FeiqPacket::from_str(packet_str);
    /// assert!(packet.is_ok());
    /// ```
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        match feiq_packet_parser().parse(s) {
            Ok((packet, remaining)) => {
                if !remaining.is_empty() {
                    return Err(ParseError::TrailingData(remaining.to_string()));
                }
                Ok(packet)
            }
            Err(e) => Err(ParseError::ParserError(e.to_string())),
        }
    }
}

/// 解析错误类型
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Parser error: {0}")]
    ParserError(String),

    #[error("Trailing data after parsing: {0}")]
    TrailingData(String),

    #[error("Invalid command number: {0}")]
    InvalidCommand(String),

    #[error("Invalid message number: {0}")]
    InvalidMessageNumber(String),
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
        let result = feiq_packet_parser().parse(input);

        assert!(result.is_ok());
        let (packet, remaining) = result.unwrap();

        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, 1);
        assert_eq!(packet.sender, "admin@PC-001/192.168.1.100");
        assert_eq!(packet.receiver, "2425|AA:BB:CC:DD:EE:FF");
        assert_eq!(packet.msg_no, "12345");
        assert_eq!(packet.extension, None);
        assert!(remaining.is_empty());
    }

    #[test]
    fn test_parse_sendmsg_packet() {
        let input = "1.0:32:sender:host:receiver:12345:Hello World";
        let result = feiq_packet_parser().parse(input);

        assert!(result.is_ok());
        let (packet, _) = result.unwrap();

        assert_eq!(packet.command, 32);
        assert_eq!(packet.sender, "sender");
        assert_eq!(packet.receiver, "receiver");
        assert_eq!(packet.msg_no, "12345");
        assert_eq!(packet.extension, Some("Hello World".to_string()));
    }

    #[test]
    fn test_parse_with_extension() {
        let input = "1.0:20:sender:host:receiver:12345:This is a message with:colons";
        let result = feiq_packet_parser().parse(input);

        assert!(result.is_ok());
        let (packet, _) = result.unwrap();

        assert_eq!(packet.extension, Some("This is a message with:colons".to_string()));
    }

    #[test]
    fn test_parse_empty_receiver() {
        let input = "1.0:1:sender::12345:";
        let result = feiq_packet_parser().parse(input);

        assert!(result.is_ok());
        let (packet, _) = result.unwrap();

        assert_eq!(packet.receiver, "");
    }

    #[test]
    fn test_parse_sender_info() {
        let input = "admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF";
        let result = sender_info_parser().parse(input);

        assert!(result.is_ok());
        let (info, _) = result.unwrap();

        assert_eq!(info.username, "admin");
        assert_eq!(info.hostname, "PC-001");
        assert_eq!(info.ip_port, "192.168.1.100:2425");
        assert_eq!(info.mac, "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn test_from_str() {
        let packet_str = "1.0:1:sender:host:receiver:12345:";
        let packet = FeiqPacket::from_str(packet_str);

        assert!(packet.is_ok());
        let packet = packet.unwrap();

        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, 1);
    }

    #[test]
    fn test_from_str_with_trailing() {
        let packet_str = "1.0:1:sender:host:receiver:12345:extra data";
        let packet = FeiqPacket::from_str(packet_str);

        assert!(matches!(packet, Err(ParseError::TrailingData(_))));
    }

    #[test]
    fn test_empty_command() {
        let input = "1.0::sender:host:receiver:12345:";
        let result = feiq_packet_parser().parse(input);

        // 命令字为空时应该解析失败
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_message_no() {
        let input = "1.0:1:sender:host:receiver::";
        let result = feiq_packet_parser().parse(input);

        // 消息编号为空时应该解析失败
        assert!(result.is_err());
    }

    #[test]
    fn test_real_world_example_1() {
        // 在线广播包
        let input = "1.0:1:admin@DESKTOP-ABC/192.168.1.100:2425|00:11:22:33:44:55::1706064000:";
        let result = feiq_packet_parser().parse(input);

        assert!(result.is_ok());
        let (packet, _) = result.unwrap();
        assert_eq!(packet.command, 1); // BR_ENTRY
    }

    #[test]
    fn test_real_world_example_2() {
        // 发送消息包
        let input = "1.0:32:alice@DESKTOP-ABC/192.168.1.100:2425|00:11:22:33:44:55:bob@DESKTOP-XYZ/192.168.1.101:2425|66:77:88:99:aa:bb:cc:1706064010:Hello!";
        let result = feiq_packet_parser().parse(input);

        assert!(result.is_ok());
        let (packet, _) = result.unwrap();
        assert_eq!(packet.command, 32); // SENDMSG
        assert_eq!(packet.extension, Some("Hello!".to_string()));
    }

    #[test]
    fn test_file_attachment_format() {
        // 文件附件格式 (简化版)
        let input = "1.0:83200020:sender:host:receiver:12345:\u{7}1:file.txt:1024:0:1706064010=0:1706064020=0:";
        let result = feiq_packet_parser().parse(input);

        assert!(result.is_ok());
        let (packet, _) = result.unwrap();
        assert_eq!(packet.command, 0x83200020); // SENDMSG | FILEATTACHOPT | UTF8OPT
        assert!(packet.extension.unwrap().starts_with('\u{7}'));
    }
}
