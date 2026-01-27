// src-tauri/src/error.rs
//
/// 统一错误定义

use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    Database(#[from] sea_orm::DbErr),

    /// 网络错误
    #[error("网络错误: {0}")]
    Network(String),

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// 业务逻辑错误
    #[error("业务错误: {0}")]
    Business(String),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialize(String),

    /// 协议解析错误
    #[error("协议解析错误: {0}")]
    Protocol(String),

    /// 未找到
    #[error("未找到: {0}")]
    NotFound(String),

    /// 已存在
    #[error("记录已存在: {0}")]
    AlreadyExists(String),
}

/// 应用结果类型
pub type AppResult<T> = Result<T, AppError>;

/// 转换为 IPC 返回的 String
impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}
