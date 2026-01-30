// src-tauri/src/ipc/group.rs
//
/// 群组相关 IPC 接口（薄层 - 只做参数转换和错误映射）
use crate::core::group::GroupService;
use crate::database::handler::group::{GroupHandler, GroupMemberHandler};
use crate::database::handler::user::UserHandler;
use crate::types::{GroupInfo, GroupMember, GroupRole, MapErrToFrontend};
use sea_orm::DbConn;
use tauri::State;

#[tauri::command]
pub async fn create_group_handler(
    group_name: String,
    creator_uid: i64,
    member_uids: Vec<i64>,
    db: State<'_, DbConn>,
) -> Result<i64, String> {
    let gid = GroupService::create_group(db.inner(), group_name, creator_uid, None, None)
        .await
        .map_err_to_frontend()?;

    // 添加成员（创建者已自动添加）
    for member_uid in member_uids {
        if member_uid != creator_uid {
            GroupService::add_member(db.inner(), gid, member_uid, String::new())
                .await
                .ok();
        }
    }

    Ok(gid)
}

/// 获取群组信息
#[tauri::command]
pub async fn get_group_info_handler(gid: i64, db: State<'_, DbConn>) -> Result<GroupInfo, String> {
    let group = GroupHandler::find_by_id(db.inner(), gid).await.map_err_to_frontend()?;

    // 获取成员数量
    let _members = GroupMemberHandler::list_by_group(db.inner(), gid).await.map_err_to_frontend()?;

    Ok(GroupInfo {
        gid: group.gid,
        group_name: group.group_name,
        avatar: group.avatar,
        creator_uid: group.creator_uid,
        desc: group.description,
        create_time: group.create_time.to_string(),
    })
}

/// 获取群成员列表
#[tauri::command]
pub async fn get_group_members_handler(gid: i64, db: State<'_, DbConn>) -> Result<Vec<GroupMember>, String> {
    let members = GroupService::get_members(db.inner(), gid)
        .await
        .map_err_to_frontend()?;

    let mut result = Vec::new();
    for m in members {
        // 获取用户昵称
        let nickname = match UserHandler::find_by_id(db.inner(), m.member_uid).await {
            Ok(user) => user.nickname,
            _ => format!("User{}", m.member_uid),
        };

        result.push(GroupMember {
            id: m.id,
            gid: m.gid,
            member_uid: m.member_uid,
            nickname,
            role: match m.role {
                0 => GroupRole::Member,
                1 => GroupRole::Admin,
                2 => GroupRole::Owner,
                _ => GroupRole::Member,
            },
            join_time: m.join_time.to_string(),
        });
    }

    Ok(result)
}

/// 添加群成员
#[tauri::command]
pub async fn add_group_member_handler(
    gid: i64,
    member_uid: i64,
    _role: i8,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    GroupService::add_member(db.inner(), gid, member_uid, String::new())
        .await
        .map_err_to_frontend()?;
    Ok(())
}

/// 移除群成员
#[tauri::command]
pub async fn remove_group_member_handler(gid: i64, member_uid: i64, db: State<'_, DbConn>) -> Result<(), String> {
    GroupService::remove_member(db.inner(), gid, member_uid)
        .await
        .map_err_to_frontend()?;
    Ok(())
}

/// 更新成员角色
#[tauri::command]
pub async fn update_member_role_handler(
    gid: i64,
    member_uid: i64,
    role: i8,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    GroupMemberHandler::update_role(db.inner(), gid, member_uid, role)
        .await
        .map_err_to_frontend()?;
    Ok(())
}

/// 获取用户加入的群组列表
#[tauri::command]
pub async fn get_user_groups_handler(user_uid: i64, db: State<'_, DbConn>) -> Result<Vec<GroupInfo>, String> {
    let groups = GroupService::get_groups(db.inner(), user_uid)
        .await
        .map_err_to_frontend()?;

    let result = groups
        .into_iter()
        .map(|group| GroupInfo {
            gid: group.gid,
            group_name: group.group_name,
            avatar: group.avatar,
            creator_uid: group.creator_uid,
            desc: group.description,
            create_time: group.create_time.to_string(),
        })
        .collect();

    Ok(result)
}

/// 更新群组信息
#[tauri::command]
pub async fn update_group_info_handler(
    gid: i64,
    group_name: String,
    desc: String,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    GroupService::update_group(
        db.inner(),
        gid,
        Some(group_name),
        if desc.is_empty() { None } else { Some(desc) },
        None,
    )
    .await
    .map_err_to_frontend()?;
    Ok(())
}

/// 删除群组
#[tauri::command]
pub async fn delete_group_handler(gid: i64, db: State<'_, DbConn>) -> Result<(), String> {
    GroupService::delete_group(db.inner(), gid)
        .await
        .map_err_to_frontend()?;
    Ok(())
}
