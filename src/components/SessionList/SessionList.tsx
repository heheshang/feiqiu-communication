// 组件 - 会话列表

import React from 'react';
import { useChat } from '../../hooks/useChat';
import { useUserStore } from '../../store';
import { formatTime } from '../../utils/time';
import type { ChatSession } from '../../types';
import type { UserInfo } from '../../types';
import type { ChatMessage } from '../../types';

import './SessionList.less';

interface SessionListProps {
  selectedUserId?: number;
  onSessionSelect?: (sessionId: number, userId: number) => void;
}

export const SessionList: React.FC<SessionListProps> = ({ selectedUserId, onSessionSelect }) => {
  const { sessions, getSessionList, getMessagesBySession } = useChat();
  const { findOnlineUserById, findContactById } = useUserStore();

  React.useEffect(() => {
    getSessionList();
  }, [getSessionList]);

  // 获取用户信息的辅助函数
  const getUserInfo = (uid: number): UserInfo | undefined => {
    return findOnlineUserById(uid) || findContactById(uid);
  };

  return (
    <div className="session-list">
      {sessions.map((session) => (
        <SessionItem
          key={session.sid}
          session={session}
          isSelected={selectedUserId === session.target_id}
          onSelect={() => onSessionSelect?.(session.sid, session.target_id)}
          getUserInfo={getUserInfo}
          getLastMessage={(sessionId) => {
            const msgs = getMessagesBySession(sessionId);
            return msgs.length > 0 ? msgs[msgs.length - 1] : undefined;
          }}
        />
      ))}
    </div>
  );
};

interface SessionItemProps {
  session: ChatSession;
  isSelected?: boolean;
  onSelect?: () => void;
  getUserInfo: (uid: number) => UserInfo | undefined;
  getLastMessage: (sessionId: number) => ChatMessage | undefined;
}

const SessionItem: React.FC<SessionItemProps> = ({
  session,
  isSelected,
  onSelect,
  getUserInfo,
  getLastMessage,
}) => {
  // 使用缓存的会话名称，如果没有则查找用户信息
  const displayName =
    session.session_name || getUserInfo(session.target_id)?.nickname || `User ${session.target_id}`;

  // 使用缓存的最后消息，如果没有则从消息列表中获取
  const lastMessage = session.last_message || getLastMessage(session.sid)?.content || '...';

  // 获取用户头像
  const avatar = getUserInfo(session.target_id)?.avatar;

  // 获取最后消息时间
  const lastMessageTime = session.last_message_time || session.update_time;

  return (
    <div className={`session-item ${isSelected ? 'selected' : ''}`} onClick={onSelect}>
      <div className="session-avatar">
        {avatar ? (
          <img src={avatar} alt={displayName} className="avatar-image" />
        ) : (
          <div className="avatar-placeholder">{displayName[0]}</div>
        )}
      </div>
      <div className="session-info">
        <div className="session-header-row">
          <div className="session-name">{displayName}</div>
          <div className="session-time">{formatTime(lastMessageTime)}</div>
        </div>
        <div className="session-footer-row">
          <div className="session-message">{lastMessage}</div>
          {session.unread_count > 0 && (
            <div className="session-badge">
              {session.unread_count > 99 ? '99+' : session.unread_count}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
