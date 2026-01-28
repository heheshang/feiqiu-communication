// src-tauri/src/core/chat/mod.rs
//
/// 聊天业务逻辑层
///
/// 根据项目架构文档，该模块负责：
/// - 处理从网络层接收到的消息
/// - 管理消息发送流程
/// - 处理已读回执
/// - 管理会话状态和未读计数
///
/// 模块结构：
/// - receiver: 消息接收处理器
/// - sender: 消息发送处理器
/// - receipt: 已读回执处理器
/// - manager: 会话管理器
pub mod manager;
pub mod receipt;
pub mod receiver;
pub mod sender;

// 重新导出主要类型
pub use manager::ChatManager;
pub use receipt::ReceiptHandler;
pub use receiver::MessageReceiver;
pub use sender::MessageSender;
