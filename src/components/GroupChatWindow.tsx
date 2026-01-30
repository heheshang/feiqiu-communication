// src/components/GroupChatWindow.tsx
//
/// 群组聊天窗口组件
/// 显示群组消息、成员列表和消息输入框

import React, { useEffect, useState, useMemo } from 'react';
import MessageList from './ChatWindow/MessageList';
import MessageInput from './ChatWindow/MessageInput';
import { AddMemberDialog } from './AddMemberDialog';
import { GroupSettingsDialog } from './GroupSettingsDialog';
import { useGroupStore } from '../store/groupStore';
import { useChatStore } from '../store/chatStore';
import { useUserStore } from '../store/userStore';
import { chatService } from '../services/chatService';
import { groupService } from '../services/groupService';
import type { GroupMember } from '../types';
import './GroupChatWindow.less';

interface GroupChatWindowProps {
  gid?: number;
  onGroupDeleted?: () => void;
}

const GroupChatWindow: React.FC<GroupChatWindowProps> = ({ gid, onGroupDeleted }) => {
  const currentGroup = useGroupStore((state) => state.currentGroup);
  const members = useGroupStore((state) => state.members);
  const fetchGroupMembers = useGroupStore((state) => state.fetchGroupMembers);
  const isLoadingMembers = useGroupStore((state) => state.isLoadingMembers);

  // Memoize messages selector to avoid infinite loop from getSnapshot
  const messages = useMemo(
    () => (gid ? useChatStore.getState().getMessagesBySession(gid) : []),
    [gid]
  );
  const fetchMessages = useChatStore((state) => state.fetchMessages);
  const isLoadingMessages = useChatStore((state) =>
    gid ? state.isLoadingMessages.has(gid) : false
  );

  const currentUser = useUserStore((state) => state.currentUser);

  const [showMembers, setShowMembers] = useState(false);
  const [showAddMemberDialog, setShowAddMemberDialog] = useState(false);
  const [showSettingsDialog, setShowSettingsDialog] = useState(false);

  const groupMembers = (gid && members[gid]) || [];
  const isLoadingGroupMembers = gid ? isLoadingMembers.has(gid) : false;

  // Get current user's role in this group
  const currentUserMember = useMemo(
    () => groupMembers.find((m: GroupMember) => m.member_uid === currentUser?.uid),
    [groupMembers, currentUser]
  );

  // 加载群组成员
  useEffect(() => {
    if (gid && !members[gid]) {
      fetchGroupMembers(gid).catch((error) => {
        console.error('Failed to fetch group members:', error);
      });
    }
  }, [gid, fetchGroupMembers]); // Don't include 'members' to avoid infinite loop

  // 加载消息历史
  useEffect(() => {
    if (gid && currentUser) {
      fetchMessages(1, gid, () => chatService.getHistory(1, gid, 0)).catch((error) => {
        console.error('Failed to fetch messages:', error);
      });
    }
  }, [gid, currentUser, fetchMessages]);

  // 处理发送消息
  const handleSendMessage = async (content: string) => {
    if (!gid || !currentUser) return;

    try {
      await chatService.sendMessage(1, gid, content, currentUser.uid);
      await fetchMessages(1, gid, () => chatService.getHistory(1, gid, 0));
    } catch (error) {
      console.error('Failed to send message:', error);
    }
  };

  // 如果没有选中群组，显示空状态
  if (!currentGroup || !gid) {
    return (
      <div className="group-chat-window group-chat-window-empty">
        <div className="empty-state">
          <div className="empty-icon">👥</div>
          <div className="empty-text">
            <p>选择一个群组开始聊天</p>
          </div>
        </div>
      </div>
    );
  }

  const canManageMembers =
    currentUserMember && (currentUserMember.role === 2 || currentUserMember.role === 1); // Owner or Admin

  // Handlers
  const handleAddMember = () => {
    setShowAddMemberDialog(true);
  };

  const handleRemoveMember = async (member: GroupMember) => {
    if (!gid || !currentUser) return;

    // Don't allow removing owner or yourself
    if (member.role === 2) {
      alert('Cannot remove the group owner');
      return;
    }
    if (member.member_uid === currentUser.uid) {
      alert('Cannot remove yourself from the group');
      return;
    }

    if (!confirm(`Remove ${member.nickname} from the group?`)) {
      return;
    }

    try {
      await groupService.removeGroupMember(gid, member.member_uid);
      await fetchGroupMembers(gid); // Refresh member list
    } catch (error) {
      console.error('Failed to remove member:', error);
      alert('Failed to remove member. Please try again.');
    }
  };

  const handleUpdateRole = async (member: GroupMember, newRole: number) => {
    if (!gid) return;

    // Don't allow changing owner role
    if (member.role === 2) {
      alert('Cannot change the owner role');
      return;
    }

    try {
      await groupService.updateMemberRole(gid, member.member_uid, newRole);
      await fetchGroupMembers(gid); // Refresh member list
    } catch (error) {
      console.error('Failed to update role:', error);
      alert('Failed to update role. Please try again.');
    }
  };

  return (
    <div className="group-chat-window">
      {/* 头部 */}
      <div className="chat-header">
        <div className="chat-header-info">
          <div className="chat-header-name">{currentGroup.group_name}</div>
          <div className="chat-header-status">
            <span className="member-count">{groupMembers.length} 人</span>
          </div>
        </div>
        <div className="chat-header-actions">
          <button
            className="header-action-btn"
            title="群组成员"
            onClick={() => setShowMembers(!showMembers)}
          >
            <svg viewBox="0 0 24 24" fill="none">
              <circle cx="8" cy="8" r="3" stroke="currentColor" strokeWidth="2" />
              <circle cx="16" cy="8" r="3" stroke="currentColor" strokeWidth="2" />
              <path
                d="M8 11C5.79086 11 4 12.7909 4 15V20H12V15C12 12.7909 10.2091 11 8 11Z"
                stroke="currentColor"
                strokeWidth="2"
              />
              <path
                d="M16 11C13.7909 11 12 12.7909 12 15V20H20V15C20 12.7909 18.2091 11 16 11Z"
                stroke="currentColor"
                strokeWidth="2"
              />
            </svg>
          </button>
          <button
            className="header-action-btn"
            title="群组设置"
            onClick={() => setShowSettingsDialog(true)}
          >
            <svg viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="1" fill="currentColor" />
              <circle cx="12" cy="5" r="1" fill="currentColor" />
              <circle cx="12" cy="19" r="1" fill="currentColor" />
            </svg>
          </button>
        </div>
      </div>

      {/* 主容器 */}
      <div className="group-chat-container">
        {/* 消息列表 */}
        <div className="group-chat-main">
          <MessageList
            messages={messages}
            currentUserId={currentUser?.uid}
            hasMore={true}
            isLoading={isLoadingMessages}
          />
        </div>

        {/* 成员列表侧边栏 */}
        {showMembers && (
          <div className="group-members-sidebar">
            <div className="members-header">
              <h3>群组成员</h3>
              <div className="members-header-actions">
                {canManageMembers && (
                  <button className="add-member-btn" onClick={handleAddMember} title="Add members">
                    <svg viewBox="0 0 24 24" fill="none" width="16" height="16">
                      <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" />
                      <path d="M12 8V16M8 12H16" stroke="currentColor" strokeWidth="2" />
                    </svg>
                  </button>
                )}
                <button className="close-btn" onClick={() => setShowMembers(false)} title="关闭">
                  ✕
                </button>
              </div>
            </div>

            <div className="members-list">
              {isLoadingGroupMembers ? (
                <div className="members-loading">加载中...</div>
              ) : groupMembers.length === 0 ? (
                <div className="members-empty">暂无成员</div>
              ) : (
                groupMembers.map((member: GroupMember) => (
                  <div key={member.id} className="member-item">
                    <div className="member-avatar">{member.nickname.charAt(0)}</div>
                    <div className="member-info">
                      <div className="member-name">{member.nickname}</div>
                      <div className="member-role">
                        {member.role === 2 ? '群主' : member.role === 1 ? '管理员' : '成员'}
                      </div>
                    </div>
                    {canManageMembers &&
                      member.member_uid !== currentUser?.uid &&
                      member.role !== 2 && (
                        <div className="member-actions">
                          <button
                            className="member-action-btn"
                            onClick={() => handleUpdateRole(member, member.role === 1 ? 0 : 1)}
                            title={member.role === 1 ? 'Demote to member' : 'Promote to admin'}
                          >
                            {member.role === 1 ? '⬇️' : '⬆️'}
                          </button>
                          <button
                            className="member-action-btn remove-btn"
                            onClick={() => handleRemoveMember(member)}
                            title="Remove from group"
                          >
                            ✕
                          </button>
                        </div>
                      )}
                  </div>
                ))
              )}
            </div>
          </div>
        )}
      </div>

      {/* 输入框 */}
      <MessageInput
        targetId={gid}
        sessionType={1}
        onSendMessage={handleSendMessage}
        placeholder="输入群组消息..."
      />

      {/* Add Member Dialog */}
      {gid && currentUser && (
        <AddMemberDialog
          isOpen={showAddMemberDialog}
          onClose={() => setShowAddMemberDialog(false)}
          gid={gid}
          currentMemberUids={groupMembers.map((m: GroupMember) => m.member_uid)}
          onMembersAdded={() => fetchGroupMembers(gid)}
          currentUserId={currentUser.uid}
        />
      )}

      {/* Group Settings Dialog */}
      {gid && currentGroup && (
        <GroupSettingsDialog
          isOpen={showSettingsDialog}
          onClose={() => setShowSettingsDialog(false)}
          group={currentGroup}
          onGroupUpdated={async () => {
            // Refresh groups list
            await useGroupStore.getState().fetchGroups(currentUser?.uid || 1);
            // Notify parent that group was deleted/left
            onGroupDeleted?.();
          }}
        />
      )}
    </div>
  );
};

export default GroupChatWindow;
