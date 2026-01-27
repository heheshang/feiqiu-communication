// src/components/ChatWindow/MessageList.tsx
//
/// 消息列表组件
/// 显示聊天消息历史，支持滚动加载

import React, { useEffect, useRef } from 'react';
import MessageItem from './MessageItem';
import './MessageList.less';
import type { ChatMessage, UserInfo } from '../../types';

interface MessageListProps {
  targetId?: number;
  targetUser?: UserInfo;
  messages?: ChatMessage[];
  currentUserId?: number;
}

const MessageList: React.FC<MessageListProps> = ({
  targetId,
  targetUser,
  messages = [],
  currentUserId = 0 // TODO: 从用户状态获取
}) => {
  const listRef = useRef<HTMLDivElement>(null);
  const endRef = useRef<HTMLDivElement>(null);

  // 自动滚动到底部
  useEffect(() => {
    if (endRef.current) {
      endRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [messages]);

  // 处理滚动到顶部加载更多
  const handleScroll = (e: React.UIEvent<HTMLDivElement>) => {
    const { scrollTop } = e.currentTarget;
    if (scrollTop === 0) {
      // TODO: 触发加载更多历史消息
      console.log('Load more messages');
    }
  };

  if (!targetUser) {
    return (
      <div className="message-list message-list-empty">
        <div className="empty-message">请选择联系人开始聊天</div>
      </div>
    );
  }

  if (messages.length === 0) {
    return (
      <div className="message-list message-list-empty">
        <div className="empty-message">
          <p>还没有消息</p>
          <p className="empty-hint">打个招呼吧 ~</p>
        </div>
      </div>
    );
  }

  return (
    <div
      ref={listRef}
      className="message-list"
      onScroll={handleScroll}
    >
      {messages.map((message, index) => {
        const isSelf = message.sender_uid === currentUserId;
        return (
          <MessageItem
            key={message.mid || index}
            message={message}
            isSelf={isSelf}
            showAvatar={!isSelf}
            showTime={
              index === 0 ||
              new Date(messages[index - 1].send_time).getDate() !== new Date(message.send_time).getDate()
            }
          />
        );
      })}
      {/* 底部锚点，用于自动滚动 */}
      <div ref={endRef} className="message-list-end" />
    </div>
  );
};

export default MessageList;
