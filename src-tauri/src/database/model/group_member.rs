// src-tauri/src/database/model/group_member.rs
//
//! SeaORM 实体模型 - 群成员表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 群成员表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "group_member")]
pub struct Model {
    /// 成员 ID
    #[sea_orm(primary_key)]
    pub id: i64,

    /// 群组 ID
    pub gid: i64,

    /// 成员用户 ID
    pub member_uid: i64,

    /// 角色 (0-普通成员, 1-管理员, 2-群主)
    pub role: i8,

    /// 加入时间
    pub join_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::Gid",
        to = "super::group::Column::Gid"
    )]
    Group,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::MemberUid",
        to = "super::user::Column::Uid"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
