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
