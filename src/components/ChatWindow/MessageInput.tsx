// src/components/ChatWindow/MessageInput.tsx
//
/// 消息输入框组件
/// 支持文本输入、表情、文件上传等功能

import React, { useState, useRef, useEffect } from 'react';
import { EmojiPicker } from '../EmojiPicker/EmojiPicker';
import { FileUpload } from '../FileUpload/FileUpload';
import './MessageInput.less';
import type { SessionType } from '../../types';

interface MessageInputProps {
  targetId?: number;
  sessionType?: SessionType;
  onSendMessage?: (content: string) => Promise<void>;
  onSendFile?: (file: File) => Promise<void>;
  placeholder?: string;
}

const MessageInput: React.FC<MessageInputProps> = ({
  targetId,
  sessionType = 0,
  onSendMessage,
  onSendFile,
  placeholder = '输入消息...',
}) => {
  const [content, setContent] = useState('');
  const [isFocused, setIsFocused] = useState(false);
  const [showEmoji, setShowEmoji] = useState(false);
  const [showFileUpload, setShowFileUpload] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // 自动调整文本框高度
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
      const scrollHeight = textareaRef.current.scrollHeight;
      const maxHeight = 120; // 最大高度
      textareaRef.current.style.height = Math.min(scrollHeight, maxHeight) + 'px';
    }
  }, [content]);

  const handleSend = async () => {
    if (!content.trim() || !targetId) return;

    try {
      await onSendMessage?.(content.trim());
      setContent('');
    } catch (error) {
      console.error('发送消息失败:', error);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleEmojiSelect = (emoji: string) => {
    setContent((prev) => prev + emoji);
    setShowEmoji(false);
    // 聚焦回输入框
    setTimeout(() => textareaRef.current?.focus(), 100);
  };

  const handleFileUpload = async (file: File) => {
    if (!targetId) return;

    try {
      await onSendFile?.(file);
      setShowFileUpload(false);
    } catch (error) {
      console.error('文件上传失败:', error);
    }
  };

  const canSend = content.trim().length > 0 && targetId !== undefined;

  return (
    <div className={`message-input ${isFocused ? 'focused' : ''}`}>
      {/* Emoji 选择器 */}
      {showEmoji && (
        <EmojiPicker onEmojiSelect={handleEmojiSelect} onClose={() => setShowEmoji(false)} />
      )}

      {/* 文件上传 */}
      {showFileUpload && targetId !== undefined && (
        <FileUpload
          targetId={targetId}
          sessionType={sessionType}
          onUpload={handleFileUpload}
          onClose={() => setShowFileUpload(false)}
        />
      )}

      {/* 工具栏 */}
      <div className="input-toolbar">
        <button className="toolbar-btn" title="表情" onClick={() => setShowEmoji(!showEmoji)}>
          <svg viewBox="0 0 24 24" fill="none">
            <circle cx="8" cy="8" r="2" fill="currentColor" />
            <circle cx="16" cy="8" r="2" fill="currentColor" />
            <circle cx="8" cy="16" r="2" fill="currentColor" />
            <circle cx="16" cy="16" r="2" fill="currentColor" />
            <circle cx="12" cy="12" r="2" fill="currentColor" />
            <path
              d="M7 7C7 5.34315 7 5 7 5"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
            />
            <path
              d="M17 7C17 5.34315 17 5 17 5"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
            />
            <path
              d="M7 17C7 18.6568 7 19 7 19"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
            />
            <path
              d="M17 17C17 18.6568 17 19 17 19"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
            />
          </svg>
        </button>

        <button className="toolbar-btn" title="发送文件" onClick={() => setShowFileUpload(true)}>
          <svg viewBox="0 0 24 24" fill="none">
            <path
              d="M12 15V3M12 15L8 11M12 15L16 11"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
            <path
              d="M21 15V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V15"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
            />
            <path
              d="M17 8L12 13L7 8"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </button>

        <button className="toolbar-btn" title="截图（Ctrl+Alt+A）">
          <svg viewBox="0 0 24 24" fill="none">
            <path
              d="M21 19V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2z"
              stroke="currentColor"
              strokeWidth="2"
            />
            <path d="M8 9h8M12 13v-2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
          </svg>
        </button>
      </div>

      {/* 文本输入区 */}
      <div className="input-textarea-wrapper">
        <textarea
          ref={textareaRef}
          className="input-textarea"
          placeholder={placeholder}
          value={content}
          onChange={(e) => setContent(e.target.value)}
          onKeyDown={handleKeyDown}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          rows={1}
        />
      </div>

      {/* 发送按钮 */}
      <div className="input-actions">
        <span className="char-count">{content.length}/2000</span>
        <button
          className={`send-btn ${canSend ? 'active' : ''}`}
          onClick={handleSend}
          disabled={!canSend}
          title={canSend ? '发送 (Enter)' : '请输入消息'}
        >
          发送
        </button>
      </div>
    </div>
  );
};

export default MessageInput;
