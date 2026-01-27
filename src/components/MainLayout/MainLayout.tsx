// src/components/MainLayout/MainLayout.tsx
//
/// 主布局组件 - 三栏布局（仿微信）
/// 左侧：会话列表 | 中间：通讯录 | 右侧：聊天窗口

import React, { useState } from 'react';
import SessionList from '../SessionList/SessionList';
import ContactList from '../Contact/ContactList';
import ChatWindow from '../ChatWindow/ChatWindow';
import { useUser } from '../../hooks/useUser';
import { useContact } from '../../hooks/useContact';
import type { UserInfo } from '../../types';
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
    viewMode: 'normal'
  });

  const handleUserSelect = (user: UserInfo) => {
    setLayoutState({
      selectedUser: user,
      viewMode: 'chat'
    });
  };

  const handleBackToList = () => {
    setLayoutState({
      selectedUser: null,
      viewMode: 'normal'
    });
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
              <path d="M15 18L9 12L15 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
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
              <path d="M15 18L9 12L15 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          </div>
        )}
        <ContactList
          users={onlineUsers}
          onUserClick={handleUserSelect}
        />
      </div>

      {/* 右侧：聊天窗口 */}
      <div className="layout-chat">
        <ChatWindow
          targetId={layoutState.selectedUser?.uid}
          targetUser={layoutState.selectedUser || undefined}
          sessionType="single"
        />
      </div>
    </div>
  );
};

export default MainLayout;
