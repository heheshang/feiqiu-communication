// src-tauri/src/database/handler/user.rs
//
//! 用户表 CRUD 操作

use crate::database::model::{user, User};
use crate::error::{AppError, AppResult};
use sea_orm::*;

/// 用户处理器
pub struct UserHandler;

impl UserHandler {
    /// 创建新用户
    pub async fn create(db: &DbConn, user_data: user::Model) -> AppResult<user::Model> {
        let new_user = user::ActiveModel {
            uid: ActiveValue::NotSet,
            feiq_ip: ActiveValue::Set(user_data.feiq_ip),
            feiq_port: ActiveValue::Set(user_data.feiq_port),
            feiq_machine_id: ActiveValue::Set(user_data.feiq_machine_id),
            nickname: ActiveValue::Set(user_data.nickname),
            avatar: ActiveValue::Set(user_data.avatar),
            status: ActiveValue::Set(user_data.status),
            create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let result = User::insert(new_user).exec(db).await.map_err(AppError::Database)?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找用户
    pub async fn find_by_id(db: &DbConn, uid: i64) -> AppResult<user::Model> {
        let user = User::find_by_id(uid)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", uid)))?;

        Ok(user)
    }

    /// 根据 IP 和端口查找用户
    pub async fn find_by_ip_port(db: &DbConn, ip: &str, port: u16) -> AppResult<Option<user::Model>> {
        let user = User::find()
            .filter(user::Column::FeiqIp.eq(ip))
            .filter(user::Column::FeiqPort.eq(port as i32))
            .one(db)
            .await
            .map_err(AppError::Database)?;

        Ok(user)
    }

    /// 获取当前用户（本地用户）
    ///
    /// 返回数据库中的第一个用户作为当前用户
    /// 通常只有一个本地用户记录
    pub async fn get_current_user(db: &DbConn) -> AppResult<user::Model> {
        let user = User::find()
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound("未找到当前用户".to_string()))?;

        Ok(user)
    }

    /// 获取当前用户 ID
    ///
    /// 便捷方法，返回当前用户的 uid
    pub async fn get_current_user_id(db: &DbConn) -> AppResult<i64> {
        let user = Self::get_current_user(db).await?;
        Ok(user.uid)
    }

    /// 更新用户信息
    pub async fn update(db: &DbConn, uid: i64, user_data: user::Model) -> AppResult<user::Model> {
        let existing_user = Self::find_by_id(db, uid).await?;

        let user_update = user::ActiveModel {
            uid: ActiveValue::Set(uid),
            feiq_ip: ActiveValue::Set(user_data.feiq_ip),
            feiq_port: ActiveValue::Set(user_data.feiq_port),
            feiq_machine_id: ActiveValue::Set(user_data.feiq_machine_id),
            nickname: ActiveValue::Set(user_data.nickname),
            avatar: ActiveValue::Set(user_data.avatar),
            status: ActiveValue::Set(user_data.status),
            create_time: ActiveValue::Set(existing_user.create_time),
            update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        user_update.update(db).await.map_err(AppError::Database)
    }

    /// 更新用户状态
    pub async fn update_status(db: &DbConn, uid: i64, status: i8) -> AppResult<()> {
        let existing_user = Self::find_by_id(db, uid).await?;

        let user_update = user::ActiveModel {
            uid: ActiveValue::Set(uid),
            feiq_ip: ActiveValue::Set(existing_user.feiq_ip),
            feiq_port: ActiveValue::Set(existing_user.feiq_port),
            feiq_machine_id: ActiveValue::Set(existing_user.feiq_machine_id),
            nickname: ActiveValue::Set(existing_user.nickname),
            avatar: ActiveValue::Set(existing_user.avatar),
            status: ActiveValue::Set(status),
            create_time: ActiveValue::Set(existing_user.create_time),
            update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        user_update.update(db).await.map_err(AppError::Database)?;
        Ok(())
    }

    /// 删除用户
    pub async fn delete(db: &DbConn, uid: i64) -> AppResult<()> {
        User::delete_by_id(uid).exec(db).await.map_err(AppError::Database)?;
        Ok(())
    }

    /// 获取所有用户
    pub async fn list_all(db: &DbConn) -> AppResult<Vec<user::Model>> {
        let users = User::find().all(db).await.map_err(AppError::Database)?;

        Ok(users)
    }

    /// 根据状态查找用户
    pub async fn find_by_status(db: &DbConn, status: i8) -> AppResult<Vec<user::Model>> {
        let users = User::find()
            .filter(user::Column::Status.eq(status))
            .all(db)
            .await
            .map_err(AppError::Database)?;

        Ok(users)
    }

    /// 根据 machine_id 查找用户
    pub async fn find_by_machine_id(db: &DbConn, machine_id: &str) -> AppResult<Option<user::Model>> {
        let user = User::find()
            .filter(user::Column::FeiqMachineId.eq(machine_id))
            .one(db)
            .await
            .map_err(AppError::Database)?;

        Ok(user)
    }

    /// 根据 machine_id 创建或更新用户（upsert）
    ///
    /// 如果用户不存在则创建，如果存在则更新信息
    pub async fn upsert_by_machine_id(
        db: &DbConn,
        machine_id: &str,
        feiq_ip: &str,
        feiq_port: u16,
        nickname: &str,
        status: i8,
    ) -> AppResult<user::Model> {
        match Self::find_by_machine_id(db, machine_id).await? {
            Some(existing_user) => {
                // 用户存在，更新信息
                let user_update = user::ActiveModel {
                    uid: ActiveValue::Set(existing_user.uid),
                    feiq_ip: ActiveValue::Set(feiq_ip.to_string()),
                    feiq_port: ActiveValue::Set(feiq_port),
                    feiq_machine_id: ActiveValue::Set(machine_id.to_string()),
                    nickname: ActiveValue::Set(nickname.to_string()),
                    avatar: ActiveValue::Set(existing_user.avatar),
                    status: ActiveValue::Set(status),
                    create_time: ActiveValue::Set(existing_user.create_time),
                    update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
                };

                user_update.update(db).await.map_err(AppError::Database)
            }
            None => {
                // 用户不存在，创建新用户
                let new_user = user::ActiveModel {
                    uid: ActiveValue::NotSet,
                    feiq_ip: ActiveValue::Set(feiq_ip.to_string()),
                    feiq_port: ActiveValue::Set(feiq_port),
                    feiq_machine_id: ActiveValue::Set(machine_id.to_string()),
                    nickname: ActiveValue::Set(nickname.to_string()),
                    avatar: ActiveValue::Set(None),
                    status: ActiveValue::Set(status),
                    create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
                    update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
                };

                let result = User::insert(new_user).exec(db).await.map_err(AppError::Database)?;
                Self::find_by_id(db, result.last_insert_id).await
            }
        }
    }
}
