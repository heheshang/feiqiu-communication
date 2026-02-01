use sea_orm::DbConn;
use tauri::{AppHandle, Manager};
use tracing::{error, info};

use crate::core::chat::receipt::ReceiptHandler;
use crate::core::chat::receiver::MessageReceiver;
use crate::core::contact::start_discovery;
use crate::database::init_database;
use crate::database::handler::UserHandler;
use crate::database::model::user;
use crate::event::bus::EVENT_RECEIVER;
use crate::event::handlers::{handle_network_event, handle_ui_event};
use crate::event::model::AppEvent;
use crate::network::udp::{init_udp_socket, start_udp_receiver};

pub async fn init_app(app_handle: &AppHandle) -> Result<DbConn, Box<dyn std::error::Error>> {
    init_logging();

    info!("飞秋通讯启动中...");

    let app_data_dir = app_handle.path().app_data_dir()?;
    info!("应用数据目录: {:?}", app_data_dir);

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("创建应用数据目录失败: {} (路径: {:?})", e, app_data_dir))?;

    if !app_data_dir.exists() {
        return Err(format!("应用数据目录创建后仍不存在: {:?}", app_data_dir).into());
    }

    let db_path = app_data_dir.join("feiqiu.db");
    info!("数据库文件路径: {:?}", db_path);

    if !db_path.exists() {
        info!("创建数据库文件: {}", db_path.display());
        if let Err(e) = std::fs::File::create(&db_path) {
            return Err(format!("无法创建数据库文件: {} (路径: {:?})", e, db_path).into());
        }
        info!("数据库文件创建成功");
    } else {
        info!("数据库文件已存在: {}", db_path.display());
    }

    let db_str = db_path.to_str().ok_or_else(|| "数据库路径包含无效字符")?;
    let db = init_database(Some(db_str)).await?;

    ensure_current_user_exists(&db).await?;

    start_background_services(app_handle.clone(), db.clone()).await;

    info!("飞秋通讯启动完成");

    Ok(db)
}

fn init_logging() {
    use tracing_subscriber::fmt;
    use tracing_subscriber::EnvFilter;

    fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("feiqiu_communication=info".parse().unwrap()),
        )
        .init();
}

async fn ensure_current_user_exists(db: &DbConn) -> Result<(), String> {
    let (local_ip, local_port) = get_local_network_info()
        .await
        .map_err(|e| format!("获取本地网络信息失败: {}", e))?;

    let machine_id = format!("{}:{}", local_ip, local_port);

    match UserHandler::find_by_ip_port(db, &local_ip, local_port).await {
        Ok(Some(_user)) => {
            info!("用户已存在: {}", local_ip);
            Ok(())
        }
        Ok(None) => {
            info!("创建新用户: {}", local_ip);

            let nickname = get_computer_name().await.unwrap_or_else(|_| "用户".to_string());

            let new_user = user::Model {
                uid: 0,
                feiq_ip: local_ip.clone(),
                feiq_port: local_port,
                feiq_machine_id: machine_id.clone(),
                nickname,
                avatar: None,
                status: 1,
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
            Ok(())
        }
    }
}

async fn start_background_services(app_handle: AppHandle, db: DbConn) {
    if let Err(e) = init_udp_socket().await {
        error!("初始化 UDP socket 失败: {}", e);
        return;
    }

    tokio::task::yield_now().await;

    let db_clone = db.clone();
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        event_loop(app_handle_clone, db_clone).await;
    });

    let db_clone = std::sync::Arc::new(db.clone());
    tokio::spawn(async move {
        MessageReceiver::new(db_clone).start();
    });

    let db_clone = std::sync::Arc::new(db.clone());
    tokio::spawn(async move {
        ReceiptHandler::new(db_clone).start();
    });

    tokio::spawn(async move {
        if let Err(e) = start_udp_receiver().await {
            error!("UDP 接收器启动失败: {}", e);
        }
    });

    tokio::spawn(async move {
        if let Err(e) = start_discovery().await {
            error!("用户发现服务启动失败: {}", e);
        }
    });
}

async fn event_loop(_app_handle: AppHandle, db: DbConn) {
    loop {
        match EVENT_RECEIVER.recv() {
            Ok(event) => {
                let db_clone = db.clone();
                tokio::spawn(async move {
                    match event {
                        AppEvent::Network(net_event) => {
                            handle_network_event(net_event, &db_clone).await;
                        }
                        AppEvent::Ui(ui_event) => {
                            handle_ui_event(ui_event).await;
                        }
                        _ => {}
                    }
                });
            }
            Err(e) => {
                error!("事件接收失败: {}", e);
            }
        }
    }
}

async fn get_local_network_info() -> Result<(String, u16), String> {
    use local_ip_address::local_ip;

    let ip = local_ip().map_err(|e| format!("获取本地 IP 失败: {}", e))?;

    Ok((ip.to_string(), 2425))
}

async fn get_computer_name() -> Result<String, String> {
    if let Ok(name) = std::env::var("COMPUTERNAME") {
        return Ok(name);
    }
    if let Ok(name) = std::env::var("HOSTNAME") {
        return Ok(name);
    }

    hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .map_err(|_| "用户".to_string())
}
