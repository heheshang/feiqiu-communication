// src-tauri/src/types.rs
//
/// 共享类型定义
use serde::{Deserialize, Serialize};

// ============================================================
// 用户相关
// ============================================================

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub uid: i64,
    pub nickname: String,
    pub feiq_ip: String,
    pub feiq_port: u16,
    pub feiq_machine_id: String,
    pub avatar: Option<String>,
    pub status: i8, // 0-离线, 1-在线, 2-忙碌
}

/// 在线状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum OnlineStatus {
    Offline = 0,
    Online = 1,
    Busy = 2,
}

impl OnlineStatus {
    #[allow(dead_code)]
    pub fn from_i8(value: i8) -> Self {
        match value {
            0 => OnlineStatus::Offline,
            1 => OnlineStatus::Online,
            2 => OnlineStatus::Busy,
            _ => OnlineStatus::Offline,
        }
    }
}

// ============================================================
// 聊天相关
// ============================================================

/// 会话类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    Single = 0, // 单聊
    Group = 1,  // 群聊
}

impl SessionType {
    #[allow(dead_code)]
    pub fn from_i8(value: i8) -> Self {
        match value {
            0 => SessionType::Single,
            1 => SessionType::Group,
            _ => SessionType::Single,
        }
    }
}

/// 消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Text = 0,  // 文字消息
    File = 1,  // 文件消息
    Emoji = 2, // Emoji 消息
}

/// 消息状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageStatus {
    Sending = 0, // 发送中
    Sent = 1,    // 已发送
    Read = 2,    // 已读
    Failed = -1, // 发送失败
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub mid: i64,
    pub session_type: SessionType,
    pub target_id: i64,
    pub sender_uid: i64,
    pub msg_type: MessageType,
    pub content: String,
    pub send_time: String,
    pub status: MessageStatus,
}

/// 聊天会话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub sid: i64,
    pub owner_uid: i64,
    pub session_type: SessionType,
    pub target_id: i64,
    pub last_msg_id: Option<i64>,
    pub unread_count: i32,
    pub update_time: String,
}

// ============================================================
// 通讯录相关
// ============================================================

/// 联系人
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub owner_uid: i64,
    pub contact_uid: i64,
    pub remark: Option<String>,
    pub tag: Option<String>,
}

// ============================================================
// 群组相关
// ============================================================

/// 群组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    pub gid: i64,
    pub group_name: String,
    pub avatar: Option<String>,
    pub creator_uid: i64,
    pub desc: Option<String>,
    pub create_time: String,
}

/// 群成员角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupRole {
    Member = 0, // 普通成员
    Admin = 1,  // 管理员
    Owner = 2,  // 群主
}

/// 群成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub id: i64,
    pub gid: i64,
    pub member_uid: i64,
    pub nickname: String, // 成员昵称
    pub role: GroupRole,
    pub join_time: String,
}

// ============================================================
// 文件相关
// ============================================================

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub fid: i64,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub file_type: String,
    pub uploader_uid: i64,
    pub upload_time: String,
}

/// 文件传输状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TransferStatus {
    Pending = 0,      // 等待中
    Transferring = 1, // 传输中
    Completed = 2,    // 已完成
    Failed = -1,      // 失败
    Cancelled = -2,   // 已取消
}

/// 文件传输进度
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TransferProgress {
    pub file_id: i64,
    pub progress: u64,
    pub total: u64,
    pub status: TransferStatus,
}

/// 待恢复的传输信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransfer {
    pub tid: i64,
    pub file_id: i64,
    pub file_name: String,
    pub file_path: String,
    pub transferred: i64,
    pub file_size: i64,
    pub status: TransferStatus,
    pub target_ip: String,
    pub direction: i8, // 0=下载, 1=上传
}
