// src-tauri/src/network/feiq/packer.rs
//
/// 飞秋协议封装器
use crate::network::feiq::{
    constants::*,
    model::{FeiQExtInfo, FeiQPacket, FileAttachment, ProtocolPacket},
    utils::{format_mac_addr, timestamp_to_local},
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

/// 获取 MAC 地址（简化版本，实际应该从网络接口获取）
fn get_mac_address() -> String {
    // 简化版本：生成一个随机 MAC 地址
    // 实际应用中应该从网络接口获取真实的 MAC 地址
    "5C60BA7361C6".to_string()
}

/// 生成唯一包 ID
fn generate_packet_id() -> String {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    format!("T{:010}", timestamp % 10000000000)
}

impl ProtocolPacket {
    /// 创建在线广播包 (BR_ENTRY)
    #[allow(dead_code)]
    pub fn make_entry_packet() -> ProtocolPacket {
        Self::make_packet(IPMSG_BR_ENTRY, None)
    }

    /// 创建在线响应包 (ANSENTRY)
    #[allow(dead_code)]
    pub fn make_ansentry_packet() -> ProtocolPacket {
        Self::make_packet(IPMSG_ANSENTRY, None)
    }

    /// 创建离线广播包 (BR_EXIT)
    #[allow(dead_code)]
    pub fn make_exit_packet() -> ProtocolPacket {
        Self::make_packet(IPMSG_BR_EXIT, None)
    }

    /// 创建消息包 (SENDMSG)
    #[allow(dead_code)]
    pub fn make_message_packet(content: &str, need_check: bool) -> ProtocolPacket {
        let mut command = IPMSG_SENDMSG | IPMSG_UTF8OPT;
        if need_check {
            command |= IPMSG_SENDCHECKOPT;
        }
        Self::make_packet(command, Some(content.to_string()))
    }

    /// 创建接收确认包 (RECVMSG)
    #[allow(dead_code)]
    pub fn make_recv_packet(msg_no: &str) -> ProtocolPacket {
        Self::make_packet(IPMSG_RECVMSG, Some(msg_no.to_string()))
    }

    /// 创建已读回执包 (READMSG)
    #[allow(dead_code)]
    pub fn make_read_packet(msg_no: &str) -> ProtocolPacket {
        Self::make_packet(IPMSG_READMSG, Some(msg_no.to_string()))
    }

    /// 创建已读应答包 (ANSREADMSG)
    #[allow(dead_code)]
    pub fn make_ansread_packet(msg_no: &str) -> ProtocolPacket {
        Self::make_packet(IPMSG_ANSREADMSG, Some(msg_no.to_string()))
    }

    // ============================================================
    // 文件传输相关数据包
    // ============================================================

    /// 创建文件附件消息包 (SENDMSG | FILEATTACHOPT)
    #[allow(dead_code)]
    pub fn make_file_attach_packet(files: &[FileAttachment], receiver: &str) -> ProtocolPacket {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time should be after Unix epoch")
            .as_secs();

        // 构建文件附件头: 多个文件用 \x07 分隔
        let file_headers: Vec<String> = files.iter().map(|f| f.to_ipmsg_header()).collect();
        let extension = Some(file_headers.join("\x07"));

        let (username, hostname, ip, port) = get_system_user_info();
        let sender = format!("{}@{}/{}:{}", username, hostname, ip, port);

        ProtocolPacket {
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
    pub fn make_get_file_data_packet(packet_no: &str, file_id: u64, offset: u64) -> ProtocolPacket {
        let extension = Some(format!("{}:{}:{}", packet_no, file_id, offset));
        Self::make_packet(IPMSG_GETFILEDATA, extension)
    }

    /// 创建文件释放包 (RELEASEFILES)
    #[allow(dead_code)]
    pub fn make_release_files_packet(packet_no: &str) -> ProtocolPacket {
        Self::make_packet(IPMSG_RELEASEFILES, Some(packet_no.to_string()))
    }

    /// 创建基础数据包 (IPMsg 格式)
    #[allow(dead_code)]
    fn make_packet(command: u32, extension: Option<String>) -> ProtocolPacket {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time should be after Unix epoch")
            .as_secs();

        let (username, hostname, ip, port) = get_system_user_info();
        let sender = format!("{}@{}/{}:{}", username, hostname, ip, port);

        ProtocolPacket {
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
// FeiQPacket 实现（飞秋协议专用）
// ============================================================

impl FeiQPacket {
    // ============================================================
    // FeiQ 格式数据包创建方法
    // ============================================================

    /// 创建 FeiQ 格式的在线广播包
    ///
    /// 格式: 1_lbt6_0#128#MAC#端口#0#0#4001#9:时间戳:包ID:主机名:用户ID:备注
    pub fn make_feiq_entry_packet(nickname: Option<&str>) -> FeiQPacket {
        let (username, hostname, _ip, _port) = get_system_user_info();
        let mac_addr = get_mac_address();
        let mac_formatted = format_mac_addr(&mac_addr).unwrap_or_else(|_| mac_addr.clone());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;

        let packet_id = generate_packet_id();
        let nickname = nickname.unwrap_or(&username).to_string();
        let remark = "".to_string(); // 备注可以为空

        FeiQPacket {
            pkg_type: "1_lbt6_0".to_string(),
            func_flag: 128, // 客户端在线广播
            mac_addr_raw: mac_addr.clone(),
            mac_addr_formatted: mac_formatted,
            udp_port: 2425,
            file_transfer_id: 0,
            extra_flag: 0,
            client_version: 0x4001,
            ext_info: FeiQExtInfo {
                msg_sub_type: 9, // 在线广播
                timestamp,
                timestamp_local: timestamp_to_local(timestamp),
                unique_id: packet_id,
                hostname: hostname.clone(),
                nickname: nickname.clone(),
                remark,
            },
        }
    }

    /// 创建 FeiQ 格式的在线响应包 (ANSENTRY)
    pub fn make_feiq_ansentry_packet(nickname: Option<&str>) -> FeiQPacket {
        let mut packet = Self::make_feiq_entry_packet(nickname);
        packet.ext_info.msg_sub_type = 10; // ANSENTRY 响应
        packet
    }

    /// 创建 FeiQ 格式的离线广播包
    pub fn make_feiq_exit_packet(nickname: Option<&str>) -> FeiQPacket {
        let mut packet = Self::make_feiq_entry_packet(nickname);
        packet.ext_info.msg_sub_type = 11; // 离线广播
        packet.func_flag = 0; // 离线时功能标志为 0
        packet
    }

    /// 序列化为 FeiQ 协议字符串
    ///
    /// 格式: 版本号#长度#MAC#端口#标志1#标志2#命令#类型:时间戳:包ID:主机名:用户ID:备注
    pub fn to_feiq_string(&self) -> String {
        // 计算数据段长度
        let data_section = format!(
            "{}:{}:{}:{}:{}:{}",
            self.ext_info.msg_sub_type,
            self.ext_info.timestamp,
            self.ext_info.unique_id,
            self.ext_info.hostname,
            self.ext_info.nickname,
            self.ext_info.remark
        );

        format!(
            "1_lbt6_0#{}#{}#{}#{}#{}#{:x}#{}:{}",
            self.func_flag,
            self.mac_addr_raw,
            self.udp_port,
            self.file_transfer_id,
            self.extra_flag,
            self.client_version,
            self.ext_info.msg_sub_type,
            data_section
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
        let packet = ProtocolPacket::make_entry_packet();
        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.base_command(), IPMSG_BR_ENTRY);
    }

    #[test]
    fn test_make_message_packet() {
        let packet = ProtocolPacket::make_message_packet("Hello", true);
        assert_eq!(packet.base_command(), IPMSG_SENDMSG);
        assert!(packet.has_option(IPMSG_UTF8OPT));
        assert!(packet.has_option(IPMSG_SENDCHECKOPT));
        assert_eq!(packet.extension, Some("Hello".to_string()));
    }

    #[test]
    fn test_to_string() {
        let packet = ProtocolPacket {
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

    #[test]
    fn test_make_feiq_entry_packet_format() {
        let packet = FeiQPacket::make_feiq_entry_packet(Some("testuser"));
        let serialized = packet.to_feiq_string();

        assert!(
            serialized.contains('#'),
            "FeiQ packet should contain # delimiter (not : like IPMsg)"
        );
        assert!(serialized.contains("1_lbt6_0"), "Should have correct version string");
        assert!(
            serialized.contains("4001"),
            "Should have correct client version command (0x4001)"
        );
        assert!(serialized.contains("#2425#"), "Should have port 2425");
        assert!(serialized.contains("#128#"), "Should have func_flag 128 for online");
        assert!(serialized.contains(":9:"), "Should have msg_sub_type 9 for BR_ENTRY");
    }

    #[test]
    fn test_make_feiq_ansentry_packet_format() {
        let packet = FeiQPacket::make_feiq_ansentry_packet(Some("testuser"));
        let serialized = packet.to_feiq_string();

        assert!(
            serialized.contains('#'),
            "FeiQ ANSENTRY packet should contain # delimiter"
        );
        assert!(serialized.contains(":10:"), "ANSENTRY should have msg_sub_type 10");
        assert!(
            serialized.contains("#128#"),
            "ANSENTRY should have func_flag 128 (online)"
        );
        assert!(serialized.contains("1_lbt6_0"), "Should have correct version string");
    }

    #[test]
    fn test_make_feiq_exit_packet_format() {
        let packet = FeiQPacket::make_feiq_exit_packet(Some("testuser"));
        let serialized = packet.to_feiq_string();

        assert!(serialized.contains('#'), "FeiQ EXIT packet should contain # delimiter");
        assert!(serialized.contains(":11:"), "EXIT should have msg_sub_type 11");
        assert!(serialized.contains("#0#"), "EXIT should have func_flag 0 (offline)");
        assert!(serialized.contains("1_lbt6_0"), "Should have correct version string");
    }

    #[test]
    fn test_feiq_packet_serialization() {
        let packet = FeiQPacket::make_feiq_entry_packet(Some("testuser"));
        let serialized = packet.to_feiq_string();

        let header_parts: Vec<&str> = serialized.split('#').collect();

        assert_eq!(header_parts[0], "1_lbt6_0", "Version should be 1_lbt6_0");
        assert_eq!(header_parts[3], "2425", "Port should be 2425");
        assert_eq!(header_parts[6], "4001", "Client version should be 0x4001");

        let data_section = header_parts.get(7).expect("Should have data section");
        let data_parts: Vec<&str> = data_section.split(':').collect();

        assert_eq!(data_parts[0], "9", "msg_sub_type should be 9 for BR_ENTRY");
        assert!(data_parts.len() >= 6, "Should have all 6 required data fields");
        assert!(!data_parts[1].is_empty(), "Timestamp should not be empty");
        assert!(!data_parts[2].is_empty(), "Packet ID should not be empty");
        assert!(!data_parts[3].is_empty(), "Hostname should not be empty");
        assert!(!data_parts[4].is_empty(), "Nickname should not be empty");
    }

    #[test]
    fn test_feiq_packet_has_mac_address() {
        let packet = FeiQPacket::make_feiq_entry_packet(Some("testuser"));
        let serialized = packet.to_feiq_string();

        let header_parts: Vec<&str> = serialized.split('#').collect();
        assert!(header_parts.len() > 2, "Should have multiple # separated parts");

        let mac_part = header_parts[2];
        assert!(!mac_part.is_empty(), "MAC address should not be empty");
        assert_eq!(mac_part.len(), 12, "Raw MAC should be 12 hex characters");
        assert!(
            mac_part.chars().all(|c| c.is_ascii_hexdigit()),
            "MAC should contain only hex characters"
        );
    }

    #[test]
    fn test_feiq_packet_fields_match() {
        let nickname = "testuser";
        let packet = FeiQPacket::make_feiq_entry_packet(Some(nickname));
        let serialized = packet.to_feiq_string();

        assert!(serialized.contains(nickname), "Packet should contain the nickname");
        assert_eq!(packet.func_flag, 128, "Entry packet should have func_flag 128");
        assert_eq!(packet.client_version, 0x4001, "Client version should be 0x4001");
        assert_eq!(packet.udp_port, 2425, "UDP port should be 2425");
        assert_eq!(packet.ext_info.msg_sub_type, 9, "Entry should have msg_sub_type 9");
    }

    #[test]
    fn test_feiq_exit_has_zero_func_flag() {
        let packet = FeiQPacket::make_feiq_exit_packet(Some("testuser"));

        assert_eq!(packet.func_flag, 0, "Exit packet should have func_flag 0 (offline)");
        assert_eq!(packet.ext_info.msg_sub_type, 11, "Exit should have msg_sub_type 11");
    }

    #[test]
    fn test_feiq_ansentry_has_correct_fields() {
        let nickname = "testuser";
        let packet = FeiQPacket::make_feiq_ansentry_packet(Some(nickname));

        assert_eq!(packet.ext_info.msg_sub_type, 10, "ANSENTRY should have msg_sub_type 10");
        assert_eq!(packet.func_flag, 128, "ANSENTRY should have func_flag 128 (online)");
        assert!(packet.ext_info.nickname.contains(nickname), "Should preserve nickname");
        assert_eq!(packet.client_version, 0x4001, "Should have correct client version");
    }
}
