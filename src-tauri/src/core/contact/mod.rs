// src-tauri/src/core/contact/mod.rs
//
/// 联系人管理模块
pub mod discovery;

pub use discovery::{
    add_online_user, find_user_by_ip, get_online_users, get_online_users_list, remove_online_user, start_discovery,
};
