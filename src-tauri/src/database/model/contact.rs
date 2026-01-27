// src-tauri/src/database/model/contact.rs
//
//! SeaORM 实体模型 - 联系人表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 联系人表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contact")]
pub struct Model {
    /// 联系人 ID
    #[sea_orm(primary_key)]
    pub id: i64,

    /// 所有者用户 ID
    pub owner_uid: i64,

    /// 联系人用户 ID
    pub contact_uid: i64,

    /// 备注
    #[sea_orm(column_type = "Text", nullable)]
    pub remark: Option<String>,

    /// 分组标签
    #[sea_orm(column_type = "Text", nullable)]
    pub tag: Option<String>,

    /// 创建时间
    pub create_time: DateTime,

    /// 更新时间
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ContactUid",
        to = "super::user::Column::Uid"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
