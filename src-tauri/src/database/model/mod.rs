// src-tauri/src/database/model/mod.rs
//
//! SeaORM 实体模型
//!
//! 定义所有数据库实体模型

pub mod chat_message;
pub mod chat_session;
pub mod contact;
pub mod file_storage;
pub mod group;
pub mod group_member;
pub mod transfer_state;
pub mod user;

// 导出所有实体
pub use chat_message::Entity as ChatMessage;
pub use chat_session::Entity as ChatSession;
pub use contact::Entity as Contact;
pub use file_storage::Entity as FileStorage;
pub use group::Entity as Group;
pub use group_member::Entity as GroupMember;
pub use transfer_state::Entity as TransferState;
pub use user::Entity as User;
