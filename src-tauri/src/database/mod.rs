// src-tauri/src/database/mod.rs
//
pub mod handler;
pub mod migration;
/// 数据库访问层模块
pub mod model;

use crate::error::{AppError, AppResult};
use sea_orm::{Database, DbConn, ConnectionTrait, Statement};
use sea_orm_migration::MigratorTrait;

/// SQLite 性能优化配置
async fn optimize_sqlite(db: &DbConn) -> AppResult<()> {
    // 启用 WAL 模式 - 提高并发性能
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "PRAGMA journal_mode = WAL;".to_string()
    )).await.map_err(AppError::Database)?;

    // 同步模式设为 NORMAL - 平衡性能和数据安全
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "PRAGMA synchronous = NORMAL;".to_string()
    )).await.map_err(AppError::Database)?;

    // 增加缓存大小到 10MB
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "PRAGMA cache_size = -10000;".to_string()
    )).await.map_err(AppError::Database)?;

    // 启用内存映射 I/O
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "PRAGMA mmap_size = 30000000000;".to_string()
    )).await.map_err(AppError::Database)?;

    // 临时表使用内存存储
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "PRAGMA temp_store = MEMORY;".to_string()
    )).await.map_err(AppError::Database)?;

    tracing::info!("SQLite 性能优化配置完成");
    Ok(())
}

/// 初始化数据库连接
pub async fn init_database(db_path: Option<&str>) -> AppResult<DbConn> {
    let db_url = if let Some(path) = db_path {
        let normalized_path = path.replace('\\', "/");
        if normalized_path.contains(":") && normalized_path.chars().next().unwrap().is_ascii_alphabetic() {
            format!("sqlite:{}", normalized_path)
        } else {
            format!("sqlite:{}", normalized_path)
        }
    } else {
        "sqlite:./feiqiu.db".to_string()
    };

    tracing::info!("正在连接数据库: {}", db_url);

    let db = Database::connect(&db_url).await.map_err(AppError::Database)?;

    // 应用性能优化
    optimize_sqlite(&db).await?;

    tracing::info!("正在运行数据库迁移...");
    crate::database::migration::Migrator::up(&db, None).await.map_err(|e| {
        tracing::error!("数据库迁移失败: {}", e);
        AppError::Database(e)
    })?;
    tracing::info!("数据库迁移完成");

    tracing::info!("数据库初始化完成");

    Ok(db)
}
