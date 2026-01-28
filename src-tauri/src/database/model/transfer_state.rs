// src-tauri/src/database/model/transfer_state.rs
//
//! SeaORM 实体模型 - 文件传输状态表

use sea_orm::entity::prelude::*;

/// 文件传输状态表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_state")]
pub struct Model {
    /// 传输 ID
    #[sea_orm(primary_key)]
    pub tid: i64,

    /// 文件 ID (关联 file_storage 表)
    pub file_id: i64,

    /// 会话类型 (0=单聊, 1=群聊)
    pub session_type: i8,

    /// 目标 ID
    pub target_id: i64,

    /// 传输方向 (0=下载, 1=上传)
    pub direction: i8,

    /// 已传输字节数
    pub transferred: i64,

    /// 文件总大小
    pub file_size: i64,

    /// 传输状态 (0=等待中, 1=传输中, 2=已完成, -1=失败, -2=已取消)
    pub status: i8,

    /// 数据包编号 (用于恢复传输)
    pub packet_no: String,

    /// 目标 IP 地址
    pub target_ip: String,

    /// 目标端口
    pub target_port: u16,

    /// SHA256 校验和
    pub checksum: String,

    /// 错误信息 (失败时记录)
    pub error_message: Option<String>,

    /// 更新时间
    pub update_time: String,

    /// 创建时间
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file_storage::Entity",
        from = "Column::FileId",
        to = "super::file_storage::Column::Fid"
    )]
    File,
}

impl ActiveModelBehavior for ActiveModel {}
