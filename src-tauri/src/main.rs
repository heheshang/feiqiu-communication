#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod database;
mod error;
mod event;
mod ipc;
mod network;
mod types;
mod utils;

use app::commands::get_version;
use app::setup::setup_app;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_version,
            ipc::user::get_current_user_handler,
            ipc::user::update_current_user_handler,
            ipc::chat::get_chat_history_handler,
            ipc::chat::send_text_message_handler,
            ipc::chat::get_session_list_handler,
            ipc::chat::mark_messages_read_handler,
            ipc::chat::mark_message_read_and_send_receipt,
            ipc::chat::retry_send_message,
            ipc::contact::get_contact_list_handler,
            ipc::contact::get_online_users_handler,
            ipc::file::send_file_request_handler,
            ipc::file::accept_file_request_handler,
            ipc::file::reject_file_request_handler,
            ipc::file::get_file_handler,
            ipc::file::cancel_upload_handler,
            ipc::file::get_pending_transfers_handler,
            ipc::file::resume_transfer_handler,
            ipc::group::create_group_handler,
            ipc::group::get_group_info_handler,
            ipc::group::get_group_members_handler,
            ipc::group::add_group_member_handler,
            ipc::group::remove_group_member_handler,
            ipc::group::update_member_role_handler,
            ipc::group::get_user_groups_handler,
            ipc::group::update_group_info_handler,
            ipc::group::delete_group_handler,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            setup_app(handle)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");
}
