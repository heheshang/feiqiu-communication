// src/components/MainLayout/MainLayout.tsx
//
/// 主布局组件 - 三栏布局（仿微信）
/// 左侧：会话列表 | 中间：通讯录 | 右侧：聊天窗口

import React from 'react';
import './MainLayout.less';
import SessionList from '../SessionList/SessionList';
import ContactList from '../Contact/ContactList';
import ChatWindow from '../ChatWindow/ChatWindow';

interface MainLayoutProps {
  // 预留扩展属性
}

const MainLayout: React.FC<MainLayoutProps> = () => {
  return (
    <div className="main-layout">
      {/* 左侧：会话列表 */}
      <div className="layout-sidebar">
        <SessionList />
      </div>

      {/* 中间：通讯录 */}
      <div className="layout-contact">
        <ContactList />
      </div>

      {/* 右侧：聊天窗口 */}
      <div className="layout-chat">
        <ChatWindow />
      </div>
    </div>
  );
};

export default MainLayout;
