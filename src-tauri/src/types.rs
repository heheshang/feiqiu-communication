// src-tauri/src/types.rs
//
/// 共享类型定义
use serde::{Deserialize, Serialize};

// ============================================================
// 错误类型
// ============================================================

/// 前端错误结构（可序列化，用于IPC传递）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

/// 错误代码枚举
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Database = 0,
    Network = 1,
    Io = 2,
    Business = 3,
    Serialize = 4,
    Protocol = 5,
    NotFound = 6,
    AlreadyExists = 7,
    Validation = 8,
    Permission = 9,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Database => write!(f, "DATABASE_ERROR"),
            ErrorCode::Network => write!(f, "NETWORK_ERROR"),
            ErrorCode::Io => write!(f, "IO_ERROR"),
            ErrorCode::Business => write!(f, "BUSINESS_ERROR"),
            ErrorCode::Serialize => write!(f, "SERIALIZE_ERROR"),
            ErrorCode::Protocol => write!(f, "PROTOCOL_ERROR"),
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::AlreadyExists => write!(f, "ALREADY_EXISTS"),
            ErrorCode::Validation => write!(f, "VALIDATION_ERROR"),
            ErrorCode::Permission => write!(f, "PERMISSION_DENIED"),
        }
    }
}

impl std::fmt::Display for FrontendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref details) = self.details {
            write!(f, "[{}] {} - {}", self.code, self.message, details)
        } else {
            write!(f, "[{}] {}", self.code, self.message)
        }
    }
}

impl std::error::Error for FrontendError {}

impl From<crate::error::AppError> for FrontendError {
    fn from(err: crate::error::AppError) -> Self {
        match err {
            crate::error::AppError::Database(e) => FrontendError {
                code: ErrorCode::Database,
                message: "数据库操作失败".to_string(),
                details: Some(e.to_string()),
            },
            crate::error::AppError::Network(msg) => FrontendError {
                code: ErrorCode::Network,
                message: "网络操作失败".to_string(),
                details: Some(msg),
            },
            crate::error::AppError::Io(e) => FrontendError {
                code: ErrorCode::Io,
                message: "文件操作失败".to_string(),
                details: Some(e.to_string()),
            },
            crate::error::AppError::Business(msg) => FrontendError {
                code: ErrorCode::Business,
                message: msg,
                details: None,
            },
            crate::error::AppError::Serialize(msg) => FrontendError {
                code: ErrorCode::Serialize,
                message: "序列化失败".to_string(),
                details: Some(msg),
            },
            crate::error::AppError::Protocol(msg) => FrontendError {
                code: ErrorCode::Protocol,
                message: "协议解析失败".to_string(),
                details: Some(msg),
            },
            crate::error::AppError::NotFound(what) => FrontendError {
                code: ErrorCode::NotFound,
                message: format!("{} 不存在", what),
                details: None,
            },
            crate::error::AppError::AlreadyExists(what) => FrontendError {
                code: ErrorCode::AlreadyExists,
                message: format!("{} 已存在", what),
                details: None,
            },
        }
    }
}

impl FrontendError {
    /// 将错误转换为 JSON 字符串（用于 IPC 返回）
    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| format!("{{\"code\":{:?},\"message\":\"{}\"}}", self.code, self.message))
    }

    /// 从 JSON 字符串解析错误
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_else(|_| FrontendError {
            code: ErrorCode::Business,
            message: json.to_string(),
            details: None,
        })
    }
}

/// IPC 错误转换辅助宏
/// 用法：`.map_err_to_frontend()`
pub trait MapErrToFrontend<T> {
    fn map_err_to_frontend(self) -> Result<T, String>;
}

impl<T, E: Into<crate::error::AppError>> MapErrToFrontend<T> for Result<T, E> {
    fn map_err_to_frontend(self) -> Result<T, String> {
        self.map_err(|e| {
            let app_err: crate::error::AppError = e.into();
            let frontend_err: FrontendError = app_err.into();
            frontend_err.to_json()
        })
    }
}

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
