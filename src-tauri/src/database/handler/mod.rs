// src-tauri/src/database/handler/mod.rs
//
//! 数据库 CRUD 处理器模块

pub mod chat;
pub mod contact;
pub mod file;
pub mod group;
pub mod transfer_state;
pub mod user;

pub use chat::{ChatMessageHandler, ChatSessionHandler};
pub use contact::ContactHandler;
pub use file::FileStorageHandler;
pub use transfer_state::TransferStateHandler;
pub use user::UserHandler;
