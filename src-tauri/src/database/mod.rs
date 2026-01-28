// src-tauri/src/database/mod.rs
//
/// 数据库访问层模块
pub mod model;
// pub mod migration; // TODO: Fix migration API issues
pub mod handler;

use crate::error::{AppError, AppResult};
use sea_orm::{ConnectionTrait, Database, DbConn, Statement};

/// 初始化数据库连接
pub async fn init_database() -> AppResult<DbConn> {
    let db_url = "sqlite://./feiqiu.db";

    tracing::info!("正在连接数据库: {}", db_url);

    let db = Database::connect(db_url).await.map_err(|e| AppError::Database(e))?;

    // 创建数据库表
    create_tables(&db).await?;

    tracing::info!("数据库初始化完成");

    Ok(db)
}

/// 创建数据库表
async fn create_tables(db: &DbConn) -> AppResult<()> {
    tracing::info!("正在创建数据库表...");

    // 创建用户表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS user (
            uid INTEGER PRIMARY KEY AUTOINCREMENT,
            feiq_ip TEXT NOT NULL,
            feiq_port INTEGER NOT NULL,
            feiq_machine_id TEXT NOT NULL,
            nickname TEXT NOT NULL,
            avatar TEXT,
            status INTEGER NOT NULL DEFAULT 0,
            create_time TEXT NOT NULL,
            update_time TEXT NOT NULL
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建联系人表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS contact (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            owner_uid INTEGER NOT NULL,
            contact_uid INTEGER NOT NULL,
            remark TEXT,
            tag TEXT,
            create_time TEXT NOT NULL,
            update_time TEXT NOT NULL,
            FOREIGN KEY (owner_uid) REFERENCES user(uid) ON DELETE CASCADE ON UPDATE CASCADE,
            FOREIGN KEY (contact_uid) REFERENCES user(uid) ON DELETE CASCADE ON UPDATE CASCADE
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建群组表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS group_table (
            gid INTEGER PRIMARY KEY AUTOINCREMENT,
            group_name TEXT NOT NULL,
            avatar TEXT,
            creator_uid INTEGER NOT NULL,
            description TEXT,
            create_time TEXT NOT NULL,
            update_time TEXT NOT NULL,
            FOREIGN KEY (creator_uid) REFERENCES user(uid) ON DELETE CASCADE ON UPDATE CASCADE
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建群组成员表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS group_member (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            gid INTEGER NOT NULL,
            member_uid INTEGER NOT NULL,
            role INTEGER NOT NULL DEFAULT 0,
            join_time TEXT NOT NULL,
            FOREIGN KEY (gid) REFERENCES group_table(gid) ON DELETE CASCADE ON UPDATE CASCADE,
            FOREIGN KEY (member_uid) REFERENCES user(uid) ON DELETE CASCADE ON UPDATE CASCADE,
            UNIQUE(gid, member_uid)
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建聊天消息表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS chat_message (
            mid INTEGER PRIMARY KEY AUTOINCREMENT,
            session_type INTEGER NOT NULL,
            target_id INTEGER NOT NULL,
            sender_uid INTEGER NOT NULL,
            msg_type INTEGER NOT NULL,
            content TEXT NOT NULL,
            send_time TEXT NOT NULL,
            status INTEGER NOT NULL DEFAULT 0,
            create_time TEXT NOT NULL,
            update_time TEXT NOT NULL,
            FOREIGN KEY (sender_uid) REFERENCES user(uid) ON DELETE CASCADE ON UPDATE CASCADE
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建聊天会话表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS chat_session (
            sid INTEGER PRIMARY KEY AUTOINCREMENT,
            owner_uid INTEGER NOT NULL,
            session_type INTEGER NOT NULL,
            target_id INTEGER NOT NULL,
            last_msg_id INTEGER,
            unread_count INTEGER NOT NULL DEFAULT 0,
            update_time TEXT NOT NULL,
            create_time TEXT NOT NULL,
            FOREIGN KEY (owner_uid) REFERENCES user(uid) ON DELETE CASCADE ON UPDATE CASCADE,
            FOREIGN KEY (last_msg_id) REFERENCES chat_message(mid) ON DELETE SET NULL ON UPDATE CASCADE
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建文件存储表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS file_storage (
            fid INTEGER PRIMARY KEY AUTOINCREMENT,
            file_name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            file_type TEXT NOT NULL,
            uploader_uid INTEGER NOT NULL,
            upload_time TEXT NOT NULL,
            create_time TEXT NOT NULL,
            FOREIGN KEY (uploader_uid) REFERENCES user(uid) ON DELETE CASCADE ON UPDATE CASCADE
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建文件传输状态表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS transfer_state (
            tid INTEGER PRIMARY KEY AUTOINCREMENT,
            file_id INTEGER NOT NULL,
            session_type INTEGER NOT NULL,
            target_id INTEGER NOT NULL,
            direction INTEGER NOT NULL,
            transferred INTEGER NOT NULL DEFAULT 0,
            file_size INTEGER NOT NULL,
            status INTEGER NOT NULL DEFAULT 0,
            packet_no TEXT NOT NULL,
            target_ip TEXT NOT NULL,
            target_port INTEGER NOT NULL,
            checksum TEXT NOT NULL,
            error_message TEXT,
            update_time TEXT NOT NULL,
            create_time TEXT NOT NULL,
            FOREIGN KEY (file_id) REFERENCES file_storage(fid) ON DELETE CASCADE ON UPDATE CASCADE
        )
        "#
        .to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    // 创建索引
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "CREATE INDEX IF NOT EXISTS idx_contact_owner ON contact(owner_uid)".to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "CREATE INDEX IF NOT EXISTS idx_group_member_gid_uid ON group_member(gid, member_uid)".to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "CREATE INDEX IF NOT EXISTS idx_chat_message_sender ON chat_message(sender_uid)".to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "CREATE INDEX IF NOT EXISTS idx_chat_session_owner_target ON chat_session(owner_uid, target_id)".to_string(),
    ))
    .await
    .map_err(|e| AppError::Database(e))?;

    tracing::info!("数据库表创建完成");

    Ok(())
}
