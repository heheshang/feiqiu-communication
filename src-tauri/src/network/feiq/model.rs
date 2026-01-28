// src-tauri/src/network/feiq/model.rs
//
//! 飞秋协议数据包模型

use serde::{Deserialize, Serialize};

// 导入常量
use crate::network::feiq::constants::{
    IPMSG_UTF8OPT, IPMSG_SENDCHECKOPT, IPMSG_FILEATTACHOPT,
};

/// 协议类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum ProtocolType {
    /// IPMsg 协议 (标准格式)
    #[default]
    IPMsg,
    /// 飞秋协议 (扩展格式)
    FeiQ,
}

/// 飞秋协议数据包（支持 IPMsg 和 FeiQ 两种格式）
///
/// IPMsg 格式: `版本号:命令字:发送者信息:接收者信息:消息编号:附加信息`
///
/// FeiQ 格式: `版本号#长度#MAC地址#端口#标志1#标志2#命令#类型:时间戳:包ID:主机名:用户ID:内容`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeiqPacket {
    /// 协议类型
    #[serde(skip)]
    pub protocol_type: ProtocolType,

    /// 协议版本（IPMsg 通常为 "1.0"，FeiQ 为 "1_lbt6_0" 等）
    pub version: String,

    /// 命令字（包含基础命令和选项标志）
    pub command: u32,

    /// 消息类型（FeiQ 专用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<u32>,

    /// 发送者信息（格式: 用户名@机器名/IP:端口|MAC地址）
    pub sender: String,

    /// 发送者主机名（FeiQ 专用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// 发送者 MAC 地址（FeiQ 专用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_addr: Option<String>,

    /// 接收者信息（可选，广播时为空）
    pub receiver: String,

    /// 消息编号（用于去重和确认）
    pub msg_no: String,

    /// 时间戳（FeiQ 专用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

    /// 用户 ID（FeiQ 专用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// 附加信息（消息内容/文件信息等）
    pub extension: Option<String>,

    /// 发送者 IP（由 UDP 接收器填充）
    #[serde(skip)]
    pub ip: String,

    /// 发送者端口（FeiQ 专用）
    #[serde(skip)]
    pub port: Option<u16>,
}

impl Default for FeiqPacket {
    fn default() -> Self {
        Self {
            protocol_type: ProtocolType::IPMsg,
            version: "1.0".to_string(),
            command: 0,
            msg_type: None,
            sender: String::new(),
            hostname: None,
            mac_addr: None,
            receiver: String::new(),
            msg_no: String::new(),
            timestamp: None,
            user_id: None,
            extension: None,
            ip: String::new(),
            port: None,
        }
    }
}

impl FeiqPacket {
    /// 从原始数据检测协议类型
    pub fn detect_protocol(data: &str) -> ProtocolType {
        // FeiQ 格式: 包含 # 分隔符
        if data.contains('#') {
            ProtocolType::FeiQ
        } else {
            ProtocolType::IPMsg
        }
    }

    /// 获取基础命令字（去除选项标志）
    #[allow(dead_code)]
    pub fn base_command(&self) -> u32 {
        self.command & 0xFF
    }

    /// 检查是否包含某个选项标志
    #[allow(dead_code)]
    pub fn has_option(&self, flag: u32) -> bool {
        (self.command & flag) != 0
    }

    /// 检查是否为 UTF-8 编码
    #[allow(dead_code)]
    pub fn is_utf8(&self) -> bool {
        self.has_option(IPMSG_UTF8OPT)
    }

    /// 检查是否需要确认
    #[allow(dead_code)]
    pub fn need_check(&self) -> bool {
        self.has_option(IPMSG_SENDCHECKOPT)
    }

    /// 检查是否带文件附件
    #[allow(dead_code)]
    pub fn has_file(&self) -> bool {
        self.has_option(IPMSG_FILEATTACHOPT)
    }

    /// 获取消息编号的数值
    #[allow(dead_code)]
    pub fn msg_no_value(&self) -> u64 {
        self.msg_no.parse().unwrap_or(0)
    }

    /// 创建 IPMsg 格式数据包
    pub fn new_ipmsg(
        version: String,
        command: u32,
        sender: String,
        receiver: String,
        msg_no: String,
        extension: Option<String>,
    ) -> Self {
        Self {
            protocol_type: ProtocolType::IPMsg,
            version,
            command,
            sender,
            receiver,
            msg_no,
            extension,
            ..Default::default()
        }
    }

    /// 创建 FeiQ 格式数据包
    pub fn new_feiq(
        version: String,
        command: u32,
        msg_type: u32,
        hostname: String,
        mac_addr: String,
        port: u16,
        timestamp: u64,
        user_id: String,
        msg_no: String,
        extension: Option<String>,
    ) -> Self {
        Self {
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
            port: Some(port),
            ..Default::default()
        }
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_packet() {
        let packet = FeiqPacket::default();
        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, 0);
        assert_eq!(packet.protocol_type, ProtocolType::IPMsg);
    }

    #[test]
    fn test_base_command() {
        let packet = FeiqPacket {
            command: 0x00800120, // SENDMSG | UTF8OPT | SENDCHECKOPT
            ..Default::default()
        };
        assert_eq!(packet.base_command(), 0x20); // IPMSG_SENDMSG
    }

    #[test]
    fn test_has_option() {
        let packet = FeiqPacket {
            command: 0x00800120,
            ..Default::default()
        };
        assert!(packet.has_option(IPMSG_UTF8OPT));
        assert!(packet.has_option(IPMSG_SENDCHECKOPT));
        assert!(!packet.has_option(IPMSG_FILEATTACHOPT));
    }

    #[test]
    fn test_detect_protocol_ipmsg() {
        let data = "1.0:32:sender:host:receiver:12345:Hello";
        assert_eq!(FeiqPacket::detect_protocol(data), ProtocolType::IPMsg);
    }

    #[test]
    fn test_detect_protocol_feiq() {
        let data = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:SHIKUN-SH:6291459:ssk";
        assert_eq!(FeiqPacket::detect_protocol(data), ProtocolType::FeiQ);
    }

    #[test]
    fn test_new_ipmsg() {
        let packet = FeiqPacket::new_ipmsg(
            "1.0".to_string(),
            32,
            "user@PC/192.168.1.100:2425".to_string(),
            "".to_string(),
            "12345".to_string(),
            Some("Hello".to_string()),
        );
        assert_eq!(packet.protocol_type, ProtocolType::IPMsg);
        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, 32);
    }

    #[test]
    fn test_new_feiq() {
        let packet = FeiqPacket::new_feiq(
            "1_lbt6_0".to_string(),
            0x4001,
            9,
            "SHIKUN-SH".to_string(),
            "5C60BA7361C6".to_string(),
            6452,
            1765442982,
            "6291459".to_string(),
            "T0220165".to_string(),
            Some("ssk".to_string()),
        );
        assert_eq!(packet.protocol_type, ProtocolType::FeiQ);
        assert_eq!(packet.version, "1_lbt6_0");
        assert_eq!(packet.command, 0x4001);
        assert_eq!(packet.msg_type, Some(9));
        assert_eq!(packet.hostname, Some("SHIKUN-SH".to_string()));
        assert_eq!(packet.mac_addr, Some("5C60BA7361C6".to_string()));
    }
}
