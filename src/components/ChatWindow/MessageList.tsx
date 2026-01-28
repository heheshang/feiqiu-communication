// src/components/ChatWindow/MessageList.tsx
//
/// 消息列表组件
/// 显示聊天消息历史，支持滚动加载

import React, { useEffect, useRef, useState } from 'react';
import MessageItem from './MessageItem';
import './MessageList.less';
import type { ChatMessage, UserInfo, SessionType } from '../../types';

interface MessageListProps {
  targetId?: number;
  targetUser?: UserInfo;
  messages?: ChatMessage[];
  currentUserId?: number;
  sessionType?: SessionType;
  onLoadMore?: () => void;
  hasMore?: boolean;
  isLoading?: boolean;
}

const MessageList: React.FC<MessageListProps> = ({
  targetId,
  targetUser,
  messages = [],
  currentUserId = 0, // TODO: 从用户状态获取
  sessionType = 0,
  onLoadMore,
  hasMore = true,
  isLoading = false,
}) => {
  const listRef = useRef<HTMLDivElement>(null);
  const endRef = useRef<HTMLDivElement>(null);
  const scrollHeightRef = useRef(0);
  const [initialLoadDone, setInitialLoadDone] = useState(false);

  // 自动滚动到底部（仅在初始加载或新消息时）
  useEffect(() => {
    if (endRef.current && initialLoadDone) {
      endRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [messages.length, initialLoadDone]);

  // 标记初始加载完成
  useEffect(() => {
    if (messages.length > 0 && !initialLoadDone) {
      setInitialLoadDone(true);
    }
  }, [messages.length, initialLoadDone]);

  // 处理滚动到顶部加载更多
  const handleScroll = (e: React.UIEvent<HTMLDivElement>) => {
    const currentTarget = e.currentTarget;
    const { scrollTop } = currentTarget;

    // 当滚动到顶部附近时触发加载
    if (scrollTop < 50 && hasMore && !isLoading && onLoadMore) {
      // 保存当前滚动高度
      scrollHeightRef.current = currentTarget.scrollHeight;

      // 触发加载更多
      onLoadMore();

      // 加载完成后恢复滚动位置
      requestAnimationFrame(() => {
        if (listRef.current) {
          const newScrollHeight = listRef.current.scrollHeight;
          const heightDiff = newScrollHeight - scrollHeightRef.current;
          listRef.current.scrollTop = heightDiff;
        }
      });
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
      {/* 加载更多指示器 */}
      {isLoading && (
        <div className="message-loading-more">
          <span className="loading-spinner"></span>
          <span className="loading-text">加载中...</span>
        </div>
      )}

      {/* 没有更多消息提示 */}
      {!hasMore && messages.length > 0 && (
        <div className="message-no-more">
          <span>没有更多消息了</span>
        </div>
      )}

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
