// src-tauri/src/network/feiq/parser.rs
//
/// 飞秋协议解析器 (使用 combine)
///
/// 注意：飞秋协议使用 GBK 编码，而非 UTF-8
use crate::network::feiq::model::{FeiQExtInfo, FeiQPacket};
use crate::network::feiq::utils::{format_mac_addr, timestamp_to_local};
use encoding::DecoderTrap;
use std::str::Utf8Error;

/// GBK 编码器引用 (用于解码飞秋协议消息)
const GBK_ENCODING: encoding::EncodingRef = encoding::all::GBK;

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

/// 解析飞秋数据包（仅支持 FeiQ 协议）
///
/// # FeiQ 协议格式
/// ```text
/// 版本号#长度#MAC地址#端口#标志1#标志2#命令#类型:时间戳:包ID:主机名:用户ID:内容
/// ```
///
/// # 示例
///
/// ```rust
/// use crate::network::feiq::parser::parse_feiq_packet;
///
/// // FeiQ 格式
/// let feiq = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:SHIKUN-SH:6291459:ssk";
/// let packet = parse_feiq_packet(feiq).unwrap();
/// ```
pub fn parse_feiq_packet(s: &str) -> Result<FeiQPacket, ParseError> {
    parse_feiq_packet_feiq(s)
}

/// 解析 FeiQ 格式数据包（详细版本）
///
/// 格式: `版本号#长度#MAC地址#端口#标志1#标志2#命令#类型:时间戳:包ID:主机名:用户ID:内容`
fn parse_feiq_packet_feiq(s: &str) -> Result<FeiQPacket, ParseError> {
    parse_feiq_packet_detail(s)
}

/// 解析飞秋数据包字符串的核心函数（详细版本）
///
/// 输入：飞秋UDP数据包原始字符串（如"1_lbt6_0#128#5C60BA7361C6#..."）
/// 输出：解析后的FeiQPacket结构体（带错误处理）
pub fn parse_feiq_packet_detail(packet_str: &str) -> Result<FeiQPacket, ParseError> {
    // 1. 拆分主字段（#分隔）
    let main_fields: Vec<&str> = packet_str.split('#').collect();
    if main_fields.len() != 8 {
        return Err(ParseError::InvalidFormat(format!(
            "飞秋数据包主字段数量错误，需为8个（当前：{}）",
            main_fields.len()
        )));
    }

    // 2. 解析主字段基础信息
    let pkg_type = main_fields[0].to_string();
    let func_flag = main_fields[1]
        .parse::<u32>()
        .map_err(|e| ParseError::InvalidFormat(format!("功能标识位解析失败：{}", e)))?;
    let mac_addr_raw = main_fields[2].to_string();
    let mac_addr_formatted =
        format_mac_addr(&mac_addr_raw).map_err(|e| ParseError::InvalidFormat(format!("MAC地址格式化失败：{}", e)))?;

    // UDP 端口可能是十进制或十六进制
    let udp_port = if let Ok(port) = main_fields[3].parse::<u16>() {
        port
    } else {
        u16::from_str_radix(main_fields[3], 16).map_err(|_| ParseError::InvalidFormat("UDP端口解析失败".to_string()))?
    };

    let file_transfer_id = main_fields[4]
        .parse::<u32>()
        .map_err(|e| ParseError::InvalidFormat(format!("文件传输ID解析失败：{}", e)))?;
    let extra_flag = main_fields[5]
        .parse::<u32>()
        .map_err(|e| ParseError::InvalidFormat(format!("附加标志位解析失败：{}", e)))?;
    let client_version = main_fields[6]
        .parse::<u32>()
        .map_err(|e| ParseError::InvalidFormat(format!("客户端版本号解析失败：{}", e)))?;

    // 3. 拆分扩展信息段（:分隔）
    let data_section = main_fields[7];
    let ext_fields: Vec<&str> = data_section.split(':').collect();

    // FeiQ 协议的扩展字段数量可变，通常是 6-7 个字段
    // 最后一个字段可能为空（以 : 结尾）
    if ext_fields.len() < 6 {
        return Err(ParseError::InvalidFormat(format!(
            "飞秋扩展字段数量错误，至少需要6个（当前：{}）",
            ext_fields.len()
        )));
    }

    // 4. 解析扩展信息段
    // 注意：第一个字段可能与 header 中的 msg_sub_type 重复
    let msg_sub_type = ext_fields[0]
        .parse::<u8>()
        .map_err(|e| ParseError::InvalidFormat(format!("消息子类型码解析失败：{}", e)))?;

    // 根据字段数量处理不同的格式
    // 标准6字段格式: msg_sub_type:timestamp:unique_id:hostname:nickname:remark
    // 扩展7字段格式: msg_sub_type:counter:timestamp:packet_id:hostname:unique_id:remark
    let (timestamp, unique_id, hostname, nickname, remark) = if ext_fields.len() >= 7 {
        // 7+ 字段格式（新版本 FeiQ）- 格式: msg_sub_type:counter:timestamp:packet_id:hostname:unique_id:remark
        let ts = ext_fields[2]
            .parse::<i64>()
            .map_err(|e| ParseError::InvalidFormat(format!("时间戳解析失败：{}", e)))?;
        let _packet_id = ext_fields[3].to_string(); // 某种包 ID
        let host = ext_fields[4].to_string();
        let uid = ext_fields[5].to_string(); // unique_id
        let nick = String::new(); // 7字段格式通常没有 nickname 字段
        let rem = if ext_fields.len() > 6 {
            ext_fields[6..].join(":")
        } else {
            String::new()
        };
        (ts, uid, host, nick, rem)
    } else {
        // 标准 6 字段格式
        let ts = ext_fields[1]
            .parse::<i64>()
            .map_err(|e| ParseError::InvalidFormat(format!("时间戳解析失败：{}", e)))?;
        let uid = ext_fields[2].to_string();
        let host = ext_fields[3].to_string();
        let nick = ext_fields[4].to_string();
        let rem = if ext_fields.len() > 5 {
            ext_fields[5].to_string()
        } else {
            String::new()
        };
        (ts, uid, host, nick, rem)
    };

    // 转换时间戳为本地时间字符串
    let timestamp_local = timestamp_to_local(timestamp);

    // 5. 封装最终结构体
    Ok(FeiQPacket {
        pkg_type,
        func_flag,
        mac_addr_raw,
        mac_addr_formatted,
        udp_port,
        file_transfer_id,
        extra_flag,
        client_version,
        ext_info: FeiQExtInfo {
            msg_sub_type,
            timestamp,
            timestamp_local,
            unique_id,
            hostname,
            nickname,
            remark,
        },
    })
}

/// 从字节数组解析飞秋数据包
///
/// 用于处理从 UDP socket 接收的原始字节数据
#[allow(dead_code)]
pub fn parse_feiq_packet_bytes(bytes: &[u8]) -> Result<FeiQPacket, ParseError> {
    let s = decode_gbk(bytes)?;
    parse_feiq_packet(&s)
}

/// 使用 GBK 编码解码字节数据
///
/// 飞秋协议使用 GBK 编码，此函数用于正确解码从网络接收的字节数据
///
/// # 参数
/// * `bytes` - 原始字节数据
///
/// # 返回
/// 解码后的字符串
pub fn decode_gbk(bytes: &[u8]) -> Result<String, ParseError> {
    GBK_ENCODING
        .decode(bytes, DecoderTrap::Strict)
        .map_err(|e| ParseError::InvalidFormat(format!("GBK decode error: {}", e)))
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::feiq::constants::*;

    // ==================== FeiQ 格式测试 ====================

    // TODO: ProtocolPacket tests disabled - FeiQ-only migration in progress
    // The parser now returns FeiQPacket directly, not the old ProtocolPacket wrapper
    // These tests need to be rewritten to test FeiQPacket structure directly
    /*
    #[test]
    fn test_parse_feiq_basic() {
        let input = "1_lbt6_0#128#5C60BA7361C6#2425#0#0#4001#9:1765442982:T0170006:SHIKUN-SH:6291459:ssk";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.protocol_type, ProtocolType::FeiQ);
        assert_eq!(packet.version, "1_lbt6_0");
        assert_eq!(packet.command, 4001);
        assert_eq!(packet.msg_type, Some(9));

        // 检查详细信息
        assert!(packet.feiq_detail.is_some());
        let detail = packet.feiq_detail.as_ref().unwrap();
        assert_eq!(detail.pkg_type, "1_lbt6_0");
        assert_eq!(detail.func_flag, 128);
        assert_eq!(detail.mac_addr_formatted, "5C-60-BA-73-61-C6");
        assert_eq!(detail.udp_port, 2425); // decimal port
        assert_eq!(detail.ext_info.msg_sub_type, 9);
        assert_eq!(detail.ext_info.hostname, "SHIKUN-SH");
        assert_eq!(detail.ext_info.nickname, "6291459");
        assert_eq!(detail.ext_info.remark, "ssk");
    }
    */

    #[test]
    fn test_parse_feiq_detail() {
        let input = "1_lbt6_0#128#5C60BA7361C6#2425#0#0#4001#9:1765442982:T0170006:SHIKUN-SH:6291459:ssk";
        let detail = parse_feiq_packet_detail(input).unwrap();

        assert_eq!(detail.pkg_type, "1_lbt6_0");
        assert_eq!(detail.func_flag, 128);
        assert_eq!(detail.mac_addr_formatted, "5C-60-BA-73-61-C6");
        assert_eq!(detail.udp_port, 2425); // decimal port
        assert_eq!(detail.ext_info.timestamp_local, "2025-12-11 16:49:42");
        assert_eq!(detail.ext_info.unique_id, "T0170006");
        assert_eq!(detail.ext_info.hostname, "SHIKUN-SH");
        assert_eq!(detail.ext_info.nickname, "6291459");
        assert_eq!(detail.ext_info.remark, "ssk");
    }

    #[test]
    fn test_parse_feiq_7_field_format() {
        // 实际的飞秋数据包（7字段格式）
        // 格式: msg_sub_type:counter:timestamp:packet_id:hostname:unique_id:remark
        let input = "1_lbt6_0#128#5C60BA7361C6#2425#0#0#16385#9:9:1769669929:T1769669929:shikunsh-n:T0220165:";
        let detail = parse_feiq_packet_detail(input).unwrap();

        assert_eq!(detail.pkg_type, "1_lbt6_0");
        assert_eq!(detail.func_flag, 128);
        assert_eq!(detail.mac_addr_formatted, "5C-60-BA-73-61-C6");
        assert_eq!(detail.udp_port, 2425); // decimal port
        assert_eq!(detail.client_version, 16385); // 0x4001 in decimal
        assert_eq!(detail.ext_info.msg_sub_type, 9);
        assert_eq!(detail.ext_info.timestamp, 1769669929);
        assert_eq!(detail.ext_info.unique_id, "T0220165");
        assert_eq!(detail.ext_info.hostname, "shikunsh-n");
        assert_eq!(detail.ext_info.nickname, ""); // 7字段格式没有 nickname
        assert_eq!(detail.ext_info.remark, "");
    }

    // TODO: Protocol detection test disabled - FeiQ-only migration in progress
    // Protocol detection no longer needed since parser only supports FeiQ format now
    /*
    #[test]
    fn test_detect_protocol_feiq() {
        assert_eq!(
            ProtocolPacket::detect_protocol(
                "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:HOST:123:msg"
            ),
            ProtocolType::FeiQ
        );
    }
    */

    #[test]
    fn test_formated_mac() {
        let input = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:SHIKUN-SH:6291459:ssk";
        let packet = parse_feiq_packet(input).unwrap();

        assert_eq!(packet.formatted_mac(), "5C-60-BA-73-61-C6");
    }

    #[test]
    fn test_local_timestamp() {
        let input = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:SHIKUN-SH:6291459:ssk";
        let packet = parse_feiq_packet(input).unwrap();

        let local_ts = packet.local_timestamp();
        assert!(local_ts.contains("2025"));
    }
}
