// src-tauri/src/database/handler/mod.rs
//
//! 数据库 CRUD 处理器模块

pub mod user;
pub mod contact;
pub mod group;
pub mod chat;
pub mod file;

pub use user::UserHandler;
pub use contact::ContactHandler;
pub use group::{GroupHandler, GroupMemberHandler};
pub use chat::{ChatMessageHandler, ChatSessionHandler};
pub use file::{FileStorageHandler, FileStats};
