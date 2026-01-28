// src-tauri/src/main.rs
//
// 飞秋通讯 - Tauri 应用入口
// 基于 Tauri 2.0 + Rust + React

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// 导入模块
mod core;
mod database;
mod error;
mod event;
mod ipc;
mod network;
mod types;
mod utils;

use database::init_database;
use event::bus::EVENT_RECEIVER;
use event::model::AppEvent;
use network::udp::start_udp_receiver;
use tracing::{error, info};

// ============================================================
// Tauri 命令：基础功能
// ============================================================

/// 获取应用版本
#[tauri::command]
async fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// 初始化应用
#[tauri::command]
async fn init_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 初始化日志
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    info!("飞秋通讯启动中...");

    // 初始化数据库
    let db = init_database().await.map_err(|e| format!("数据库初始化失败: {}", e))?;

    // 存储数据库连接到应用状态
    app_handle.manage(db);

    // 启动 UDP 接收器（后台任务）
    let _app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        if let Err(e) = start_udp_receiver().await {
            error!("UDP 接收器启动失败: {}", e);
        }
    });

    // 启动事件处理器
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        event_loop(app_handle_clone).await;
    });

    info!("飞秋通讯启动完成");
    Ok(())
}

/// 事件循环：处理全局事件
async fn event_loop(app_handle: tauri::AppHandle) {
    loop {
        match EVENT_RECEIVER.recv() {
            Ok(event) => {
                match event {
                    AppEvent::Network(net_event) => {
                        // 处理网络事件
                        handle_network_event(net_event, &app_handle).await;
                    }
                    AppEvent::Ui(ui_event) => {
                        // 处理 UI 事件
                        handle_ui_event(ui_event, &app_handle).await;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                error!("事件接收失败: {}", e);
            }
        }
    }
}

/// 处理网络事件
async fn handle_network_event(event: crate::event::model::NetworkEvent, _app_handle: &tauri::AppHandle) {
    match event {
        crate::event::model::NetworkEvent::PacketReceived { packet, addr } => {
            info!("收到数据包: {} from {}", packet, addr);
            // TODO: 解析并处理具体的数据包
            // let feiq_packet: FeiqPacket = serde_json::from_str(&packet).unwrap();
        }
        _ => {}
    }
}

/// 处理 UI 事件
async fn handle_ui_event(event: crate::event::model::UiEvent, _app_handle: &tauri::AppHandle) {
    match event {
        _ => {}
    }
}

// ============================================================
// Tauri 主函数
// ============================================================

#[tokio::main]
async fn main() {
    // 构建并运行 Tauri 应用
    tauri::Builder::default()
        // 注册命令
        .invoke_handler(tauri::generate_handler![
            get_version,
            init_app,
            // 聊天相关
            ipc::chat::get_chat_history_handler,
            ipc::chat::send_text_message_handler,
            ipc::chat::get_session_list_handler,
            ipc::chat::mark_messages_read_handler,
            ipc::chat::mark_message_read_and_send_receipt,
            ipc::chat::retry_send_message,
            // 通讯录相关
            ipc::contact::get_contact_list_handler,
            ipc::contact::get_online_users_handler,
            // 文件相关
            ipc::file::send_file_request_handler,
            ipc::file::accept_file_request_handler,
            ipc::file::reject_file_request_handler,
            ipc::file::get_file_handler,
            ipc::file::cancel_upload_handler,
            ipc::file::get_pending_transfers_handler,
            ipc::file::resume_transfer_handler,
            // 群组相关
            ipc::group::create_group_handler,
            ipc::group::get_group_info_handler,
            ipc::group::get_group_members_handler,
            ipc::group::add_group_member_handler,
            ipc::group::remove_group_member_handler,
            ipc::group::update_member_role_handler,
            ipc::group::get_user_groups_handler,
        ])
        // 应用启动事件
        .setup(|app| {
            // 确保在开发时可以打开 DevTools
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");
}
