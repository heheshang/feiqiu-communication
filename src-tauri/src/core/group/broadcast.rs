// src-tauri/src/core/group/broadcast.rs
//
/// 群组消息广播功能
use crate::database::handler::group::GroupMemberHandler;
use crate::database::handler::user::UserHandler;
use crate::error::AppResult;
use crate::network::feiq::model::FeiQPacket;
use crate::network::udp::sender;
use sea_orm::DbConn;

/// 群组消息广播器
pub struct GroupBroadcaster;

impl GroupBroadcaster {
    /// 向群组所有成员广播消息
    pub async fn broadcast_message(
        db: &DbConn,
        gid: i64,
        packet: &FeiQPacket,
        sender_uid: i64,
    ) -> AppResult<usize> {
        // 获取群组所有成员
        let members = GroupMemberHandler::list_by_group(db, gid).await?;

        let mut sent_count = 0;

        for member in members {
            // 跳过发送者自己
            if member.member_uid == sender_uid {
                continue;
            }

            // 获取成员的网络信息
            if let Ok(user) = UserHandler::find_by_id(db, member.member_uid).await {
                // 检查用户是否在线
                if user.status == 1 {
                    // 发送消息到该成员的 IP:Port
                    let addr = format!("{}:{}", user.feiq_ip, user.feiq_port);
                    if sender::send_packet(&addr, packet).await.is_err() {
                        // 记录发送失败但继续发送给其他成员
                        tracing::warn!("Failed to send group message to {}", addr);
                    } else {
                        sent_count += 1;
                    }
                }
            }
        }

        Ok(sent_count)
    }

    /// 获取群组成员数量
    pub async fn get_member_count(db: &DbConn, gid: i64) -> AppResult<usize> {
        let members = GroupMemberHandler::list_by_group(db, gid).await?;
        Ok(members.len())
    }

    /// 获取群组在线成员数量
    pub async fn get_online_member_count(db: &DbConn, gid: i64) -> AppResult<usize> {
        let members = GroupMemberHandler::list_by_group(db, gid).await?;
        let mut online_count = 0;

        for member in members {
            if let Ok(user) = UserHandler::find_by_id(db, member.member_uid).await {
                if user.status == 1 {
                    online_count += 1;
                }
            }
        }

        Ok(online_count)
    }
}
