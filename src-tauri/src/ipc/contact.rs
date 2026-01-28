// src-tauri/src/ipc/contact.rs
//
use crate::database::handler::{ContactHandler, UserHandler};
use crate::types::{Contact, UserInfo};
use sea_orm::DbConn;
/// 通讯录相关 IPC 接口
use tauri::State;

/// 获取通讯录列表
#[tauri::command]
pub async fn get_contact_list_handler(owner_uid: i64, state: State<'_, DbConn>) -> Result<Vec<Contact>, String> {
    let db = state.inner();

    let contacts = ContactHandler::list_by_owner(db, owner_uid).await.map_err(|e| e.to_string())?;

    // Convert to frontend type
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

/// 获取在线用户列表
#[tauri::command]
pub async fn get_online_users_handler(state: State<'_, DbConn>) -> Result<Vec<UserInfo>, String> {
    let db = state.inner();

    // Get online users (status = 1 or 2)
    let online_users = UserHandler::find_by_status(db, 1).await.map_err(|e| e.to_string())?;

    let busy_users = UserHandler::find_by_status(db, 2).await.map_err(|e| e.to_string())?;

    let all_users: Vec<_> = online_users.into_iter().chain(busy_users).collect();

    // Convert to frontend type
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
