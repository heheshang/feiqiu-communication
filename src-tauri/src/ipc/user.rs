// src-tauri/src/ipc/user.rs
//
/// 用户相关 IPC 接口
use crate::database::handler::UserHandler;
use crate::types::{MapErrToFrontend, UserInfo};
use sea_orm::DbConn;
use tauri::State;
use tracing::{error, info};

/// 获取当前用户信息
///
/// 从数据库中查找当前登录用户，如果不存在则创建默认用户
#[tauri::command]
pub async fn get_current_user_handler(state: State<'_, DbConn>) -> Result<UserInfo, String> {
    let db = state.inner();

    // 获取本地 IP 和端口
    let (local_ip, local_port) = get_local_network_info().unwrap_or_else(|e| {
        error!("获取本地网络信息失败: {}, 使用默认值", e);
        ("127.0.0.1".to_string(), 2425)
    });

    // 生成机器 ID
    let machine_id = format!("{}:{}", local_ip, local_port);

    // 尝试从数据库查找用户
    match UserHandler::find_by_ip_port(db, &local_ip, local_port).await {
        Ok(Some(user)) => {
            info!("找到已存在的用户: {}", user.nickname);
            Ok(UserInfo {
                uid: user.uid,
                nickname: user.nickname,
                feiq_ip: user.feiq_ip,
                feiq_port: user.feiq_port as u16,
                feiq_machine_id: user.feiq_machine_id,
                avatar: user.avatar,
                status: user.status,
            })
        }
        Ok(None) => {
            // 用户不存在，创建新用户
            info!("创建新用户: {}", local_ip);

            // 使用计算机名作为默认昵称
            let nickname = get_computer_name().unwrap_or_else(|_| "用户".to_string());

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

            let created_user = UserHandler::create(db, new_user).await.map_err_to_frontend()?;

            Ok(UserInfo {
                uid: created_user.uid,
                nickname: created_user.nickname,
                feiq_ip: created_user.feiq_ip,
                feiq_port: created_user.feiq_port as u16,
                feiq_machine_id: created_user.feiq_machine_id,
                avatar: created_user.avatar,
                status: created_user.status,
            })
        }
        Err(e) => {
            // 查询失败，返回临时用户
            error!("查询用户失败: {}, 返回临时用户", e);
            Ok(UserInfo {
                uid: 0,
                nickname: "我".to_string(),
                feiq_ip: local_ip,
                feiq_port: local_port,
                feiq_machine_id: machine_id,
                avatar: None,
                status: 1,
            })
        }
    }
}

/// 更新当前用户信息
#[tauri::command]
pub async fn update_current_user_handler(
    uid: i64,
    nickname: Option<String>,
    avatar: Option<String>,
    state: State<'_, DbConn>,
) -> Result<UserInfo, String> {
    let db = state.inner();

    // 获取现有用户
    let mut user = UserHandler::find_by_id(db, uid).await.map_err_to_frontend()?;

    // 更新字段
    if let Some(nick) = nickname {
        user.nickname = nick;
    }
    if let Some(av) = avatar {
        user.avatar = Some(av);
    }

    // 保存更新
    let updated_user = UserHandler::update(db, uid, user).await.map_err_to_frontend()?;

    Ok(UserInfo {
        uid: updated_user.uid,
        nickname: updated_user.nickname,
        feiq_ip: updated_user.feiq_ip,
        feiq_port: updated_user.feiq_port as u16,
        feiq_machine_id: updated_user.feiq_machine_id,
        avatar: updated_user.avatar,
        status: updated_user.status,
    })
}

/// 获取本地网络信息
///
/// 返回本地 IP 地址和端口
fn get_local_network_info() -> Result<(String, u16), String> {
    // 获取本地 IP 地址
    use local_ip_address::local_ip;

    let ip = local_ip().map_err(|e| format!("获取本地 IP 失败: {}", e))?;

    Ok((ip.to_string(), 2425))
}

/// 获取计算机名称
fn get_computer_name() -> Result<String, String> {
    // 尝试从环境变量获取
    if let Ok(name) = std::env::var("COMPUTERNAME") {
        return Ok(name);
    }
    if let Ok(name) = std::env::var("HOSTNAME") {
        return Ok(name);
    }

    // 使用默认值
    Ok("用户".to_string())
}
