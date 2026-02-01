// src/components/ChatWindow/ChatWindow.tsx
//
/// 聊天窗口容器组件
/// 包含头部信息、消息列表和输入框

import React from 'react';
import MessageList from './MessageList';
import MessageInput from './MessageInput';
import './ChatWindow.less';
import type { UserInfo, SessionType, ChatMessage } from '../../types';
import { OnlineStatus } from '../../types/user';

interface ChatWindowProps {
  targetUser?: UserInfo;
  sessionType?: SessionType;
  messages?: ChatMessage[];
  currentUserId?: number;
  hasMore?: boolean;
  isLoading?: boolean;
  onLoadMore?: () => void;
  onRetryMessage?: (message: ChatMessage) => void;
  onSendFile?: (file: File) => Promise<void>;
}

const ChatWindow: React.FC<ChatWindowProps> = ({
  targetUser,
  sessionType = 0,
  messages = [],
  currentUserId = 0,
  hasMore = true,
  isLoading = false,
  onLoadMore,
  onRetryMessage,
  onSendFile,
}) => {
  // 如果没有选中用户，显示空状态
  if (!targetUser) {
    return (
      <div className="chat-window chat-window-empty">
        <div className="empty-state">
          <div className="empty-icon">💬</div>
          <div className="empty-text">
            <p>选择一个联系人开始聊天</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="chat-window">
      {/* 头部 */}
      <div className="chat-header">
        <div className="chat-header-info">
          <div className="chat-header-name">{targetUser.nickname}</div>
          <div className="chat-header-status">
            {targetUser.status === OnlineStatus.Online && (
              <span className="status-text online">在线</span>
            )}
            {targetUser.status === OnlineStatus.Busy && (
              <span className="status-text busy">忙碌</span>
            )}
            {targetUser.status === OnlineStatus.Offline && (
              <span className="status-text offline">离线</span>
            )}
          </div>
        </div>
        <div className="chat-header-actions">
          <button className="header-action-btn" title="查看资料">
            <svg viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" />
              <path d="M12 6v6l4 2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
            </svg>
          </button>
          <button className="header-action-btn" title="更多">
            <svg viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="1" fill="currentColor" />
              <circle cx="12" cy="5" r="1" fill="currentColor" />
              <circle cx="12" cy="19" r="1" fill="currentColor" />
            </svg>
          </button>
        </div>
      </div>

      {/* 消息列表 */}
      <MessageList
        targetUser={targetUser}
        messages={messages}
        currentUserId={currentUserId}
        hasMore={hasMore}
        isLoading={isLoading}
        onLoadMore={onLoadMore}
        onRetryMessage={onRetryMessage}
      />

      {/* 输入框 */}
      <MessageInput sessionType={sessionType} targetId={targetUser?.uid} onSendFile={onSendFile} />
    </div>
  );
};

export default ChatWindow;
