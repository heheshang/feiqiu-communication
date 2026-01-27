// src/event/model.rs
///
/// 事件模型定义
///
/// 参考: langzime/ipmsg-rs (src/models/event.rs)
///

use crate::models::user::UserInfo;
use crate::models::message::ChatMessage;
use crate::models::file::FileInfo;
use crate::network::feiq::model::FeiqPacket;

// ============================================================
// 应用事件（顶层事件）
// ============================================================

/// 应用事件类型
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// 收到 UDP 数据包
    PacketReceived {
        packet: FeiqPacket,
        addr: String,
    },

    /// 用户上线
    UserOnline {
        user: UserInfo,
    },

    /// 用户下线
    UserOffline {
        ip: String,
    },

    /// 用户更新信息
    UserUpdated {
        user: UserInfo,
    },

    /// 消息发送成功
    MessageSent {
        msg_id: i64,
    },

    /// 消息发送失败
    MessageSendFailed {
        msg_id: i64,
        error: String,
    },

    /// UDP 接收器启动
    UdpReceiverStarted {
        port: u16,
    },

    /// UDP 接收器错误
    UdpReceiverError {
        error: String,
    },
}

// ============================================================
// UI 事件
// ============================================================

/// UI 相关事件
#[derive(Debug, Clone)]
pub enum UiEvent {
    /// 显示消息通知
    ShowNotification {
        title: String,
        body: String,
    },

    /// 更新用户列表
    UpdateUserList {
        users: Vec<UserInfo>,
    },

    /// 添加单个用户
    AddUser {
        user: UserInfo,
    },

    /// 移除用户
    RemoveUser {
        ip: String,
    },

    /// 打开聊天窗口
    OpenChatWindow {
        user_id: i64,
    },

    /// 关闭聊天窗口
    CloseChatWindow {
        user_id: i64,
    },

    /// 更新聊天窗口标题
    UpdateChatTitle {
        user_id: i64,
        title: String,
    },

    /// 显示消息
    DisplayMessage {
        session_type: i8,
        target_id: i64,
        message: ChatMessage,
    },

    /// 更新消息状态
    UpdateMessageStatus {
        msg_id: i64,
        status: i8,
    },

    /// 更新未读计数
    UpdateUnreadCount {
        session_type: i8,
        target_id: i64,
        count: i32,
    },

    /// 文件传输进度更新
    FileTransferProgress {
        file_id: i64,
        progress: u64,
        total: u64,
    },

    /// 文件传输完成
    FileTransferComplete {
        file_id: i64,
    },

    /// 文件传输失败
    FileTransferFailed {
        file_id: i64,
        error: String,
    },
}

// ============================================================
// 文件事件
// ============================================================

/// 文件相关事件
#[derive(Debug, Clone)]
pub enum FileEvent {
    /// 文件接收请求
    ReceiveRequest {
        from_user: String,
        files: Vec<FileInfo>,
    },

    /// 开始文件下载
    DownloadStarted {
        file_id: i64,
    },

    /// 文件下载完成
    DownloadCompleted {
        file_id: i64,
        path: String,
    },

    /// 文件下载失败
    DownloadFailed {
        file_id: i64,
        error: String,
    },

    /// 文件上传开始
    UploadStarted {
        file_id: i64,
    },

    /// 文件上传完成
    UploadCompleted {
        file_id: i64,
    },

    /// 文件上传失败
    UploadFailed {
        file_id: i64,
        error: String,
    },

    /// 取消文件传输
    TransferCancelled {
        file_id: i64,
    },
}

// ============================================================
// 聊天事件
// ============================================================

/// 聊天相关事件
#[derive(Debug, Clone)]
pub enum ChatEvent {
    /// 发送消息
    SendMessage {
        session_type: i8,
        target_id: i64,
        content: String,
    },

    /// 消息已读
    MessageRead {
        msg_id: i64,
    },

    /// 消息删除
    MessageDeleted {
        msg_id: i64,
    },

    /// 会话创建
    SessionCreated {
        session_id: i64,
    },

    /// 会话更新
    SessionUpdated {
        session_id: i64,
    },

    /// 会话删除
    SessionDeleted {
        session_id: i64,
    },

    /// 群组创建
    GroupCreated {
        group_id: i64,
    },

    /// 群组成员添加
    GroupMemberAdded {
        group_id: i64,
        user_id: i64,
    },

    /// 群组成员移除
    GroupMemberRemoved {
        group_id: i64,
        user_id: i64,
    },

    /// 群组解散
    GroupDisbanded {
        group_id: i64,
    },
}

// ============================================================
// 辅助函数
// ============================================================

impl AppEvent {
    /// 获取事件的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEvent::Network(NetworkEvent::PacketReceived { .. }) => "Network::PacketReceived",
            AppEvent::Network(NetworkEvent::UserOnline { .. }) => "Network::UserOnline",
            AppEvent::Ui(UiEvent::ShowNotification { .. }) => "Ui::ShowNotification",
            AppEvent::File(FileEvent::ReceiveRequest { .. }) => "File::ReceiveRequest",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = AppEvent::Network(NetworkEvent::PacketReceived {
            packet: FeiqPacket::default(),
            addr: "192.168.1.100".to_string(),
        });

        assert_eq!(event.as_str(), "Network::PacketReceived");
    }

    #[test]
    fn test_ui_event() {
        let event = UiEvent::ShowNotification {
            title: "Test".to_string(),
            body: "Message".to_string(),
        };

        match event {
            UiEvent::ShowNotification { title, body } => {
                assert_eq!(title, "Test");
                assert_eq!(body, "Message");
            }
            _ => panic!("Wrong event type"),
        }
    }
}
