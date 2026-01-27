// src-tauri/src/database/model/chat_session.rs
//
//! SeaORM 实体模型 - 聊天会话表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 聊天会话表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_session")]
pub struct Model {
    /// 会话 ID
    #[sea_orm(primary_key)]
    pub sid: i64,

    /// 所有者用户 ID
    pub owner_uid: i64,

    /// 会话类型 (0-单聊, 1-群聊)
    pub session_type: i8,

    /// 目标 ID (单聊时为用户ID, 群聊时为群组ID)
    pub target_id: i64,

    /// 最后一条消息 ID
    #[sea_orm(nullable)]
    pub last_msg_id: Option<i64>,

    /// 未读消息数量
    pub unread_count: i32,

    /// 更新时间
    pub update_time: DateTime,

    /// 创建时间
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerUid",
        to = "super::user::Column::Uid"
    )]
    Owner,

    #[sea_orm(
        belongs_to = "super::chat_message::Entity",
        from = "Column::LastMsgId",
        to = "super::chat_message::Column::Mid"
    )]
    LastMessage,
}

impl ActiveModelBehavior for ActiveModel {}
