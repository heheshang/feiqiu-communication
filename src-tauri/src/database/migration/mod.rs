// src-tauri/src/database/migration/mod.rs
//
//! 数据库迁移脚本模块
//!
//! 使用 SeaORM 迁移系统管理数据库版本和表结构

pub mod m20250127_000001_create_user_table;
pub mod m20250127_000002_create_contact_table;
pub mod m20250127_000003_create_group_tables;
pub mod m20250127_000004_create_chat_tables;
pub mod m20250127_000005_create_file_storage_table;
pub mod m20250129_000006_create_transfer_state_table;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250127_000001_create_user_table::Migration),
            Box::new(m20250127_000002_create_contact_table::Migration),
            Box::new(m20250127_000003_create_group_tables::Migration),
            Box::new(m20250127_000004_create_chat_tables::Migration),
            Box::new(m20250127_000005_create_file_storage_table::Migration),
            Box::new(m20250129_000006_create_transfer_state_table::Migration),
        ]
    }
}
