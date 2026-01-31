// src-tauri/src/network/feiq/packer.rs
//
/// 飞秋协议封装器
use crate::network::feiq::model::{FeiQExtInfo, FeiQPacket};
use crate::network::feiq::utils::{format_mac_addr, timestamp_to_local};
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
            func_flag: 128,
            mac_addr_raw: mac_addr,
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

    /// 创建 FeiQ 格式的消息包 (SENDMSG)
    ///
    /// 格式: 1_lbt6_0#func_flag#MAC#端口#0#0#4001#20:时间戳:包ID:主机名:用户名:消息内容
    pub fn make_feiq_message_packet(content: &str, nickname: Option<&str>) -> FeiQPacket {
        let (username, hostname, _ip, _port) = get_system_user_info();
        let mac_addr = get_mac_address();
        let mac_formatted = format_mac_addr(&mac_addr).unwrap_or_else(|_| mac_addr.clone());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
        let packet_id = generate_packet_id();
        let nickname = nickname.unwrap_or(&username).to_string();

        FeiQPacket {
            pkg_type: "1_lbt6_0".to_string(),
            func_flag: 128,
            mac_addr_raw: mac_addr.clone(),
            mac_addr_formatted: mac_formatted,
            udp_port: 2425,
            file_transfer_id: 0,
            extra_flag: 0,
            client_version: 0x4001,
            ext_info: FeiQExtInfo {
                msg_sub_type: 0x20,
                timestamp,
                timestamp_local: timestamp_to_local(timestamp),
                unique_id: packet_id,
                hostname: hostname.clone(),
                nickname,
                remark: content.to_string(),
            },
        }
    }

    /// 创建 FeiQ 格式的接收确认包 (RECVMSG)
    pub fn make_feiq_recv_packet(msg_no: &str) -> FeiQPacket {
        let (username, hostname, _ip, _port) = get_system_user_info();
        let mac_addr = get_mac_address();
        let mac_formatted = format_mac_addr(&mac_addr).unwrap_or_else(|_| mac_addr.clone());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
        let packet_id = generate_packet_id();

        FeiQPacket {
            pkg_type: "1_lbt6_0".to_string(),
            func_flag: 128,
            mac_addr_raw: mac_addr.clone(),
            mac_addr_formatted: mac_formatted,
            udp_port: 2425,
            file_transfer_id: 0,
            extra_flag: 0,
            client_version: 0x4001,
            ext_info: FeiQExtInfo {
                msg_sub_type: 0x21,
                timestamp,
                timestamp_local: timestamp_to_local(timestamp),
                unique_id: packet_id,
                hostname,
                nickname: username,
                remark: msg_no.to_string(),
            },
        }
    }

    /// 创建 FeiQ 格式的已读回执包 (READMSG)
    pub fn make_feiq_read_packet(msg_no: &str) -> FeiQPacket {
        let mut packet = Self::make_feiq_recv_packet(msg_no);
        packet.ext_info.msg_sub_type = 0x30;
        packet
    }

    /// 创建 FeiQ 格式的已读应答包 (ANSREADMSG)
    pub fn make_feiq_ansread_packet(msg_no: &str) -> FeiQPacket {
        let mut packet = Self::make_feiq_recv_packet(msg_no);
        packet.ext_info.msg_sub_type = 0x32;
        packet
    }

    // ============================================================
    // 文件传输相关数据包 (FeiQ 格式)
    // ============================================================

    /// 创建文件附件请求包 (FILEATTACH)
    ///
    /// 格式: remark 字段包含文件信息 "filename:size:mtime:attr"
    pub fn make_feiq_file_attach_packet(
        files: &[crate::network::feiq::model::FileAttachment],
        nickname: Option<&str>,
    ) -> FeiQPacket {
        let (username, hostname, _ip, _port) = get_system_user_info();
        let mac_addr = get_mac_address();
        let mac_formatted = format_mac_addr(&mac_addr).unwrap_or_else(|_| mac_addr.clone());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
        let packet_id = generate_packet_id();
        let nickname = nickname.unwrap_or(&username).to_string();

        // 构建文件附件信息: 多个文件用 \a (0x07) 分隔
        // 格式: "filename1:size1:mtime1:attr1\afilename2:size2:mtime2:attr2"
        let files_info: Vec<String> = files
            .iter()
            .map(|f| format!("{}:{}:{}:{}", f.file_name, f.file_size, f.mtime, f.attr))
            .collect();
        let remark = files_info.join("\x07");

        FeiQPacket {
            pkg_type: "1_lbt6_0".to_string(),
            func_flag: 128,
            mac_addr_raw: mac_addr,
            mac_addr_formatted: mac_formatted,
            udp_port: 2425,
            file_transfer_id: 0,
            extra_flag: 0,
            client_version: 0x4001,
            ext_info: FeiQExtInfo {
                msg_sub_type: 0x20, // SENDMSG
                timestamp,
                timestamp_local: timestamp_to_local(timestamp),
                unique_id: packet_id,
                hostname: hostname.clone(),
                nickname,
                remark,
            },
        }
    }

    /// 创建文件数据请求包 (GETFILEDATA)
    ///
    /// 用于接收方请求文件数据块
    pub fn make_feiq_get_file_data_packet(
        packet_no: &str,
        file_id: u64,
        offset: u64,
        nickname: Option<&str>,
    ) -> FeiQPacket {
        let (username, hostname, _ip, _port) = get_system_user_info();
        let mac_addr = get_mac_address();
        let mac_formatted = format_mac_addr(&mac_addr).unwrap_or_else(|_| mac_addr.clone());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
        let packet_id = generate_packet_id();
        let nickname = nickname.unwrap_or(&username).to_string();

        // remark 字段: "packet_no:file_id:offset"
        let remark = format!("{}:{}:{}", packet_no, file_id, offset);

        FeiQPacket {
            pkg_type: "1_lbt6_0".to_string(),
            func_flag: 128,
            mac_addr_raw: mac_addr,
            mac_addr_formatted: mac_formatted,
            udp_port: 2425,
            file_transfer_id: file_id as u32,
            extra_flag: 0,
            client_version: 0x4001,
            ext_info: FeiQExtInfo {
                msg_sub_type: 0x60, // GETFILEDATA
                timestamp,
                timestamp_local: timestamp_to_local(timestamp),
                unique_id: packet_id,
                hostname: hostname.clone(),
                nickname,
                remark,
            },
        }
    }

    /// 创建文件数据包 (用于发送文件数据块)
    ///
    /// 用于发送方响应文件数据请求
    pub fn make_feiq_file_data_packet(
        packet_no: &str,
        file_id: u64,
        offset: u64,
        data: &[u8],
        nickname: Option<&str>,
    ) -> FeiQPacket {
        let (username, hostname, _ip, _port) = get_system_user_info();
        let mac_addr = get_mac_address();
        let mac_formatted = format_mac_addr(&mac_addr).unwrap_or_else(|_| mac_addr.clone());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
        let packet_id = generate_packet_id();
        let nickname = nickname.unwrap_or(&username).to_string();

        // 使用 base64 编码文件数据
        use base64::Engine;
        let data_base64 = base64::engine::general_purpose::STANDARD.encode(data);
        let remark = format!("{}:{}:{}:{}", packet_no, file_id, offset, data_base64);

        FeiQPacket {
            pkg_type: "1_lbt6_0".to_string(),
            func_flag: 128,
            mac_addr_raw: mac_addr,
            mac_addr_formatted: mac_formatted,
            udp_port: 2425,
            file_transfer_id: file_id as u32,
            extra_flag: 0,
            client_version: 0x4001,
            ext_info: FeiQExtInfo {
                msg_sub_type: 0x61, // File data response
                timestamp,
                timestamp_local: timestamp_to_local(timestamp),
                unique_id: packet_id,
                hostname: hostname.clone(),
                nickname,
                remark,
            },
        }
    }

    /// 创建文件释放包 (RELEASEFILES)
    ///
    /// 用于取消文件传输或通知发送方释放文件资源
    pub fn make_feiq_release_files_packet(packet_no: &str, nickname: Option<&str>) -> FeiQPacket {
        let (username, hostname, _ip, _port) = get_system_user_info();
        let mac_addr = get_mac_address();
        let mac_formatted = format_mac_addr(&mac_addr).unwrap_or_else(|_| mac_addr.clone());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
        let packet_id = generate_packet_id();
        let nickname = nickname.unwrap_or(&username).to_string();

        // remark 字段只包含 packet_no
        let remark = packet_no.to_string();

        FeiQPacket {
            pkg_type: "1_lbt6_0".to_string(),
            func_flag: 128,
            mac_addr_raw: mac_addr,
            mac_addr_formatted: mac_formatted,
            udp_port: 2425,
            file_transfer_id: 0,
            extra_flag: 0,
            client_version: 0x4001,
            ext_info: FeiQExtInfo {
                msg_sub_type: 0x62, // RELEASEFILES (using 0x62 as file release)
                timestamp,
                timestamp_local: timestamp_to_local(timestamp),
                unique_id: packet_id,
                hostname: hostname.clone(),
                nickname,
                remark,
            },
        }
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
