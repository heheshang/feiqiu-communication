// src-tauri/src/database/mod.rs
//
pub mod handler;
pub mod migration;
/// 数据库访问层模块
pub mod model;

use crate::error::{AppError, AppResult};
use sea_orm::{Database, DbConn};
use sea_orm_migration::MigratorTrait;

/// 初始化数据库连接
pub async fn init_database(db_path: Option<&str>) -> AppResult<DbConn> {
    let db_url = if let Some(path) = db_path {
        // 将反斜杠转换为正斜杠
        let normalized_path = path.replace('\\', "/");

        // SQLite URL 格式:
        // - Windows 绝对路径: sqlite:C:/path/to/file.db (不带 ///)
        // - Unix 绝对路径: sqlite:/path/to/file.db
        // - 相对路径: sqlite:./file.db 或 sqlite://./file.db
        if normalized_path.contains(":") && normalized_path.chars().next().unwrap().is_ascii_alphabetic() {
            // Windows 绝对路径，如 C:/Users/... 格式为 sqlite:C:/Users/...
            format!("sqlite:{}", normalized_path)
        } else {
            // 相对路径或 Unix 路径
            format!("sqlite:{}", normalized_path)
        }
    } else {
        // 默认使用当前目录下的 feiqiu.db
        "sqlite:./feiqiu.db".to_string()
    };

    tracing::info!("正在连接数据库: {}", db_url);

    let db = Database::connect(&db_url).await.map_err(|e| AppError::Database(e))?;

    // // 创建数据库表（使用原生 SQL）
    // create_tables(&db).await?;

    tracing::info!("正在运行数据库迁移...");
    crate::database::migration::Migrator::up(&db, None).await.map_err(|e| {
        tracing::error!("数据库迁移失败: {}", e);
        AppError::Database(e)
    })?;
    tracing::info!("数据库迁移完成");

    tracing::info!("数据库初始化完成");

    Ok(db)
}
