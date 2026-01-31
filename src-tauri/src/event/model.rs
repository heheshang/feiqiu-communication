// src-tauri/src/event/model.rs
//
/// 事件模型定义
/// 参考: reference/event/model.rs
use serde::{Deserialize, Serialize};

// ============================================================
// 应用事件（顶层事件）
// ============================================================

/// 应用事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    /// 网络事件
    Network(NetworkEvent),

    /// UI 事件
    Ui(UiEvent),

    /// 文件事件
    File(FileEvent),

    /// 聊天事件
    Chat(ChatEvent),
}

// ============================================================
// 网络事件
// ============================================================

/// 网络相关事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkEvent {
    /// 用户上线（IPMSG_BR_ENTRY）
    UserOnline {
        ip: String,
        port: u16,
        nickname: String,
        hostname: Option<String>,
        mac_addr: Option<String>,
    },

    /// 用户下线（IPMSG_BR_EXIT）
    UserOffline { ip: String },

    /// 在线应答（IPMSG_ANSENTRY）
    UserPresenceResponse {
        ip: String,
        port: u16,
        nickname: String,
        hostname: Option<String>,
    },

    /// 收到消息（IPMSG_SENDMSG）
    MessageReceived {
        sender_ip: String,
        sender_port: u16,
        sender_nickname: String,
        content: String,
        msg_no: String,
        needs_receipt: bool,
    },

    /// 收到确认（IPMSG_RECVMSG）
    MessageReceiptReceived { msg_no: String },

    /// 消息已读（IPMSG_READMSG）
    MessageRead { msg_no: String },

    /// 消息删除（IPMSG_DELMSG）
    MessageDeleted { msg_no: String },

    /// 文件请求（IPMSG_FILEATTACHOPT）
    FileRequestReceived {
        from_ip: String,
        files: String, // Vec<FileInfo> JSON
    },

    /// 文件数据请求（IPMSG_GETFILEDATA）
    FileDataRequest {
        from_ip: String,
        packet_no: String,
        file_id: u64,
        offset: u64,
    },

    /// 文件数据接收（文件块数据）
    FileDataReceived {
        from_ip: String,
        packet_no: String,
        file_id: u64,
        offset: u64,
        data: String, // Base64 encoded file chunk
    },

    /// 文件释放（取消文件传输）
    FileRelease { from_ip: String, packet_no: String },

    /// 用户更新信息
    UserUpdated {
        user: String, // UserInfo JSON
    },

    /// 消息发送成功
    MessageSent { msg_id: i64 },

    /// 消息发送失败
    MessageSendFailed { msg_id: i64, error: String },

    /// UDP 接收器启动
    UdpReceiverStarted { port: u16 },

    /// UDP 接收器错误
    UdpReceiverError { error: String },
}

// ============================================================
// UI 事件
// ============================================================

/// UI 相关事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiEvent {
    /// 显示消息通知
    ShowNotification { title: String, body: String },

    /// 更新用户列表
    UpdateUserList {
        users: String, // Vec<UserInfo> JSON
    },

    /// 添加单个用户
    AddUser {
        user: String, // UserInfo JSON
    },

    /// 移除用户
    RemoveUser { ip: String },

    /// 打开聊天窗口
    OpenChatWindow { user_id: i64 },

    /// 关闭聊天窗口
    CloseChatWindow { user_id: i64 },

    /// 更新聊天窗口标题
    UpdateChatTitle { user_id: i64, title: String },

    /// 显示消息
    DisplayMessage {
        session_type: i8,
        target_id: i64,
        message: String, // ChatMessage JSON
    },

    /// 更新消息状态
    UpdateMessageStatus { msg_id: i64, status: i8 },

    /// 更新未读计数
    UpdateUnreadCount {
        session_type: i8,
        target_id: i64,
        count: i32,
    },

    /// 文件传输进度更新
    FileTransferProgress { file_id: i64, progress: u64, total: u64 },

    /// 文件传输完成
    FileTransferComplete { file_id: i64 },

    /// 文件传输失败
    FileTransferFailed { file_id: i64, error: String },
}

// ============================================================
// 文件事件
// ============================================================

/// 文件相关事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileEvent {
    /// 文件接收请求
    ReceiveRequest {
        from_user: String,
        files: String, // Vec<FileInfo> JSON
    },

    /// 开始文件下载
    DownloadStarted { file_id: i64 },

    /// 文件下载完成
    DownloadCompleted { file_id: i64, path: String },

    /// 文件下载失败
    DownloadFailed { file_id: i64, error: String },

    /// 文件上传开始
    UploadStarted { file_id: i64 },

    /// 文件上传完成
    UploadCompleted { file_id: i64 },

    /// 文件上传失败
    UploadFailed { file_id: i64, error: String },

    /// 取消文件传输
    TransferCancelled { file_id: i64 },
}

// ============================================================
// 聊天事件
// ============================================================

/// 聊天相关事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEvent {
    /// 发送消息
    SendMessage {
        session_type: i8,
        target_id: i64,
        content: String,
    },

    /// 消息已读
    MessageRead { msg_id: i64 },

    /// 消息删除
    MessageDeleted { msg_id: i64 },

    /// 会话创建
    SessionCreated { session_id: i64 },

    /// 会话更新
    SessionUpdated { session_id: i64 },

    /// 会话删除
    SessionDeleted { session_id: i64 },

    /// 群组创建
    GroupCreated { group_id: i64 },

    /// 群组成员添加
    GroupMemberAdded { group_id: i64, user_id: i64 },

    /// 群组成员移除
    GroupMemberRemoved { group_id: i64, user_id: i64 },

    /// 群组解散
    GroupDisbanded { group_id: i64 },
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod event_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_user_online_event_serialization() {
        let event = NetworkEvent::UserOnline {
            ip: "192.168.1.100".to_string(),
            port: 2425,
            nickname: "TestUser".to_string(),
            hostname: Some("DESKTOP-ABC".to_string()),
            mac_addr: Some("00:11:22:33:44:55".to_string()),
        };

        // 测试序列化
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("UserOnline"));
        assert!(json.contains("192.168.1.100"));

        // 测试反序列化
        let deserialized: NetworkEvent = serde_json::from_str(&json).unwrap();
        match deserialized {
            NetworkEvent::UserOnline { ip, port, nickname, .. } => {
                assert_eq!(ip, "192.168.1.100");
                assert_eq!(port, 2425);
                assert_eq!(nickname, "TestUser");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_message_received_event_serialization() {
        let event = NetworkEvent::MessageReceived {
            sender_ip: "192.168.1.100".to_string(),
            sender_port: 2425,
            sender_nickname: "TestUser".to_string(),
            content: "Hello, World!".to_string(),
            msg_no: "12345".to_string(),
            needs_receipt: true,
        };

        // 测试序列化
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("MessageReceived"));
        assert!(json.contains("Hello, World!"));

        // 测试反序列化
        let deserialized: NetworkEvent = serde_json::from_str(&json).unwrap();
        match deserialized {
            NetworkEvent::MessageReceived {
                content, needs_receipt, ..
            } => {
                assert_eq!(content, "Hello, World!");
                assert_eq!(needs_receipt, true);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_user_offline_event_serialization() {
        let event = NetworkEvent::UserOffline {
            ip: "192.168.1.100".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: NetworkEvent = serde_json::from_str(&json).unwrap();

        match deserialized {
            NetworkEvent::UserOffline { ip } => {
                assert_eq!(ip, "192.168.1.100");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_all_network_events_are_serializable() {
        // 测试所有NetworkEvent变体都可以序列化/反序列化
        let events = vec![
            NetworkEvent::UserOnline {
                ip: "1.1.1.1".to_string(),
                port: 1,
                nickname: "A".to_string(),
                hostname: None,
                mac_addr: None,
            },
            NetworkEvent::UserOffline {
                ip: "2.2.2.2".to_string(),
            },
            NetworkEvent::UserPresenceResponse {
                ip: "3.3.3.3".to_string(),
                port: 3,
                nickname: "C".to_string(),
                hostname: None,
            },
            NetworkEvent::MessageReceived {
                sender_ip: "4.4.4.4".to_string(),
                sender_port: 4,
                sender_nickname: "D".to_string(),
                content: "Test".to_string(),
                msg_no: "0".to_string(),
                needs_receipt: false,
            },
            NetworkEvent::MessageReceiptReceived {
                msg_no: "0".to_string(),
            },
            NetworkEvent::MessageRead {
                msg_no: "0".to_string(),
            },
            NetworkEvent::MessageDeleted {
                msg_no: "0".to_string(),
            },
            NetworkEvent::FileRequestReceived {
                from_ip: "5.5.5.5".to_string(),
                files: "[]".to_string(),
            },
        ];

        for event in events {
            let json = serde_json::to_string(&event).unwrap();
            let _deserialized: NetworkEvent = serde_json::from_str(&json).unwrap();
        }
    }
}
