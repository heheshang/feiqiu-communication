// src/components/MainLayout/MainLayout.tsx
//
/// 主布局组件 - 三栏布局（仿微信）
/// 左侧：会话列表 | 中间：通讯录 | 右侧：聊天窗口

import React, { useState, useEffect } from 'react';
import { SessionList } from '../SessionList/SessionList';
import ContactList from '../Contact/ContactList';
import ChatWindow from '../ChatWindow/ChatWindow';
import { useUser } from '../../hooks/useUser';
import { useContact } from '../../hooks/useContact';
import { useChat } from '../../hooks/useChat';
import type { UserInfo, ChatMessage } from '../../types';
import './MainLayout.less';

interface LayoutState {
  selectedUser: UserInfo | null;
  viewMode: 'normal' | 'chat' | 'contact';
}

const MainLayout: React.FC = () => {
  const { currentUser } = useUser();
  const { onlineUsers } = useContact();
  const [layoutState, setLayoutState] = useState<LayoutState>({
    selectedUser: null,
    viewMode: 'normal',
  });

  // 使用 useChat hook 管理聊天状态
  const {
    messages,
    loadInitialMessages,
    loadMoreMessages,
    resetPagination,
    pagination,
    retrySendMessage,
  } = useChat();

  // 当选中用户变化时，加载初始消息
  useEffect(() => {
    if (layoutState.selectedUser) {
      loadInitialMessages(0, layoutState.selectedUser.uid);
    } else {
      resetPagination();
    }
  }, [layoutState.selectedUser?.uid, loadInitialMessages, resetPagination]);

  const handleUserSelect = (user: UserInfo) => {
    setLayoutState({
      selectedUser: user,
      viewMode: 'chat',
    });
  };

  const handleBackToList = () => {
    setLayoutState({
      selectedUser: null,
      viewMode: 'normal',
    });
  };

  const handleLoadMore = () => {
    if (layoutState.selectedUser) {
      loadMoreMessages(0, layoutState.selectedUser.uid);
    }
  };

  const handleRetryMessage = (message: ChatMessage) => {
    retrySendMessage(message);
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
        <SessionList
          selectedUserId={layoutState.selectedUser?.uid}
          onSessionSelect={(sessionId, userId) => {
            // TODO: 实现会话选择逻辑
            console.log('Selected session:', sessionId, 'User:', userId);
          }}
        />
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
        <ChatWindow
          targetUser={layoutState.selectedUser || undefined}
          sessionType={0}
          messages={messages}
          currentUserId={currentUser?.uid}
          hasMore={pagination.hasMore}
          isLoading={pagination.isLoading}
          onLoadMore={handleLoadMore}
          onRetryMessage={handleRetryMessage}
        />
      </div>
    </div>
  );
};

export default MainLayout;
