// src-tauri/src/database/handler/file.rs
//
//! 文件存储 CRUD 操作

use crate::database::model::{file_storage, FileStorage};
use crate::error::{AppError, AppResult};
use sea_orm::*;
use serde::{Deserialize, Serialize};

/// 文件存储处理器
pub struct FileStorageHandler;

impl FileStorageHandler {
    /// 记录文件上传
    pub async fn create(
        db: &DbConn,
        file_name: String,
        file_path: String,
        file_size: i64,
        file_type: String,
        uploader_uid: i64,
    ) -> AppResult<file_storage::Model> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let new_file = file_storage::ActiveModel {
            fid: ActiveValue::NotSet,
            file_name: ActiveValue::Set(file_name),
            file_path: ActiveValue::Set(file_path),
            file_size: ActiveValue::Set(file_size),
            file_type: ActiveValue::Set(file_type),
            uploader_uid: ActiveValue::Set(uploader_uid),
            upload_time: ActiveValue::Set(now),
            create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let result = FileStorage::insert(new_file)
            .exec(db)
            .await
            .map_err(AppError::Database)?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找文件
    pub async fn find_by_id(db: &DbConn, fid: i64) -> AppResult<file_storage::Model> {
        let file = FileStorage::find_by_id(fid)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound(format!("文件 {} 不存在", fid)))?;

        Ok(file)
    }

    /// 根据上传者 ID 查找文件
    pub async fn list_by_uploader(db: &DbConn, uploader_uid: i64) -> AppResult<Vec<file_storage::Model>> {
        let files = FileStorage::find()
            .filter(file_storage::Column::UploaderUid.eq(uploader_uid))
            .order_by_desc(file_storage::Column::UploadTime)
            .all(db)
            .await
            .map_err(AppError::Database)?;

        Ok(files)
    }

    /// 根据文件类型查找文件
    pub async fn list_by_type(db: &DbConn, file_type: &str) -> AppResult<Vec<file_storage::Model>> {
        let files = FileStorage::find()
            .filter(file_storage::Column::FileType.eq(file_type))
            .order_by_desc(file_storage::Column::UploadTime)
            .all(db)
            .await
            .map_err(AppError::Database)?;

        Ok(files)
    }

    /// 删除文件记录
    pub async fn delete(db: &DbConn, fid: i64) -> AppResult<()> {
        FileStorage::delete_by_id(fid)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    /// 获取文件统计信息
    pub async fn get_stats_by_uploader(db: &DbConn, uploader_uid: i64) -> AppResult<FileStats> {
        let files = Self::list_by_uploader(db, uploader_uid).await?;

        let total_count = files.len() as i64;
        let total_size: i64 = files.iter().map(|f| f.file_size).sum();

        // 按类型统计
        let mut type_counts: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for file in &files {
            *type_counts.entry(file.file_type.clone()).or_insert(0) += 1;
        }

        Ok(FileStats {
            total_count,
            total_size,
            type_counts,
        })
    }
}

/// 文件统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub total_count: i64,
    pub total_size: i64,
    pub type_counts: std::collections::HashMap<String, i64>,
}
