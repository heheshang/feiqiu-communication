// src/components/ChatWindow/MessageItem.tsx
//
/// 单条消息组件
/// WeChat 风格的消息气泡（绿色为己方，白色为对方）

import React from 'react';
import './MessageItem.less';
import type { ChatMessage } from '../../types';

interface MessageItemProps {
  message: ChatMessage;
  isSelf?: boolean;
  showAvatar?: boolean;
  showTime?: boolean;
}

const MessageItem: React.FC<MessageItemProps> = ({
  message,
  isSelf = false,
  showAvatar = true,
  showTime = false
}) => {
  // 格式化时间
  const formatTime = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const isToday = date.toDateString() === now.toDateString();

    if (isToday) {
      return date.toLocaleTimeString('zh-CN', {
        hour: '2-digit',
        minute: '2-digit'
      });
    } else {
      return date.toLocaleDateString('zh-CN', {
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
      });
    }
  };

  // 获取消息状态图标
  const getStatusIcon = () => {
    if (!isSelf) return null;

    switch (message.status) {
      case 0: // 发送中
        return (
          <span className="message-status sending">
            <svg viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" strokeDasharray="4 2"/>
            </svg>
          </span>
        );
      case 1: // 已发送
        return (
          <span className="message-status sent">
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M9 12L11 14L15 10M21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21"
                stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          </span>
        );
      case 2: // 已读
        return (
          <span className="message-status read">
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M9 12L11 14L15 10M21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21"
                    stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              <path d="M17 7L7 17M17 7H13" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          </span>
        );
      case -1: // 发送失败
        return (
          <span className="message-status failed">
            <svg viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2"/>
              <path d="M15 9L9 15M9 9L15 15" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
            </svg>
          </span>
        );
      default:
        return null;
    }
  };

  return (
    <div className={`message-item ${isSelf ? 'self' : 'other'}`}>
      {/* 时间分隔线 */}
      {showTime && (
        <div className="message-time-divider">
          <span className="time-divider-content">
            {formatTime(message.send_time)}
          </span>
        </div>
      )}

      <div className="message-content-wrapper">
        {/* 头像（仅对方消息显示） */}
        {showAvatar && !isSelf && (
          <div className="message-avatar">
            <div className="avatar-placeholder">
              {message.content.charAt(0)}
            </div>
          </div>
        )}

        {/* 消息气泡 */}
        <div className="message-bubble-wrapper">
          <div className="message-bubble">
            {/* 消息内容 */}
            <div className="message-text">
              {message.content}
            </div>

            {/* 发送时间和状态 */}
            <div className="message-meta">
              <span className="message-time">
                {formatTime(message.send_time)}
              </span>
              {getStatusIcon()}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MessageItem;
