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

use core::chat::receipt::ReceiptHandler;
use core::chat::receiver::MessageReceiver;
use core::contact::start_discovery;
use database::init_database;
use event::bus::EVENT_RECEIVER;
use event::model::AppEvent;
use network::feiq::model::ProtocolPacket;
use network::udp::{init_udp_socket, start_udp_receiver};
use sea_orm::DbConn;
use std::sync::Arc;
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
async fn init_app(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    init_logging();

    info!("飞秋通讯启动中...");

    // 获取应用数据目录
    let app_data_dir = app_handle.path().app_data_dir()?;
    info!("应用数据目录: {:?}", app_data_dir);

    // 确保目录存在
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("创建应用数据目录失败: {} (路径: {:?})", e, app_data_dir))?;

    // 验证目录是否真的存在
    if !app_data_dir.exists() {
        return Err(format!("应用数据目录创建后仍不存在: {:?}", app_data_dir).into());
    }

    // 构建数据库路径
    let db_path = app_data_dir.join("feiqiu.db");
    info!("数据库文件路径: {:?}", db_path);

    // 预先创建数据库文件（如果不存在）
    // 这有助于诊断权限问题
    if !db_path.exists() {
        info!("创建数据库文件: {}", db_path.display());
        if let Err(e) = std::fs::File::create(&db_path) {
            return Err(format!("无法创建数据库文件: {} (路径: {:?})", e, db_path).into());
        }
        info!("数据库文件创建成功");
    } else {
        info!("数据库文件已存在: {}", db_path.display());
    }

    // 初始化数据库（使用完整路径）
    // SQLite 会自动创建数据库文件，但我们需要确保父目录存在且可写
    let db_str = db_path.to_str().ok_or_else(|| "数据库路径包含无效字符")?;
    let db = init_database(Some(db_str)).await?;

    // 确保当前用户存在（如果不存在则创建）
    ensure_current_user_exists(&db).await?;

    // 将数据库连接包装在 Arc 中以便共享
    let db = Arc::new(db);

    // 存储数据库连接到应用状态
    app_handle.manage(db.clone());

    // 启动后台服务
    start_background_services(app_handle.clone(), db).await;

    // 广播上线通知
    broadcast_online_presence().await;

    info!("飞秋通讯启动完成");
    Ok(())
}

/// 初始化日志系统
fn init_logging() {
    use tracing_subscriber::fmt;
    use tracing_subscriber::EnvFilter;

    // 配置日志级别和格式
    fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .with_env_filter(EnvFilter::from_default_env().add_directive("feiqiu_communication=info".parse().unwrap()))
        .init();
}

/// 确保当前用户存在
async fn ensure_current_user_exists(db: &DbConn) -> Result<(), String> {
    use crate::database::handler::UserHandler;

    // 获取本地网络信息
    let (local_ip, local_port) = get_local_network_info()
        .await
        .map_err(|e| format!("获取本地网络信息失败: {}", e))?;

    let machine_id = format!("{}:{}", local_ip, local_port);

    // 检查用户是否已存在
    match UserHandler::find_by_ip_port(db, &local_ip, local_port).await {
        Ok(Some(_user)) => {
            info!("用户已存在: {}", local_ip);
            Ok(())
        }
        Ok(None) => {
            // 用户不存在，创建新用户
            info!("创建新用户: {}", local_ip);

            let nickname = get_computer_name().await.unwrap_or_else(|_| "用户".to_string());

            let new_user = crate::database::model::user::Model {
                uid: 0, // 数据库会自动生成
                feiq_ip: local_ip.clone(),
                feiq_port: local_port,
                feiq_machine_id: machine_id.clone(),
                nickname,
                avatar: None,
                status: 1, // 在线
                create_time: chrono::Utc::now().naive_utc(),
                update_time: chrono::Utc::now().naive_utc(),
            };

            UserHandler::create(db, new_user)
                .await
                .map_err(|e| format!("创建用户失败: {}", e))?;

            info!("用户创建成功: {} ({})", local_ip, machine_id);
            Ok(())
        }
        Err(e) => {
            error!("查询用户失败: {}, 继续使用临时用户", e);
            Ok(()) // 不阻塞启动，使用临时用户
        }
    }
}

/// 启动所有后台服务
async fn start_background_services(app_handle: tauri::AppHandle, db: Arc<DbConn>) {
    // 初始化全局 UDP 套接字（必须在其他 UDP 操作之前）
    if let Err(e) = init_udp_socket().await {
        error!("初始化 UDP socket 失败: {}", e);
    }

    // 启动用户发现服务
    let _app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        if let Err(e) = start_discovery().await {
            error!("用户发现服务启动失败: {}", e);
        }
    });

    // 启动 UDP 接收器（后台任务）
    // 注意：UDP 接收器现在使用全局共享的 UDP 套接字
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

    // 启动消息接收处理器
    let db_clone = db.clone();
    tokio::spawn(async move {
        MessageReceiver::new(db_clone).start();
    });

    // 启动已读回执处理器
    let db_clone = db.clone();
    tokio::spawn(async move {
        ReceiptHandler::new(db_clone).start();
    });
}

/// 广播上线通知
async fn broadcast_online_presence() {
    info!("广播上线通知...");

    let packet = ProtocolPacket::make_entry_packet();

    // 使用新的全局 UDP socket 广播
    if let Err(e) = crate::network::udp::socket::broadcast_packet(&packet).await {
        error!("广播上线失败: {}", e);
    } else {
        info!("上线通知已发送");
    }
}

/// 获取本地网络信息
async fn get_local_network_info() -> Result<(String, u16), String> {
    use local_ip_address::local_ip;

    let ip = local_ip().map_err(|e| format!("获取本地 IP 失败: {}", e))?;

    Ok((ip.to_string(), 2425))
}

/// 获取计算机名称
async fn get_computer_name() -> Result<String, String> {
    // 尝试从环境变量获取
    if let Ok(name) = std::env::var("COMPUTERNAME") {
        return Ok(name);
    }
    if let Ok(name) = std::env::var("HOSTNAME") {
        return Ok(name);
    }

    // 使用 hostname crate 作为后备
    hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .map_err(|_| "用户".to_string())
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
            // 数据包解析和处理由 discovery 模块的事件循环处理
            // 这里只记录日志用于调试
        }
        crate::event::model::NetworkEvent::UserOnline { user } => {
            info!("用户上线事件: {}", user);
        }
        crate::event::model::NetworkEvent::UserOffline { ip } => {
            info!("用户离线事件: {}", ip);
        }
        crate::event::model::NetworkEvent::UserUpdated { user } => {
            info!("用户更新事件: {}", user);
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
            // 用户相关
            ipc::user::get_current_user_handler,
            ipc::user::update_current_user_handler,
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
            // 初始化应用
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = init_app(&handle).await {
                    eprintln!("初始化应用失败: {}", e);
                }
            });

            // 确保在开发时可以打开 DevTools
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");
}
