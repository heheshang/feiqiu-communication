// src-tauri/src/core/mod.rs
//
/// 核心业务逻辑层
pub mod chat;
pub mod contact;
pub mod file;
pub mod group;

// 导出所有 Service
pub use chat::ChatService;
pub use contact::ContactService;
pub use file::FileService;
pub use group::GroupService;
