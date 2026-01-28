// src-tauri/src/network/feiq/packer.rs
//
/// 飞秋协议封装器

use crate::network::feiq::{
    model::FeiqPacket,
    constants::*,
};
use std::time::{SystemTime, UNIX_EPOCH};

impl FeiqPacket {
    /// 创建在线广播包 (BR_ENTRY)
    #[allow(dead_code)]
    pub fn make_entry_packet() -> Self {
        Self::make_packet(IPMSG_BR_ENTRY, None)
    }

    /// 创建在线响应包 (ANSENTRY)
    #[allow(dead_code)]
    pub fn make_ansentry_packet() -> Self {
        Self::make_packet(IPMSG_ANSENTRY, None)
    }

    /// 创建离线广播包 (BR_EXIT)
    #[allow(dead_code)]
    pub fn make_exit_packet() -> Self {
        Self::make_packet(IPMSG_BR_EXIT, None)
    }

    /// 创建消息包 (SENDMSG)
    #[allow(dead_code)]
    pub fn make_message_packet(content: &str, need_check: bool) -> Self {
        let mut command = IPMSG_SENDMSG | IPMSG_UTF8OPT;
        if need_check {
            command |= IPMSG_SENDCHECKOPT;
        }
        Self::make_packet(command, Some(content.to_string()))
    }

    /// 创建接收确认包 (RECVMSG)
    #[allow(dead_code)]
    pub fn make_recv_packet(msg_no: &str) -> Self {
        Self::make_packet(IPMSG_RECVMSG, Some(msg_no.to_string()))
    }

    /// 创建已读回执包 (READMSG)
    #[allow(dead_code)]
    pub fn make_read_packet(msg_no: &str) -> Self {
        Self::make_packet(IPMSG_READMSG, Some(msg_no.to_string()))
    }

    /// 创建基础数据包 (IPMsg 格式)
    #[allow(dead_code)]
    fn make_packet(command: u32, extension: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // TODO: 获取真实的用户名和机器名
        let sender = format!(
            "{}@{}/{}:{}",
            "User",           // 用户名
            "PC-001",         // 机器名
            "192.168.1.100",  // IP
            "2425"            // 端口
        );

        FeiqPacket {
            version: "1.0".to_string(),
            command,
            sender,
            receiver: String::new(),
            msg_no: timestamp.to_string(),
            extension,
            ip: String::new(),
            ..Default::default()
        }
    }

    /// 序列化为字符串
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let ext = self.extension.as_deref().unwrap_or("");
        format!(
            "{}:{}:{}:{}:{}:{}",
            self.version,
            self.command,
            self.sender,
            self.receiver,
            self.msg_no,
            ext
        )
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_entry_packet() {
        let packet = FeiqPacket::make_entry_packet();
        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.base_command(), IPMSG_BR_ENTRY);
    }

    #[test]
    fn test_make_message_packet() {
        let packet = FeiqPacket::make_message_packet("Hello", true);
        assert_eq!(packet.base_command(), IPMSG_SENDMSG);
        assert!(packet.has_option(IPMSG_UTF8OPT));
        assert!(packet.has_option(IPMSG_SENDCHECKOPT));
        assert_eq!(packet.extension, Some("Hello".to_string()));
    }

    #[test]
    fn test_to_string() {
        let packet = FeiqPacket {
            version: "1.0".to_string(),
            command: 32,
            sender: "sender".to_string(),
            receiver: "receiver".to_string(),
            msg_no: "12345".to_string(),
            extension: Some("Hello".to_string()),
            ip: String::new(),
            ..Default::default()
        };

        let s = packet.to_string();
        assert_eq!(s, "1.0:32:sender:receiver:12345:Hello");
    }
}
