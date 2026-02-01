import React from 'react';
import ChatWindow from '../../ChatWindow/ChatWindow';
import GroupChatWindow from '../../GroupChatWindow';
import type { UserInfo, ChatMessage } from '../../../types';

interface ChatPanelProps {
  activeTab: 'chats' | 'groups';
  selectedUser?: UserInfo;
  selectedGroupId?: number | null;
  currentUserId?: number;
  messages: ChatMessage[];
  onLoadMore: () => void;
  onRetryMessage: (message: ChatMessage) => void;
  onSendFile: (file: File) => Promise<void>;
  onGroupDeleted: () => void;
}

export const ChatPanel: React.FC<ChatPanelProps> = ({
  activeTab,
  selectedUser,
  selectedGroupId,
  currentUserId,
  messages,
  onLoadMore,
  onRetryMessage,
  onSendFile,
  onGroupDeleted,
}) => {
  return (
    <div className="layout-chat">
      {activeTab === 'chats' ? (
        <ChatWindow
          targetUser={selectedUser}
          sessionType={0}
          messages={messages}
          currentUserId={currentUserId}
          onLoadMore={onLoadMore}
          onRetryMessage={onRetryMessage}
          onSendFile={onSendFile}
        />
      ) : (
        <GroupChatWindow gid={selectedGroupId || undefined} onGroupDeleted={onGroupDeleted} />
      )}
    </div>
  );
};
