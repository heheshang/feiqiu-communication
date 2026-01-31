// src-tauri/src/core/contact/mod.rs
//
/// 联系人管理模块
pub mod discovery;
pub mod service;

pub use discovery::start_discovery;
pub use service::ContactService;
