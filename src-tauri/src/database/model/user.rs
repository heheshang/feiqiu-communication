// src-tauri/src/database/model/user.rs
//
//! SeaORM 实体模型 - 用户表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 用户表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    /// 用户 ID (雪花算法生成)
    #[sea_orm(primary_key)]
    pub uid: i64,

    /// 飞秋 IP 地址
    #[sea_orm(column_type = "Text")]
    pub feiq_ip: String,

    /// 飞秋端口
    pub feiq_port: u16,

    /// 飞秋机器 ID
    #[sea_orm(column_type = "Text")]
    pub feiq_machine_id: String,

    /// 昵称
    #[sea_orm(column_type = "Text")]
    pub nickname: String,

    /// 头像 URL
    #[sea_orm(column_type = "Text", nullable)]
    pub avatar: Option<String>,

    /// 在线状态 (0-离线, 1-在线, 2-忙碌)
    pub status: i8,

    /// 创建时间
    pub create_time: DateTime,

    /// 更新时间
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
