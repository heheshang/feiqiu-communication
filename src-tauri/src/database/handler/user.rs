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

        let result = User::insert(new_user).exec(db).await.map_err(|e| AppError::Database(e))?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找用户
    pub async fn find_by_id(db: &DbConn, uid: i64) -> AppResult<user::Model> {
        let user = User::find_by_id(uid)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?
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
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
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

        user_update.update(db).await.map_err(|e| AppError::Database(e))
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

        user_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 删除用户
    pub async fn delete(db: &DbConn, uid: i64) -> AppResult<()> {
        User::delete_by_id(uid).exec(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 获取所有用户
    pub async fn list_all(db: &DbConn) -> AppResult<Vec<user::Model>> {
        let users = User::find().all(db).await.map_err(|e| AppError::Database(e))?;

        Ok(users)
    }

    /// 根据状态查找用户
    pub async fn find_by_status(db: &DbConn, status: i8) -> AppResult<Vec<user::Model>> {
        let users = User::find()
            .filter(user::Column::Status.eq(status))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(users)
    }
}
