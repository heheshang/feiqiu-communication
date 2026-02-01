import React from 'react';
import { useChat } from '../../hooks/useChat';
import { useUserStore } from '../../store';
import { formatTime } from '../../utils/time';
import type { ChatSession, UserInfo, ChatMessage } from '../../types';
import './SessionList.less';

interface SessionListProps {
  selectedUserId?: number;
  onSessionSelect?: (sessionId: number, userId: number) => void;
}

export const SessionList: React.FC<SessionListProps> = React.memo(
  ({ selectedUserId, onSessionSelect }) => {
    const { sessions, getSessionList, getMessagesBySession } = useChat();
    const { findOnlineUserById, findContactById } = useUserStore();

    React.useEffect(() => {
      getSessionList();
    }, [getSessionList]);

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
  }
);

SessionList.displayName = 'SessionList';

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
  const displayName =
    session.session_name || getUserInfo(session.target_id)?.nickname || `User ${session.target_id}`;
  const lastMessage = session.last_message || getLastMessage(session.sid)?.content || '...';
  const avatar = getUserInfo(session.target_id)?.avatar;
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
