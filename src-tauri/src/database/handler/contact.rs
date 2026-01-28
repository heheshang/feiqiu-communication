// src-tauri/src/database/handler/contact.rs
//
//! 联系人表 CRUD 操作

use crate::database::model::{contact, Contact};
use crate::error::{AppError, AppResult};
use sea_orm::*;

/// 联系人处理器
pub struct ContactHandler;

impl ContactHandler {
    /// 添加联系人
    pub async fn create(
        db: &DbConn,
        owner_uid: i64,
        contact_uid: i64,
        remark: Option<String>,
        tag: Option<String>,
    ) -> AppResult<contact::Model> {
        // 检查是否已存在
        if let Some(_) = Self::find_by_owner_and_contact(db, owner_uid, contact_uid).await? {
            return Err(AppError::AlreadyExists(format!(
                "联系人关系已存在: {} -> {}",
                owner_uid, contact_uid
            )));
        }

        let new_contact = contact::ActiveModel {
            id: ActiveValue::NotSet,
            owner_uid: ActiveValue::Set(owner_uid),
            contact_uid: ActiveValue::Set(contact_uid),
            remark: ActiveValue::Set(remark),
            tag: ActiveValue::Set(tag),
            create_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            update_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let result = Contact::insert(new_contact).exec(db).await.map_err(|e| AppError::Database(e))?;

        Self::find_by_id(db, result.last_insert_id).await
    }

    /// 根据 ID 查找联系人
    pub async fn find_by_id(db: &DbConn, id: i64) -> AppResult<contact::Model> {
        let contact = Contact::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?
            .ok_or_else(|| AppError::NotFound(format!("联系人记录 {} 不存在", id)))?;

        Ok(contact)
    }

    /// 根据所有者 ID 和联系人 ID 查找
    pub async fn find_by_owner_and_contact(
        db: &DbConn,
        owner_uid: i64,
        contact_uid: i64,
    ) -> AppResult<Option<contact::Model>> {
        let contact = Contact::find()
            .filter(contact::Column::OwnerUid.eq(owner_uid))
            .filter(contact::Column::ContactUid.eq(contact_uid))
            .one(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(contact)
    }

    /// 获取用户的所有联系人
    pub async fn list_by_owner(db: &DbConn, owner_uid: i64) -> AppResult<Vec<contact::Model>> {
        let contacts = Contact::find()
            .filter(contact::Column::OwnerUid.eq(owner_uid))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(contacts)
    }

    /// 更新联系人备注
    pub async fn update_remark(db: &DbConn, owner_uid: i64, contact_uid: i64, remark: Option<String>) -> AppResult<()> {
        let contact = Self::find_by_owner_and_contact(db, owner_uid, contact_uid)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("联系人关系不存在: {} -> {}", owner_uid, contact_uid)))?;

        let mut contact_update: contact::ActiveModel = contact.into();
        contact_update.remark = ActiveValue::Set(remark);
        contact_update.update_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        contact_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 更新联系人标签
    pub async fn update_tag(db: &DbConn, owner_uid: i64, contact_uid: i64, tag: Option<String>) -> AppResult<()> {
        let contact = Self::find_by_owner_and_contact(db, owner_uid, contact_uid)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("联系人关系不存在: {} -> {}", owner_uid, contact_uid)))?;

        let mut contact_update: contact::ActiveModel = contact.into();
        contact_update.tag = ActiveValue::Set(tag);
        contact_update.update_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        contact_update.update(db).await.map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// 删除联系人
    pub async fn delete(db: &DbConn, owner_uid: i64, contact_uid: i64) -> AppResult<()> {
        let contact = Self::find_by_owner_and_contact(db, owner_uid, contact_uid)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("联系人关系不存在: {} -> {}", owner_uid, contact_uid)))?;

        Contact::delete_by_id(contact.id)
            .exec(db)
            .await
            .map_err(|e| AppError::Database(e))?;
        Ok(())
    }
}
