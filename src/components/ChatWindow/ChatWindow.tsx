// 组件 - 聊天窗口容器
// TODO: Phase 4 时完善聊天窗口组件

import React from 'react';
import { MessageList } from './MessageList';
import { MessageInput } from './MessageInput';

import './ChatWindow.less';

interface ChatWindowProps {
  targetId: number;
  targetName: string;
}

export const ChatWindow: React.FC<ChatWindowProps> = ({ targetId, targetName }) => {
  return (
    <div className="chat-window">
      <div className="chat-header">
        <h3>{targetName}</h3>
      </div>
      <MessageList targetId={targetId} />
      <MessageInput targetId={targetId} />
    </div>
  );
};
