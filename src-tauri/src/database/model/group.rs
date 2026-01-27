// src-tauri/src/database/model/group.rs
//
//! SeaORM 实体模型 - 群组相关表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================
// 群组表
// ============================================================

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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatorUid",
        to = "super::user::Column::Uid"
    )]
    Creator,
}

impl ActiveModelBehavior for ActiveModel {}

// ============================================================
// 群成员表
// ============================================================

/// 群成员表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "group_member")]
pub struct GroupMemberModel {
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
pub enum GroupMemberRelation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "GroupMemberColumn::Gid",
        to = "Column::Gid"
    )]
    Group,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "GroupMemberColumn::MemberUid",
        to = "super::user::Column::Uid"
    )]
    User,
}

impl ActiveModelBehavior for GroupMemberActiveModel {}
