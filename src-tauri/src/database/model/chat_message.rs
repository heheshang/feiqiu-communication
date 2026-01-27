// src-tauri/src/database/model/chat_message.rs
//
//! SeaORM 实体模型 - 聊天消息表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 聊天消息表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_message")]
pub struct Model {
    /// 消息 ID
    #[sea_orm(primary_key)]
    pub mid: i64,

    /// 会话类型 (0-单聊, 1-群聊)
    pub session_type: i8,

    /// 目标 ID (单聊时为用户ID, 群聊时为群组ID)
    pub target_id: i64,

    /// 发送者用户 ID
    pub sender_uid: i64,

    /// 消息类型 (0-文字, 1-文件, 2-Emoji)
    pub msg_type: i8,

    /// 消息内容
    #[sea_orm(column_type = "Text")]
    pub content: String,

    /// 发送时间
    #[sea_orm(column_type = "Text")]
    pub send_time: String,

    /// 消息状态 (0-发送中, 1-已发送, 2-已读, -1-失败)
    pub status: i8,

    /// 创建时间
    pub create_time: DateTime,

    /// 更新时间
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SenderUid",
        to = "super::user::Column::Uid"
    )]
    Sender,
}

impl ActiveModelBehavior for ActiveModel {}
