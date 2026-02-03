// src-tauri/src/ipc/contact.rs
//
use crate::core::ContactService;
use crate::types::{Contact, MapErrToFrontend, UserInfo};
use sea_orm::DbConn;
/// 通讯录相关 IPC 接口（薄层 - 只做参数转换和错误映射）
use tauri::State;

/// 获取通讯录列表
#[tauri::command]
pub async fn get_contact_list_handler(owner_uid: i64, db: State<'_, DbConn>) -> Result<Vec<Contact>, String> {
    let db = db.inner();
    ContactService::get_contacts(db, owner_uid).await.map_err_to_frontend()
}

/// 获取在线用户列表
#[tauri::command]
pub async fn get_online_users_handler(db: State<'_, DbConn>) -> Result<Vec<UserInfo>, String> {
    let db = db.inner();
    ContactService::get_online_users(db).await.map_err_to_frontend()
}

/// 添加联系人
#[tauri::command]
pub async fn add_contact_handler(
    owner_uid: i64,
    contact_uid: i64,
    remark: Option<String>,
    tag: Option<String>,
    db: State<'_, DbConn>,
) -> Result<i64, String> {
    let db = db.inner();
    ContactService::add_contact(db, owner_uid, contact_uid, remark, tag).await.map_err_to_frontend()
}

/// 更新联系人信息
#[tauri::command]
pub async fn update_contact_handler(
    id: i64,
    remark: Option<String>,
    tag: Option<String>,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    let db = db.inner();
    ContactService::update_contact(db, id, remark, tag).await.map_err_to_frontend()
}

/// 删除联系人
#[tauri::command]
pub async fn delete_contact_handler(
    id: i64,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    let db = db.inner();
    ContactService::delete_contact(db, id).await.map_err_to_frontend()
}

/// 检查是否已添加联系人
#[tauri::command]
pub async fn is_contact_handler(
    owner_uid: i64,
    contact_uid: i64,
    db: State<'_, DbConn>,
) -> Result<bool, String> {
    let db = db.inner();
    ContactService::is_contact(db, owner_uid, contact_uid).await.map_err_to_frontend()
}
