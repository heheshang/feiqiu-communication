// src-tauri/src/database/handler/group.rs
//
//! 群组表 CRUD 操作

use sea_orm::*;
use crate::database::model::{group, group_member, Group, GroupMember};
use crate::error::{AppError, AppResult};

/// 群组处理器
pub struct GroupHandler;

impl GroupHandler {
    /// 创建群组
    pub async fn create(db: &DbConn, group_name: String, creator_uid: i64, description: Option<String>) -> AppResult<group::Model> {
        let new_group = group::ActiveModel {
            gid: ActiveValue::NotSet,
            group_name: ActiveValue::Set(group_name),
            avatar: ActiveValue::Set(None),
            creator_uid: ActiveValue::Set(creator_uid),
            description: ActiveValue::Set(description),
            create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let result = Group::insert(new_group)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 自动将创建者添加为群主
        GroupMemberHandler::add_member(db, result.last_insert_id, creator_uid, 1).await?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找群组
    pub async fn find_by_id(db: &DbConn, gid: i64) -> AppResult<group::Model> {
        let group = Group::find_by_id(gid)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?
            .ok_or_else(|| AppError::NotFound(format!("群组 {} 不存在", gid)))?;

        Ok(group)
    }

    /// 获取用户创建的所有群组
    pub async fn list_by_creator(db: &DbConn, creator_uid: i64) -> AppResult<Vec<group::Model>> {
        let groups = Group::find()
            .filter(group::Column::CreatorUid.eq(creator_uid))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(groups)
    }

    /// 更新群组信息
    pub async fn update(db: &DbConn, gid: i64, group_name: Option<String>, avatar: Option<String>, description: Option<String>) -> AppResult<group::Model> {
        let existing_group = Self::find_by_id(db, gid).await?;

        let mut group_update: group::ActiveModel = existing_group.into();

        if let Some(name) = group_name {
            group_update.group_name = ActiveValue::Set(name);
        }
        if let Some(av) = avatar {
            group_update.avatar = ActiveValue::Set(Some(av));
        }
        if let Some(desc) = description {
            group_update.description = ActiveValue::Set(Some(desc));
        }
        group_update.update_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        group_update.update(db).await.map_err(|e| AppError::Database(e))
    }

    /// 删除群组
    pub async fn delete(db: &DbConn, gid: i64) -> AppResult<()> {
        Group::delete_by_id(gid)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;
        Ok(())
    }
}

/// 群组成员处理器
pub struct GroupMemberHandler;

impl GroupMemberHandler {
    /// 添加群组成员
    pub async fn add_member(db: &DbConn, gid: i64, member_uid: i64, role: i8) -> AppResult<group_member::Model> {
        // 检查是否已存在
        if let Some(_) = Self::find_by_group_and_member(db, gid, member_uid).await? {
            return Err(AppError::AlreadyExists(format!("用户 {} 已在群组 {} 中", member_uid, gid)));
        }

        let new_member = group_member::ActiveModel {
            id: ActiveValue::NotSet,
            gid: ActiveValue::Set(gid),
            member_uid: ActiveValue::Set(member_uid),
            role: ActiveValue::Set(role),
            join_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let result = GroupMember::insert(new_member)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找群组成员
    pub async fn find_by_id(db: &DbConn, id: i64) -> AppResult<group_member::Model> {
        let member = GroupMember::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?
            .ok_or_else(|| AppError::NotFound(format!("群组成员记录 {} 不存在", id)))?;

        Ok(member)
    }

    /// 根据群组 ID 和成员 ID 查找
    pub async fn find_by_group_and_member(db: &DbConn, gid: i64, member_uid: i64) -> AppResult<Option<group_member::Model>> {
        let member = GroupMember::find()
            .filter(group_member::Column::Gid.eq(gid))
            .filter(group_member::Column::MemberUid.eq(member_uid))
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(member)
    }

    /// 获取群组的所有成员
    pub async fn list_by_group(db: &DbConn, gid: i64) -> AppResult<Vec<group_member::Model>> {
        let members = GroupMember::find()
            .filter(group_member::Column::Gid.eq(gid))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(members)
    }

    /// 获取用户加入的所有群组
    pub async fn list_by_member(db: &DbConn, member_uid: i64) -> AppResult<Vec<group_member::Model>> {
        let members = GroupMember::find()
            .filter(group_member::Column::MemberUid.eq(member_uid))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(members)
    }

    /// 更新成员角色
    pub async fn update_role(db: &DbConn, gid: i64, member_uid: i64, role: i8) -> AppResult<()> {
        let member = Self::find_by_group_and_member(db, gid, member_uid).await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {} 不在群组 {} 中", member_uid, gid)))?;

        let mut member_update: group_member::ActiveModel = member.into();
        member_update.role = ActiveValue::Set(role);

        member_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 移除群组成员
    pub async fn remove_member(db: &DbConn, gid: i64, member_uid: i64) -> AppResult<()> {
        let member = Self::find_by_group_and_member(db, gid, member_uid).await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {} 不在群组 {} 中", member_uid, gid)))?;

        GroupMember::delete_by_id(member.id)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;
        Ok(())
    }
}
