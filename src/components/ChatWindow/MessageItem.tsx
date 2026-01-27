// 组件 - 单条消息
// TODO: Phase 4 时完善消息项组件

import React from 'react';
import type { ChatMessage } from '../../types';

import './MessageItem.less';

interface MessageItemProps {
  message: ChatMessage;
  isSelf?: boolean;
}

export const MessageItem: React.FC<MessageItemProps> = ({ message, isSelf = false }) => {
  return (
    <div className={`message-item ${isSelf ? 'self' : 'other'}`}>
      <div className="message-bubble">
        <div className="message-content">{message.content}</div>
        <div className="message-time">{new Date(message.create_time).toLocaleTimeString()}</div>
      </div>
    </div>
  );
};
