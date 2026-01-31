// src-tauri/src/database/handler/transfer_state.rs
//
//! 文件传输状态数据库操作

use crate::database::model::transfer_state::{ActiveModel, Column, Entity, Model};
use sea_orm::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferStateError {
    #[error("Database error: {0}")]
    DbError(#[from] DbErr),

    #[error("Transfer not found: {0}")]
    NotFound(i64),
}

pub type Result<T> = std::result::Result<T, TransferStateError>;

/// 文件传输状态处理器
pub struct TransferStateHandler;

impl TransferStateHandler {
    /// 创建新的传输状态记录
    pub async fn create(db: &DbConn, model: ActiveModel) -> Result<Model> {
        let result = model.insert(db).await?;
        Ok(result)
    }

    /// 根据 ID 查找传输状态
    pub async fn find_by_id(db: &DbConn, tid: i64) -> Result<Option<Model>> {
        let result = Entity::find_by_id(tid).one(db).await?;
        Ok(result)
    }

    /// 根据文件 ID 查找传输状态
    pub async fn find_by_file_id(db: &DbConn, file_id: i64) -> Result<Option<Model>> {
        let result = Entity::find().filter(Column::FileId.eq(file_id)).one(db).await?;
        Ok(result)
    }

    /// 根据数据包编号查找传输状态
    pub async fn find_by_packet_no(db: &DbConn, packet_no: &str) -> Result<Vec<Model>> {
        let result = Entity::find().filter(Column::PacketNo.eq(packet_no)).all(db).await?;
        Ok(result)
    }

    /// 查找所有未完成的传输
    pub async fn find_pending(db: &DbConn) -> Result<Vec<Model>> {
        // 查找状态为 0 (等待中) 或 1 (传输中) 的记录
        let pending_0 = Entity::find().filter(Column::Status.eq(0)).all(db).await?;

        let in_progress = Entity::find().filter(Column::Status.eq(1)).all(db).await?;

        // 合并两个结果
        let mut results = pending_0;
        results.extend(in_progress);

        Ok(results)
    }

    /// 更新传输进度
    pub async fn update_progress(db: &DbConn, tid: i64, transferred: i64, status: i8) -> Result<Model> {
        let transfer = Self::find_by_id(db, tid).await?.ok_or(TransferStateError::NotFound(tid))?;

        let mut active: ActiveModel = transfer.into();
        active.transferred = Set(transferred);
        active.status = Set(status);
        active.update_time = Set(chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string());

        let result = active.update(db).await?;
        Ok(result)
    }

    /// 更新传输状态
    pub async fn update_status(db: &DbConn, tid: i64, status: i8, error_message: Option<String>) -> Result<Model> {
        let transfer = Self::find_by_id(db, tid).await?.ok_or(TransferStateError::NotFound(tid))?;

        let mut active: ActiveModel = transfer.into();
        active.status = Set(status);
        active.update_time = Set(chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string());

        if let Some(msg) = error_message {
            active.error_message = Set(Some(msg));
        }

        let result = active.update(db).await?;
        Ok(result)
    }

    /// 删除传输记录
    pub async fn delete(db: &DbConn, tid: i64) -> Result<()> {
        Entity::delete_by_id(tid).exec(db).await?;
        Ok(())
    }

    /// 清理已完成的传输记录（超过指定天数）
    pub async fn cleanup_completed(db: &DbConn, days: i64) -> Result<u64> {
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days);

        let result = Entity::delete_many()
            .filter(Column::Status.eq(2)) // 已完成
            .filter(Column::UpdateTime.lt(cutoff_date.format("%Y-%m-%d %H:%M:%S").to_string()))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }
}
