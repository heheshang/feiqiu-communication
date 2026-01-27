// 组件 - 会话列表
// TODO: Phase 4 时完善会话列表组件

import React from 'react';
import { useChat } from '../../hooks/useChat';
import { formatTime } from '../../utils/time';
import type { ChatSession } from '../../types';

import './SessionList.less';

export const SessionList: React.FC = () => {
  const { sessions, getSessionList } = useChat();

  React.useEffect(() => {
    getSessionList();
  }, [getSessionList]);

  return (
    <div className="session-list">
      {sessions.map((session) => (
        <SessionItem key={session.session_id} session={session} />
      ))}
    </div>
  );
};

interface SessionItemProps {
  session: ChatSession;
}

const SessionItem: React.FC<SessionItemProps> = ({ session }) => {
  return (
    <div className="session-item">
      <div className="session-avatar">
        <div className="avatar-placeholder">{session.target_name[0]}</div>
      </div>
      <div className="session-info">
        <div className="session-header-row">
          <div className="session-name">{session.target_name}</div>
          <div className="session-time">{formatTime(session.last_time)}</div>
        </div>
        <div className="session-footer-row">
          <div className="session-message">{session.last_message}</div>
          {session.unread_count > 0 && (
            <div className="session-badge">{session.unread_count}</div>
          )}
        </div>
      </div>
    </div>
  );
};
