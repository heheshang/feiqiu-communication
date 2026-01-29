// src-tauri/src/network/feiq/packer.rs
//
/// 飞秋协议封装器
use crate::network::feiq::{
    constants::*,
    model::{FeiqPacket, FileAttachment},
};
use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前用户信息
///
/// 返回格式: (username, hostname, ip_address, port)
fn get_system_user_info() -> (String, String, String, String) {
    // 获取用户名
    let username = std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "User".to_string());

    // 获取主机名
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "PC-001".to_string());

    // 获取本地 IP 地址
    let ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "192.168.1.100".to_string());

    // 使用默认端口
    let port = "2425";

    (username, hostname, ip, port.to_string())
}

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

    /// 创建已读应答包 (ANSREADMSG)
    #[allow(dead_code)]
    pub fn make_ansread_packet(msg_no: &str) -> Self {
        Self::make_packet(IPMSG_ANSREADMSG, Some(msg_no.to_string()))
    }

    // ============================================================
    // 文件传输相关数据包
    // ============================================================

    /// 创建文件附件消息包 (SENDMSG | FILEATTACHOPT)
    #[allow(dead_code)]
    pub fn make_file_attach_packet(files: &[FileAttachment], receiver: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time should be after Unix epoch")
            .as_secs();

        // 构建文件附件头: 多个文件用 \x07 分隔
        let file_headers: Vec<String> = files.iter().map(|f| f.to_ipmsg_header()).collect();
        let extension = Some(file_headers.join("\x07"));

        let (username, hostname, ip, port) = get_system_user_info();
        let sender = format!("{}@{}/{}:{}", username, hostname, ip, port);

        FeiqPacket {
            version: "1.0".to_string(),
            command: IPMSG_SENDMSG | IPMSG_UTF8OPT | IPMSG_FILEATTACHOPT,
            sender,
            receiver: receiver.to_string(),
            msg_no: timestamp.to_string(),
            extension,
            ip,
            ..Default::default()
        }
    }

    /// 创建文件数据请求包 (GETFILEDATA)
    #[allow(dead_code)]
    pub fn make_get_file_data_packet(packet_no: &str, file_id: u64, offset: u64) -> Self {
        let extension = Some(format!("{}:{}:{}", packet_no, file_id, offset));
        Self::make_packet(IPMSG_GETFILEDATA, extension)
    }

    /// 创建文件释放包 (RELEASEFILES)
    #[allow(dead_code)]
    pub fn make_release_files_packet(packet_no: &str) -> Self {
        Self::make_packet(IPMSG_RELEASEFILES, Some(packet_no.to_string()))
    }

    /// 创建基础数据包 (IPMsg 格式)
    #[allow(dead_code)]
    fn make_packet(command: u32, extension: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time should be after Unix epoch")
            .as_secs();

        let (username, hostname, ip, port) = get_system_user_info();
        let sender = format!("{}@{}/{}:{}", username, hostname, ip, port);

        FeiqPacket {
            version: "1.0".to_string(),
            command,
            sender,
            receiver: String::new(),
            msg_no: timestamp.to_string(),
            extension,
            ip,
            ..Default::default()
        }
    }

    /// 序列化为字符串
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let ext = self.extension.as_deref().unwrap_or("");
        format!(
            "{}:{}:{}:{}:{}:{}",
            self.version, self.command, self.sender, self.receiver, self.msg_no, ext
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
