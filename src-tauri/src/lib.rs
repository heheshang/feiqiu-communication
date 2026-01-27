// src-tauri/src/lib.rs
//
// 飞秋通讯 - 库入口

pub mod error;
pub mod types;
pub mod utils;
pub mod event;
pub mod network;
pub mod database;
pub mod ipc;

// 重新导出常用类型
pub use error::{AppError, AppResult};
pub use types::*;
