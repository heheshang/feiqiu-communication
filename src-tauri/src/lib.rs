// src-tauri/src/lib.rs
//
// 飞秋通讯 - 库入口

pub mod core;
pub mod database;
pub mod error;
pub mod event;
pub mod ipc;
pub mod network;
pub mod types;
pub mod utils;

// 重新导出常用类型
pub use error::{AppError, AppResult};
pub use types::*;
