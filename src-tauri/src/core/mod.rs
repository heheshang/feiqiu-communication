// src-tauri/src/core/mod.rs
//
/// 核心业务逻辑层
pub mod chat;
pub mod contact;
pub mod file;
pub mod group;

pub use group::GroupBroadcaster;
