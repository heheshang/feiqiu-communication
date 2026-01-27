// src-tauri/src/core/contact/mod.rs
//
/// 联系人管理模块

pub mod discovery;

pub use discovery::{
    start_discovery,
    get_online_users,
    get_online_users_list,
    add_online_user,
    remove_online_user,
    find_user_by_ip,
};
