// 组件 - 消息列表
// TODO: Phase 4 时完善消息列表组件

import React from 'react';
import { useChat } from '../../hooks/useChat';
import type { ChatMessage } from '../../types';

import './MessageList.less';

interface MessageListProps {
  targetId: number;
}

export const MessageList: React.FC<MessageListProps> = ({ targetId: _targetId }) => {
  const { messages } = useChat();

  return (
    <div className="message-list">
      {messages.map((message) => (
        <MessageItem key={message.msg_id} message={message} />
      ))}
    </div>
  );
};

interface MessageItemProps {
  message: ChatMessage;
}

const MessageItem: React.FC<MessageItemProps> = ({ message }) => {
  const isSelf = message.sender_uid === 0; // TODO: 从用户状态判断

  return (
    <div className={`message-item ${isSelf ? 'self' : 'other'}`}>
      <div className="message-bubble">
        <div className="message-content">{message.content}</div>
        <div className="message-time">{new Date(message.create_time).toLocaleTimeString()}</div>
      </div>
    </div>
  );
};
