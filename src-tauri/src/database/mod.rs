// src-tauri/src/database/mod.rs
//
/// 数据库访问层模块

pub mod model;
// pub mod migration; // TODO: Fix migration API issues
pub mod handler;

use sea_orm::{Database, DbConn};
use crate::error::{AppError, AppResult};

/// 初始化数据库连接
pub async fn init_database() -> AppResult<DbConn> {
    let db_url = "sqlite://./feiqiu.db";

    tracing::info!("正在连接数据库: {}", db_url);

    let db = Database::connect(db_url)
        .await
        .map_err(|e| AppError::Database(e))?;

    // TODO: 运行数据库迁移
    // migration::run_migrations(&db).await?;

    tracing::info!("数据库初始化完成");

    Ok(db)
}
