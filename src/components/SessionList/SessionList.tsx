// 组件 - 会话列表
// TODO: Phase 4 时完善会话列表组件

import React from 'react';
import { useChat } from '../../hooks/useChat';
import { formatTime } from '../../utils/time';
import type { ChatSession } from '../../types';

import './SessionList.less';

interface SessionListProps {
  selectedUserId?: number;
  onSessionSelect?: (sessionId: number, userId: number) => void;
}

export const SessionList: React.FC<SessionListProps> = ({ selectedUserId, onSessionSelect }) => {
  const { sessions, getSessionList } = useChat();

  React.useEffect(() => {
    getSessionList();
  }, [getSessionList]);

  return (
    <div className="session-list">
      {sessions.map((session) => (
        <SessionItem
          key={session.sid}
          session={session}
          isSelected={selectedUserId === session.target_id}
          onSelect={() => onSessionSelect?.(session.sid, session.target_id)}
        />
      ))}
    </div>
  );
};

interface SessionItemProps {
  session: ChatSession;
  isSelected?: boolean;
  onSelect?: () => void;
}

const SessionItem: React.FC<SessionItemProps> = ({ session, isSelected, onSelect }) => {
  // TODO: Get target name from user data
  const targetName = `User ${session.target_id}`;
  const lastMessage = '...'; // TODO: Get from message data

  return (
    <div className={`session-item ${isSelected ? 'selected' : ''}`} onClick={onSelect}>
      <div className="session-avatar">
        <div className="avatar-placeholder">{targetName[0]}</div>
      </div>
      <div className="session-info">
        <div className="session-header-row">
          <div className="session-name">{targetName}</div>
          <div className="session-time">{formatTime(session.update_time)}</div>
        </div>
        <div className="session-footer-row">
          <div className="session-message">{lastMessage}</div>
          {session.unread_count > 0 && <div className="session-badge">{session.unread_count}</div>}
        </div>
      </div>
    </div>
  );
};
