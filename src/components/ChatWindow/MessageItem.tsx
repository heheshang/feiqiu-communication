// src/components/ChatWindow/MessageItem.tsx
//
/// 单条消息组件
/// WeChat 风格的消息气泡（绿色为己方，白色为对方）

import React from 'react';
import { FileProgress } from '../FileProgress';
import type { ChatMessage, TransferProgress } from '../../types';
import { getFileIconType, formatFileSize } from '../../utils/path';

interface MessageItemProps {
  message: ChatMessage;
  isSelf?: boolean;
  showAvatar?: boolean;
  showTime?: boolean;
  onRetry?: (message: ChatMessage) => void;
  transferProgress?: TransferProgress;
  onTransferCancel?: (fileId: number) => void;
}

const MessageItem: React.FC<MessageItemProps> = React.memo(
  ({
    message,
    isSelf = false,
    showAvatar = true,
    showTime = false,
    onRetry,
    transferProgress,
    onTransferCancel,
  }) => {
    // 格式化时间
    const formatTime = (timestamp: string) => {
      const date = new Date(timestamp);
      const now = new Date();
      const isToday = date.toDateString() === now.toDateString();

      if (isToday) {
        return date.toLocaleTimeString('zh-CN', {
          hour: '2-digit',
          minute: '2-digit',
        });
      } else {
        return date.toLocaleDateString('zh-CN', {
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
        });
      }
    };

    // 获取消息状态图标
    const getStatusIcon = () => {
      if (!isSelf) return null;

      switch (message.status) {
        case 0: // 发送中
          return (
            <span className="message-status sending">
              <svg viewBox="0 0 24 24" fill="none">
                <circle
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeDasharray="4 2"
                />
              </svg>
            </span>
          );
        case 1: // 已发送
          return (
            <span className="message-status sent">
              <svg viewBox="0 0 24 24" fill="none">
                <path
                  d="M9 12L11 14L15 10M21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            </span>
          );
        case 2: // 已读
          return (
            <span className="message-status read">
              <svg viewBox="0 0 24 24" fill="none">
                <path
                  d="M9 12L11 14L15 10M21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
                <path
                  d="M17 7L7 17M17 7H13"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            </span>
          );
        case -1: // 发送失败
          return (
            <span
              className="message-status failed"
              onClick={() => onRetry?.(message)}
              title="点击重试"
            >
              <svg viewBox="0 0 24 24" fill="none">
                <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" />
                <path
                  d="M15 9L9 15M9 9L15 15"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                />
              </svg>
            </span>
          );
        default:
          return null;
      }
    };

    // 渲染文件消息内容
    const renderFileContent = () => {
      // Try to parse file_info from message if available
      if (message.file_info) {
        const info = message.file_info;
        const iconType = getFileIconType(info.file_name);
        const iconClass = `file-icon file-icon-${iconType}`;

        return (
          <div className="message-file">
            <div className={iconClass}>
              <svg viewBox="0 0 24 24" fill="none" width="24" height="24">
                <path
                  d="M14 2H6C5.46957 2 4.96086 2.21071 4.58579 2.58579C4.21071 2.96086 4 3.46957 4 4V20C4 20.5304 4.21071 21.0391 4.58579 21.4142C4.96086 21.7893 5.46957 22 6 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V8L14 2Z"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
                <path
                  d="M14 2V8H20"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            </div>
            <div className="file-info">
              <div className="file-name" title={info.file_name}>
                {info.file_name}
              </div>
              <div className="file-size">{formatFileSize(info.file_size)}</div>
            </div>

            {/* 如果有传输进度，显示进度条 */}
            {transferProgress && (
              <FileProgress
                fileId={transferProgress.file_id}
                fileName={info.file_name}
                progress={transferProgress.transferred}
                total={transferProgress.total}
                speed={transferProgress.speed}
                status={transferProgress.status as any}
                onCancel={onTransferCancel}
              />
            )}
          </div>
        );
      }

      // Fallback: Parse from content string
      // Format: "[文件] filename" or "filename:size" or JSON
      let fileName = '未知文件';
      let fileSize = 0;

      // Remove "[文件]" prefix if present
      let content = message.content.replace(/^\[文件\]\s*/, '');

      // Try JSON parse first
      try {
        const fileInfo = JSON.parse(content);
        fileName = fileInfo.file_name || fileInfo.name || fileName;
        fileSize = fileInfo.file_size || fileInfo.size || 0;
      } catch {
        // Parse "filename:size" format
        const parts = content.split(':');
        fileName = parts[0] || fileName;
        if (parts[1]) {
          fileSize = parseInt(parts[1], 10);
        }
      }

      const iconType = getFileIconType(fileName);
      const iconClass = `file-icon file-icon-${iconType}`;

      return (
        <div className="message-file">
          <div className={iconClass}>
            <svg viewBox="0 0 24 24" fill="none" width="24" height="24">
              <path
                d="M14 2H6C5.46957 2 4.96086 2.21071 4.58579 2.58579C4.21071 2.96086 4 3.46957 4 4V20C4 20.5304 4.21071 21.0391 4.58579 21.4142C4.96086 21.7893 5.46957 22 6 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V8L14 2Z"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
              <path
                d="M14 2V8H20"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          </div>
          <div className="file-info">
            <div className="file-name" title={fileName}>
              {fileName}
            </div>
            <div className="file-size">{fileSize > 0 ? formatFileSize(fileSize) : '未知大小'}</div>
          </div>

          {/* 如果有传输进度，显示进度条 */}
          {transferProgress && (
            <FileProgress
              fileId={transferProgress.file_id}
              fileName={fileName}
              progress={transferProgress.transferred}
              total={transferProgress.total}
              speed={transferProgress.speed}
              status={transferProgress.status as any}
              onCancel={onTransferCancel}
            />
          )}
        </div>
      );
    };

    return (
      <div className={`message-item ${isSelf ? 'self' : 'other'}`}>
        {/* 时间分隔线 */}
        {showTime && (
          <div className="message-time-divider">
            <span className="time-divider-content">{formatTime(message.send_time)}</span>
          </div>
        )}

        <div className="message-content-wrapper">
          {/* 头像（仅对方消息显示） */}
          {showAvatar && !isSelf && (
            <div className="message-avatar">
              <div className="avatar-placeholder">{message.content.charAt(0)}</div>
            </div>
          )}

          {/* 消息气泡 */}
          <div className="message-bubble-wrapper">
            <div className="message-bubble">
              {/* 消息内容 - 根据类型渲染 */}
              {message.msg_type === 1 ? (
                renderFileContent()
              ) : (
                <div className="message-text">{message.content}</div>
              )}

              {/* 发送时间和状态 */}
              <div className="message-meta">
                <span className="message-time">{formatTime(message.send_time)}</span>
                {getStatusIcon()}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }
);

// Display name for React DevTools
MessageItem.displayName = 'MessageItem';

export default MessageItem;
