// src-tauri/src/database/model/mod.rs
//
//! SeaORM 实体模型
//!
//! 定义所有数据库实体模型

pub mod user;
pub mod contact;
pub mod group;
pub mod group_member;
pub mod chat_message;
pub mod chat_session;
pub mod file_storage;

// 导出所有实体
pub use user::Entity as User;
pub use contact::Entity as Contact;
pub use group::Entity as Group;
pub use group_member::Entity as GroupMember;
pub use chat_message::Entity as ChatMessage;
pub use chat_session::Entity as ChatSession;
pub use file_storage::Entity as FileStorage;

