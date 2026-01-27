// src-tauri/src/database/model/group.rs
//
//! SeaORM 实体模型 - 群组表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 群组表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "group")]
pub struct Model {
    /// 群组 ID
    #[sea_orm(primary_key)]
    pub gid: i64,

    /// 群名称
    #[sea_orm(column_type = "Text")]
    pub group_name: String,

    /// 头像 URL
    #[sea_orm(column_type = "Text", nullable)]
    pub avatar: Option<String>,

    /// 创建者用户 ID
    pub creator_uid: i64,

    /// 群描述
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    /// 创建时间
    pub create_time: DateTime,

    /// 更新时间
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
