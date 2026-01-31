// src-tauri/src/core/file/mod.rs
//
//! 文件传输核心业务逻辑

pub mod request;
pub mod resume;
pub mod service;
pub mod transfer;

pub use service::FileService;
