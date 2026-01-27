// 组件 - 输入框
// TODO: Phase 4 时完善输入框组件

import React, { useState } from 'react';
import { useChat } from '../../hooks/useChat';

import './MessageInput.less';

interface MessageInputProps {
  targetId: number;
}

export const MessageInput: React.FC<MessageInputProps> = ({ targetId }) => {
  const [content, setContent] = useState('');
  const { sendMessage } = useChat();

  const handleSend = async () => {
    if (!content.trim()) return;

    try {
      await sendMessage(0, targetId, content);
      setContent('');
    } catch (error) {
      console.error('发送消息失败:', error);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="message-input">
      <textarea
        className="input-textarea"
        placeholder="输入消息..."
        value={content}
        onChange={(e) => setContent(e.target.value)}
        onKeyDown={handleKeyDown}
      />
      <div className="input-toolbar">
        <button className="toolbar-btn">😀</button>
        <button className="toolbar-btn">📎</button>
        <button className="send-btn" onClick={handleSend}>
          发送
        </button>
      </div>
    </div>
  );
};
