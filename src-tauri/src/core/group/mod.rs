// src-tauri/src/core/group/mod.rs
//
/// 群组相关业务逻辑
pub mod broadcast;
pub mod service;

pub use broadcast::GroupBroadcaster;
pub use service::GroupService;
