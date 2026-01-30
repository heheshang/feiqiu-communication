//! 群组业务逻辑服务层
//!
//! GroupService 提供群组相关的业务逻辑操作，包括：
//! - 创建群组
//! - 获取群组列表
//! - 添加群成员
//! - 移除群成员
//! - 管理群组信息

use crate::database::handler::group::{GroupHandler, GroupMemberHandler};
use crate::error::AppResult;
use sea_orm::DbConn;
use tracing::{error, info};

/// 群组服务
pub struct GroupService;

impl GroupService {
    /// 创建群组
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `group_name`: 群组名称
    /// - `creator_uid`: 创建者用户ID
    /// - `desc`: 群组描述（可选）
    /// - `avatar`: 群组头像（可选）
    ///
    /// # 返回
    /// 返回新创建的群组ID
    pub async fn create_group(
        db: &DbConn,
        group_name: String,
        creator_uid: i64,
        desc: Option<String>,
        avatar: Option<String>,
    ) -> AppResult<i64> {
        // 1. 创建群组
        let group = GroupHandler::create(db, group_name.clone(), creator_uid, desc)
            .await
            .map_err(|e| {
                error!("创建群组失败: {}", e);
                e
            })?;

        let gid = group.gid;
        info!("群组已创建: gid={}, name={}", gid, group_name);

        // 2. 如果提供了头像，更新群组头像
        if let Some(avatar_url) = avatar {
            GroupHandler::update(db, gid, None, Some(avatar_url), None)
                .await
                .map_err(|e| {
                    error!("更新群组头像失败: {}", e);
                    e
                })?;
        }

        Ok(gid)
    }

    /// 获取用户的群组列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 用户ID
    ///
    /// # 返回
    /// 返回群组列表
    pub async fn get_groups(db: &DbConn, owner_uid: i64) -> AppResult<Vec<crate::database::model::group::Model>> {
        // 1. 获取用户的群组成员记录
        let memberships = GroupMemberHandler::list_by_member(db, owner_uid)
            .await
            .map_err(|e| {
                error!("获取用户群组成员记录失败: {}", e);
                e
            })?;

        // 2. 为每个成员记录获取群组信息
        let mut groups = Vec::new();
        for membership in memberships {
            match GroupHandler::find_by_id(db, membership.gid).await {
                Ok(group) => groups.push(group),
                Err(e) => {
                    error!("获取群组信息失败: gid={}, error={}", membership.gid, e);
                    // 继续处理其他群组，不中断
                }
            }
        }

        info!("获取用户群组列表: uid={}, count={}", owner_uid, groups.len());
        Ok(groups)
    }

    /// 添加群成员
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `gid`: 群组ID
    /// - `member_uid`: 成员用户ID
    /// - `nickname`: 成员昵称
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn add_member(db: &DbConn, gid: i64, member_uid: i64, nickname: String) -> AppResult<()> {
        // 1. 添加群成员（role=0 表示普通成员）
        GroupMemberHandler::add_member(db, gid, member_uid, 0)
            .await
            .map_err(|e| {
                error!("添加群成员失败: gid={}, member_uid={}, error={}", gid, member_uid, e);
                e
            })?;

        info!("群成员已添加: gid={}, member_uid={}, nickname={}", gid, member_uid, nickname);
        Ok(())
    }

    /// 移除群成员
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `gid`: 群组ID
    /// - `member_uid`: 成员用户ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn remove_member(db: &DbConn, gid: i64, member_uid: i64) -> AppResult<()> {
        // 1. 移除群成员
        GroupMemberHandler::remove_member(db, gid, member_uid)
            .await
            .map_err(|e| {
                error!("移除群成员失败: gid={}, member_uid={}, error={}", gid, member_uid, e);
                e
            })?;

        info!("群成员已移除: gid={}, member_uid={}", gid, member_uid);
        Ok(())
    }

    /// 获取群组成员列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `gid`: 群组ID
    ///
    /// # 返回
    /// 返回群组成员列表
    pub async fn get_members(db: &DbConn, gid: i64) -> AppResult<Vec<crate::database::model::group_member::Model>> {
        // 1. 获取群组成员列表
        let members = GroupMemberHandler::list_by_group(db, gid)
            .await
            .map_err(|e| {
                error!("获取群组成员列表失败: gid={}, error={}", gid, e);
                e
            })?;

        info!("获取群组成员列表: gid={}, count={}", gid, members.len());
        Ok(members)
    }

    /// 更新群组信息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `gid`: 群组ID
    /// - `group_name`: 群组名称（可选）
    /// - `desc`: 群组描述（可选）
    /// - `avatar`: 群组头像（可选）
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn update_group(
        db: &DbConn,
        gid: i64,
        group_name: Option<String>,
        desc: Option<String>,
        avatar: Option<String>,
    ) -> AppResult<()> {
        // 1. 更新群组信息
        GroupHandler::update(db, gid, group_name.clone(), avatar, desc)
            .await
            .map_err(|e| {
                error!("更新群组信息失败: gid={}, error={}", gid, e);
                e
            })?;

        info!("群组信息已更新: gid={}", gid);
        Ok(())
    }

    /// 删除群组
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `gid`: 群组ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn delete_group(db: &DbConn, gid: i64) -> AppResult<()> {
        // 1. 删除群组
        GroupHandler::delete(db, gid)
            .await
            .map_err(|e| {
                error!("删除群组失败: gid={}, error={}", gid, e);
                e
            })?;

        info!("群组已删除: gid={}", gid);
        Ok(())
    }
}
