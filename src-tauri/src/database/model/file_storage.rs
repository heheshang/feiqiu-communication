// src-tauri/src/database/model/file_storage.rs
//
//! SeaORM 实体模型 - 文件存储表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 文件存储表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "file_storage")]
pub struct Model {
    /// 文件 ID
    #[sea_orm(primary_key)]
    pub fid: i64,

    /// 文件名
    #[sea_orm(column_type = "Text")]
    pub file_name: String,

    /// 文件路径
    #[sea_orm(column_type = "Text")]
    pub file_path: String,

    /// 文件大小 (字节)
    pub file_size: i64,

    /// 文件类型 (MIME)
    #[sea_orm(column_type = "Text")]
    pub file_type: String,

    /// 上传者用户 ID
    pub uploader_uid: i64,

    /// 上传时间
    #[sea_orm(column_type = "Text")]
    pub upload_time: String,

    /// 创建时间
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UploaderUid",
        to = "super::user::Column::Uid"
    )]
    Uploader,
}

impl ActiveModelBehavior for ActiveModel {}
