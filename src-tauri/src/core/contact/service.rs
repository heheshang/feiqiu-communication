//! 联系人业务逻辑服务层
//!
//! ContactService 提供联系人相关的业务逻辑操作，包括：
//! - 获取联系人列表
//! - 添加联系人
//! - 更新联系人信息
//! - 删除联系人

use crate::database::handler::{ContactHandler, UserHandler};
use crate::error::AppResult;
use crate::types::{Contact, UserInfo};
use sea_orm::DbConn;

/// 联系人服务
pub struct ContactService;

impl ContactService {
    /// 获取用户的联系人列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 用户ID
    ///
    /// # 返回
    /// 返回联系人列表（前端类型）
    pub async fn get_contacts(db: &DbConn, owner_uid: i64) -> AppResult<Vec<Contact>> {
        let contacts = ContactHandler::list_by_owner(db, owner_uid).await?;

        // 转换为前端类型
        let result: Vec<Contact> = contacts
            .into_iter()
            .map(|c| Contact {
                id: c.id,
                owner_uid: c.owner_uid,
                contact_uid: c.contact_uid,
                remark: c.remark,
                tag: c.tag,
            })
            .collect();

        Ok(result)
    }

    /// 添加联系人
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 用户ID
    /// - `contact_uid`: 联系人用户ID
    /// - `remark`: 备注（可选）
    /// - `tag`: 标签（可选）
    ///
    /// # 返回
    /// 返回新创建的联系人ID
    pub async fn add_contact(
        db: &DbConn,
        owner_uid: i64,
        contact_uid: i64,
        remark: Option<String>,
        tag: Option<String>,
    ) -> AppResult<i64> {
        // 检查联系人是否存在
        let _contact_exists = UserHandler::find_by_id(db, contact_uid).await?;

        // 使用handler创建（handler会检查重复）
        let contact = ContactHandler::create(db, owner_uid, contact_uid, remark, tag).await?;

        Ok(contact.id)
    }

    /// 更新联系人信息
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `id`: 联系人ID
    /// - `remark`: 备注（可选）
    /// - `tag`: 标签（可选）
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn update_contact(db: &DbConn, id: i64, remark: Option<String>, tag: Option<String>) -> AppResult<()> {
        // 获取现有联系人
        let contact = ContactHandler::find_by_id(db, id).await?;

        // 使用handler更新（分别调用update_remark和update_tag）
        ContactHandler::update_remark(db, contact.owner_uid, contact.contact_uid, remark).await?;
        ContactHandler::update_tag(db, contact.owner_uid, contact.contact_uid, tag).await?;

        Ok(())
    }

    /// 删除联系人
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `id`: 联系人ID
    ///
    /// # 返回
    /// 返回操作结果
    pub async fn delete_contact(db: &DbConn, id: i64) -> AppResult<()> {
        // 获取联系人以获取owner_uid和contact_uid
        let contact = ContactHandler::find_by_id(db, id).await?;

        // 使用handler删除（需要owner_uid和contact_uid）
        ContactHandler::delete(db, contact.owner_uid, contact.contact_uid).await
    }

    /// 检查是否已添加联系人
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `owner_uid`: 用户ID
    /// - `contact_uid`: 联系人用户ID
    ///
    /// # 返回
    /// 返回是否已添加
    pub async fn is_contact(db: &DbConn, owner_uid: i64, contact_uid: i64) -> AppResult<bool> {
        let contact = ContactHandler::find_by_owner_and_contact(db, owner_uid, contact_uid).await?;
        Ok(contact.is_some())
    }

    /// 获取在线用户列表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    ///
    /// # 返回
    /// 返回在线用户列表（前端类型）
    pub async fn get_online_users(db: &DbConn) -> AppResult<Vec<UserInfo>> {
        // 获取在线用户 (status = 1 或 2)
        let online_users = UserHandler::find_by_status(db, 1).await?;
        let busy_users = UserHandler::find_by_status(db, 2).await?;

        let all_users: Vec<_> = online_users.into_iter().chain(busy_users).collect();

        // 转换为前端类型
        let result: Vec<UserInfo> = all_users
            .into_iter()
            .map(|u| UserInfo {
                uid: u.uid,
                nickname: u.nickname,
                feiq_ip: u.feiq_ip,
                feiq_port: u.feiq_port as u16,
                feiq_machine_id: u.feiq_machine_id,
                avatar: u.avatar,
                status: u.status,
            })
            .collect();

        Ok(result)
    }
}
