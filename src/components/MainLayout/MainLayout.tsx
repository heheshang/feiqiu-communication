// src/components/MainLayout/MainLayout.tsx
//
/// 主布局组件 - 三栏布局（仿微信）
/// 左侧：会话列表 | 中间：通讯录 | 右侧：聊天窗口

import React, { useState, useEffect } from 'react';
import { SessionList } from '../SessionList/SessionList';
import ContactList from '../Contact/ContactList';
import ChatWindow from '../ChatWindow/ChatWindow';
import { GroupList } from '../GroupList';
import { CreateGroupDialog } from '../CreateGroupDialog';
import GroupChatWindow from '../GroupChatWindow';
import { useUser } from '../../hooks/useUser';
import { useContact } from '../../hooks/useContact';
import { useChat } from '../../hooks/useChat';
import { useUserStore } from '../../store';
import type { UserInfo, ChatMessage } from '../../types';
import './MainLayout.less';

interface LayoutState {
  selectedUser: UserInfo | null;
  selectedSessionId: number | null;
  viewMode: 'normal' | 'chat' | 'contact';
  activeTab: 'chats' | 'groups';
  selectedGroupId: number | null;
}

const MainLayout: React.FC = () => {
  const { currentUser, getCurrentUser } = useUser();
  const { onlineUsers } = useContact();
  const { initialize } = useUserStore();

  const [layoutState, setLayoutState] = useState<LayoutState>({
    selectedUser: null,
    selectedSessionId: null,
    viewMode: 'normal',
    activeTab: 'chats',
    selectedGroupId: null,
  });

  const [createGroupDialogOpen, setCreateGroupDialogOpen] = useState(false);

  // 使用 useChat hook 管理聊天状态
  const { messages, loadSessionMessages, selectSession, retryMessage, sendFileMessage, sessions } =
    useChat();

  // 初始化用户状态
  useEffect(() => {
    initialize();
  }, [initialize]);

  // 确保 currentUser 存在
  useEffect(() => {
    if (!currentUser) {
      getCurrentUser();
    }
  }, [currentUser, getCurrentUser]);

  // 会话选择处理
  const handleSessionSelect = (sessionId: number, userId: number) => {
    // 从会话列表中查找选中的会话
    const session = sessions.find((s) => s.sid === sessionId);
    if (!session) {
      console.warn('Session not found:', sessionId);
      return;
    }

    // 从在线用户中查找目标用户
    const targetUser = onlineUsers.find((u) => u.uid === userId);
    if (!targetUser) {
      console.warn('User not found:', userId);
      return;
    }

    // 选择会话并加载数据
    selectSession(session);

    // 更新布局状态
    setLayoutState({
      selectedUser: targetUser,
      selectedSessionId: sessionId,
      viewMode: 'chat',
      activeTab: 'chats',
      selectedGroupId: null,
    });
  };

  const handleUserSelect = (user: UserInfo) => {
    setLayoutState({
      selectedUser: user,
      selectedSessionId: null,
      viewMode: 'chat',
      activeTab: 'chats',
      selectedGroupId: null,
    });
  };

  const handleBackToList = () => {
    setLayoutState({
      selectedUser: null,
      selectedSessionId: null,
      viewMode: 'normal',
      activeTab: layoutState.activeTab,
      selectedGroupId: null,
    });
  };

  const handleLoadMore = () => {
    if (layoutState.selectedUser) {
      loadSessionMessages(0, layoutState.selectedUser.uid);
    }
  };

  const handleRetryMessage = (message: ChatMessage) => {
    retryMessage(message);
  };

  const handleSendFile = async (file: File) => {
    if (!layoutState.selectedUser) return;

    try {
      await sendFileMessage(0, layoutState.selectedUser.uid, file.name, file.name);
    } catch (error) {
      console.error('发送文件失败:', error);
      throw error;
    }
  };

  // 标签页切换处理
  const handleTabChange = (tab: 'chats' | 'groups') => {
    setLayoutState({
      ...layoutState,
      activeTab: tab,
      selectedUser: null,
      selectedSessionId: null,
      selectedGroupId: null,
      viewMode: 'normal',
    });
  };

  // 群组选择处理
  const handleGroupSelect = (groupId: number) => {
    setLayoutState({
      ...layoutState,
      selectedGroupId: groupId,
      selectedUser: null,
      selectedSessionId: null,
      viewMode: 'chat',
    });
  };

  // 创建群组对话处理
  const handleCreateGroupOpen = () => {
    setCreateGroupDialogOpen(true);
  };

  const handleCreateGroupClose = () => {
    setCreateGroupDialogOpen(false);
  };

  // 移动端：返回按钮
  const showBackButton = layoutState.viewMode !== 'normal';

  return (
    <div className={`main-layout ${layoutState.viewMode}`}>
      {/* 左侧：会话列表 */}
      <div className="layout-sidebar">
        {showBackButton && (
          <div className="sidebar-back" onClick={handleBackToList}>
            <svg viewBox="0 0 24 24" fill="none">
              <path
                d="M15 18L9 12L15 6"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          </div>
        )}

        {/* 标签页切换 */}
        <div className="sidebar-tabs">
          <button
            className={`tab-button ${layoutState.activeTab === 'chats' ? 'active' : ''}`}
            onClick={() => handleTabChange('chats')}
          >
            Chats
          </button>
          <button
            className={`tab-button ${layoutState.activeTab === 'groups' ? 'active' : ''}`}
            onClick={() => handleTabChange('groups')}
          >
            Groups
          </button>
        </div>

        {/* 条件渲染：会话列表或群组列表 */}
        {layoutState.activeTab === 'chats' ? (
          <SessionList
            selectedUserId={layoutState.selectedUser?.uid}
            onSessionSelect={handleSessionSelect}
          />
        ) : (
          <div className="groups-container">
            <div className="groups-header">
              <button className="create-group-btn" onClick={handleCreateGroupOpen}>
                + Create Group
              </button>
            </div>
            <GroupList onSelectGroup={handleGroupSelect} />
          </div>
        )}
      </div>

      {/* 中间：通讯录 */}
      <div className="layout-contact">
        {showBackButton && (
          <div className="contact-back" onClick={handleBackToList}>
            <svg viewBox="0 0 24 24" fill="none">
              <path
                d="M15 18L9 12L15 6"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          </div>
        )}
        <ContactList users={onlineUsers} onUserClick={handleUserSelect} />
      </div>

      {/* 右侧：聊天窗口 */}
      <div className="layout-chat">
        {layoutState.activeTab === 'chats' ? (
          <ChatWindow
            targetUser={layoutState.selectedUser || undefined}
            sessionType={0}
            messages={Object.values(messages).flat()}
            currentUserId={currentUser?.uid}
            onLoadMore={handleLoadMore}
            onRetryMessage={handleRetryMessage}
            onSendFile={handleSendFile}
          />
        ) : (
          <GroupChatWindow
            gid={layoutState.selectedGroupId || undefined}
            onGroupDeleted={() => {
              // Clear the selected group when group is deleted or user leaves
              setLayoutState((prev) => ({ ...prev, selectedGroupId: null }));
            }}
          />
        )}
      </div>

      {/* 创建群组对话框 */}
      <CreateGroupDialog
        isOpen={createGroupDialogOpen}
        onClose={handleCreateGroupClose}
        availableMembers={onlineUsers.map((u) => ({
          id: u.uid.toString(),
          name: u.nickname,
        }))}
      />
    </div>
  );
};

export default MainLayout;
