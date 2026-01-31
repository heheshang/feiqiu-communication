// src-tauri/src/network/feiq/model.rs
//
//! 飞秋协议数据包模型

use serde::{Deserialize, Serialize};

// 导入常量
use crate::network::feiq::constants::{IPMSG_FILEATTACHOPT, IPMSG_SENDCHECKOPT, IPMSG_UTF8OPT};
// 导入工具函数
use crate::network::feiq::utils::{format_mac_addr, timestamp_to_local};

// ============================================================
// 文件附件相关
// ============================================================

/// 文件附件信息（用于文件传输）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileAttachment {
    /// 文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 文件修改时间（Unix 时间戳）
    pub mtime: u64,
    /// 文件属性 (1=普通文件, 2=目录)
    pub attr: u32,
}

impl FileAttachment {
    /// 从 IPMsg 文件头字符串解析
    ///
    /// 格式: `文件名:大小:修改时间:属性`
    /// 多个文件用 \x07 分隔
    #[allow(dead_code)]
    pub fn from_ipmsg_header(s: &str) -> Result<Vec<Self>, String> {
        let mut files = Vec::new();
        for file_str in s.split('\x07') {
            let parts: Vec<&str> = file_str.split(':').collect();
            if parts.len() < 4 {
                return Err(format!("File header must have 4 fields, found {}", parts.len()));
            }

            let file_name = parts[0].to_string();
            let file_size = parts[1].parse::<i64>().map_err(|_| "Invalid file size".to_string())?;
            let mtime = parts[2].parse::<u64>().map_err(|_| "Invalid mtime".to_string())?;
            let attr = parts[3].parse::<u32>().map_err(|_| "Invalid file attr".to_string())?;

            files.push(FileAttachment {
                file_name,
                file_size,
                mtime,
                attr,
            });
        }
        Ok(files)
    }

    /// 转换为 IPMsg 文件头字符串
    #[allow(dead_code)]
    pub fn to_ipmsg_header(&self) -> String {
        format!("{}:{}:{}:{}", self.file_name, self.file_size, self.mtime, self.attr)
    }

    /// 检查是否为目录
    #[allow(dead_code)]
    pub fn is_dir(&self) -> bool {
        self.attr == 2
    }
}

// ============================================================
// 协议类型枚举
// ============================================================

/// 协议类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum ProtocolType {
    /// IPMsg 协议 (标准格式)
    #[default]
    IPMsg,
    /// 飞秋协议 (扩展格式)
    FeiQ,
}

// ============================================================
// 飞秋扩展信息（FeiQ 协议专用）
// ============================================================

/// 飞秋数据包扩展信息（F8字段拆分后的子字段）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FeiQExtInfo {
    /// 消息子类型码（如9=上线/保活广播）
    #[serde(default)]
    pub msg_sub_type: u8,
    /// 发送时间戳（秒级）
    #[serde(default)]
    pub timestamp: i64,
    /// 发送时间戳转换为本地时间（格式：YYYY-MM-DD HH:MM:SS）
    #[serde(default)]
    pub timestamp_local: String,
    /// 发送方唯一ID（如T0170006）
    #[serde(default)]
    pub unique_id: String,
    /// 发送方主机名（如SHIKUN-SH）
    #[serde(default)]
    pub hostname: String,
    /// 发送方飞秋昵称（如6291459）
    #[serde(default)]
    pub nickname: String,
    /// 附加备注/签名（如ssk）
    #[serde(default)]
    pub remark: String,
}

/// 飞秋完整数据包结构体（主字段拆分）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FeiQPacket {
    /// 包类型标识（如1_lbt6_0=局域网广播包）
    #[serde(default)]
    pub pkg_type: String,
    /// 功能标识位（如128=客户端在线广播）
    #[serde(default)]
    pub func_flag: u16,
    /// 原始MAC地址（如5C60BA7361C6）
    #[serde(default)]
    pub mac_addr_raw: String,
    /// 格式化后的MAC地址（如5C-60-BA-73-61-C6）
    #[serde(default)]
    pub mac_addr_formatted: String,
    /// 发送方UDP端口（如1944）
    #[serde(default)]
    pub udp_port: u16,
    /// 文件传输ID（0=非文件传输）
    #[serde(default)]
    pub file_transfer_id: u32,
    /// 附加标志位（0=无特殊扩展）
    #[serde(default)]
    pub extra_flag: u32,
    /// 飞秋客户端版本号（如4001）
    #[serde(default)]
    pub client_version: u32,
    /// 扩展信息段（F8字段解析结果）
    #[serde(default)]
    pub ext_info: FeiQExtInfo,
}

/// 飞秋协议数据包（支持 IPMsg 和 FeiQ 两种格式）
///
/// IPMsg 格式: `版本号:命令字:发送者信息:接收者信息:消息编号:附加信息`
///
/// FeiQ 格式: `版本号#长度#MAC地址#端口#标志1#标志2#命令#类型:时间戳:包ID:主机名:用户ID:内容`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolPacket {
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

    /// 飞秋协议详细解析结果（仅当 protocol_type = FeiQ 时有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feiq_detail: Option<FeiQPacket>,
}

impl Default for ProtocolPacket {
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
            feiq_detail: None,
        }
    }
}

// ============================================================
// 工具函数已移至 utils.rs
// ============================================================

impl ProtocolPacket {
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
    ) -> ProtocolPacket {
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

    /// 从详细的 FeiQ 数据包结构创建 ProtocolPacket
    pub fn from_feiq_detail(detail: FeiQPacket) -> ProtocolPacket {
        Self {
            protocol_type: ProtocolType::FeiQ,
            version: detail.pkg_type.clone(),
            command: detail.client_version,
            msg_type: Some(detail.ext_info.msg_sub_type as u32),
            sender: format!("{}@{}", detail.ext_info.hostname, detail.mac_addr_raw),
            hostname: Some(detail.ext_info.hostname.clone()),
            mac_addr: Some(detail.mac_addr_raw.clone()),
            receiver: String::new(),
            msg_no: detail.ext_info.unique_id.clone(),
            timestamp: Some(detail.ext_info.timestamp as u64),
            user_id: Some(detail.ext_info.nickname.clone()),
            extension: Some(detail.ext_info.remark.clone()),
            port: Some(detail.udp_port),
            feiq_detail: Some(detail),
            ..Default::default()
        }
    }

    /// 获取格式化的 MAC 地址（如果可用）
    pub fn formatted_mac(&self) -> Option<String> {
        if let Some(ref detail) = self.feiq_detail {
            Some(detail.mac_addr_formatted.clone())
        } else if let Some(ref mac) = self.mac_addr {
            Some(format_mac_addr(mac).unwrap_or_else(|_| mac.clone()))
        } else {
            None
        }
    }

    /// 获取本地时间戳字符串（如果可用）
    pub fn local_timestamp(&self) -> Option<String> {
        if let Some(ref detail) = self.feiq_detail {
            Some(detail.ext_info.timestamp_local.clone())
        } else if let Some(ts) = self.timestamp {
            Some(timestamp_to_local(ts as i64))
        } else {
            None
        }
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ProtocolPacket tests commented out - protocol unification in progress
    // TODO: Remove ProtocolPacket struct entirely once all migration is complete

    /*
    #[test]
    fn test_default_packet() {
        let packet = ProtocolPacket::default();
        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, 0);
        assert_eq!(packet.protocol_type, ProtocolType::IPMsg);
    }

    #[test]
    fn test_base_command() {
        let packet = ProtocolPacket {
            command: 0x00800120,
            ..Default::default()
        };
        assert_eq!(packet.base_command(), 0x20);
    }

    #[test]
    fn test_has_option() {
        let packet = ProtocolPacket {
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
        assert_eq!(ProtocolPacket::detect_protocol(data), ProtocolType::IPMsg);
    }

    #[test]
    fn test_detect_protocol_feiq() {
        let data = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:SHIKUN-SH:6291459:ssk";
        assert_eq!(ProtocolPacket::detect_protocol(data), ProtocolType::FeiQ);
    }

    #[test]
    fn test_new_ipmsg() {
        let packet = ProtocolPacket::new_ipmsg(
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
    */
}
