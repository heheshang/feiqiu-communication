// src-tauri/src/network/feiq/model.rs
//
//! 飞秋协议数据包模型

use serde::{Deserialize, Serialize};

// 导入常量
use crate::network::feiq::constants::{
    IPMSG_UTF8OPT, IPMSG_SENDCHECKOPT, IPMSG_FILEATTACHOPT,
};

/// 飞秋协议数据包
///
/// 协议格式: `版本号:命令字:发送者信息:接收者信息:消息编号:附加信息`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeiqPacket {
    /// 协议版本（通常为 "1.0"）
    pub version: String,

    /// 命令字（包含基础命令和选项标志）
    pub command: u32,

    /// 发送者信息（格式: 用户名@机器名/IP:端口|MAC地址）
    pub sender: String,

    /// 接收者信息（可选，广播时为空）
    pub receiver: String,

    /// 消息编号（用于去重和确认）
    pub msg_no: String,

    /// 附加信息（消息内容/文件信息等）
    pub extension: Option<String>,

    /// 发送者 IP（由 UDP 接收器填充）
    #[serde(skip)]
    pub ip: String,
}

impl Default for FeiqPacket {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            command: 0,
            sender: String::new(),
            receiver: String::new(),
            msg_no: String::new(),
            extension: None,
            ip: String::new(),
        }
    }
}

impl FeiqPacket {
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
}
