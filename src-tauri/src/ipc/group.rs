// src-tauri/src/ipc/group.rs
//
/// 群组相关 IPC 接口
use crate::database::handler::group::{GroupHandler, GroupMemberHandler};
use crate::database::handler::user::UserHandler;
use crate::types::{GroupInfo, GroupMember, GroupRole};
use sea_orm::DbConn;
use tauri::State;

/// 创建群组
#[tauri::command]
pub async fn create_group_handler(
    group_name: String,
    creator_uid: i64,
    member_uids: Vec<i64>,
    db: State<'_, DbConn>,
) -> Result<i64, String> {
    // 创建群组
    let group = GroupHandler::create(db.inner(), group_name.clone(), creator_uid, None)
        .await
        .map_err(|e| e.to_string())?;

    let gid = group.gid;

    // 添加成员（创建者已自动添加）
    for member_uid in member_uids {
        if member_uid != creator_uid {
            // 添加为普通成员 (role = 0)
            GroupMemberHandler::add_member(db.inner(), gid, member_uid, 0)
                .await
                // 忽略已存在的错误
                .ok();
        }
    }

    Ok(gid)
}

/// 获取群组信息
#[tauri::command]
pub async fn get_group_info_handler(gid: i64, db: State<'_, DbConn>) -> Result<GroupInfo, String> {
    let group = GroupHandler::find_by_id(db.inner(), gid).await.map_err(|e| e.to_string())?;

    // 获取成员数量
    let members = GroupMemberHandler::list_by_group(db.inner(), gid)
        .await
        .map_err(|e| e.to_string())?;
    let member_count = members.len() as i64;

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
    let members = GroupMemberHandler::list_by_group(db.inner(), gid)
        .await
        .map_err(|e| e.to_string())?;

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
    role: i8,
    db: State<'_, DbConn>,
) -> Result<(), String> {
    GroupMemberHandler::add_member(db.inner(), gid, member_uid, role)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 移除群成员
#[tauri::command]
pub async fn remove_group_member_handler(gid: i64, member_uid: i64, db: State<'_, DbConn>) -> Result<(), String> {
    GroupMemberHandler::remove_member(db.inner(), gid, member_uid)
        .await
        .map_err(|e| e.to_string())?;
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
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取用户加入的群组列表
#[tauri::command]
pub async fn get_user_groups_handler(user_uid: i64, db: State<'_, DbConn>) -> Result<Vec<GroupInfo>, String> {
    let memberships = GroupMemberHandler::list_by_member(db.inner(), user_uid)
        .await
        .map_err(|e| e.to_string())?;

    let mut groups = Vec::new();
    for membership in memberships {
        let group = GroupHandler::find_by_id(db.inner(), membership.gid)
            .await
            .map_err(|e| e.to_string())?;

        groups.push(GroupInfo {
            gid: group.gid,
            group_name: group.group_name,
            avatar: group.avatar,
            creator_uid: group.creator_uid,
            desc: group.description,
            create_time: group.create_time.to_string(),
        });
    }

    Ok(groups)
}
