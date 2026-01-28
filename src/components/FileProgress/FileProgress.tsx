// 组件 - 文件传输进度
// Phase 6: 文件传输功能

import React from 'react';
import type { TransferStatus } from '../../types';
import { formatFileSize } from '../../utils/file';

import './FileProgress.less';

interface FileProgressProps {
  fileId: number;
  fileName: string;
  progress: number;
  total: number;
  speed: number;
  status: TransferStatus;
  onCancel?: (fileId: number) => void;
}

export const FileProgress: React.FC<FileProgressProps> = React.memo(
  ({ fileId, fileName, progress, total, speed, status, onCancel }) => {
    const progressPercent = Math.min(100, Math.round((progress / total) * 100));
    const remainingBytes = total - progress;
    const remainingSeconds = speed > 0 ? Math.round(remainingBytes / speed) : 0;

    const formatTime = (seconds: number): string => {
      if (seconds < 60) return `${seconds}秒`;
      if (seconds < 3600) return `${Math.floor(seconds / 60)}分${seconds % 60}秒`;
      return `${Math.floor(seconds / 3600)}小时${Math.floor((seconds % 3600) / 60)}分`;
    };

    const getStatusText = (): string => {
      switch (status) {
        case 0: // Pending
          return '等待中...';
        case 1: // Transferring
          return `传输中 ${formatTime(remainingSeconds)}`;
        case 2: // Completed
          return '传输完成';
        case -2: // Cancelled
          return '已取消';
        case -1: // Failed
          return '传输失败';
        default:
          return '';
      }
    };

    const getSpeedText = (): string => {
      if (status !== 1) return '';
      return `${formatFileSize(speed)}/s`;
    };

    return (
      <div className={`file-progress file-progress--status-${status}`}>
        <div className="file-progress-header">
          <span className="file-progress-name">{fileName}</span>
          {status === 1 && onCancel && (
            <button
              className="file-progress-cancel"
              onClick={() => onCancel(fileId)}
              title="取消传输"
            >
              ✕
            </button>
          )}
        </div>

        <div className="file-progress-info">
          <span className="file-progress-status">{getStatusText()}</span>
          <span className="file-progress-speed">{getSpeedText()}</span>
          <span className="file-progress-size">
            {formatFileSize(progress)} / {formatFileSize(total)}
          </span>
        </div>

        <div className="file-progress-bar-container">
          <div className="file-progress-bar" style={{ width: `${progressPercent}%` }} />
        </div>

        <div className="file-progress-percent">{progressPercent}%</div>
      </div>
    );
  }
);

FileProgress.displayName = 'FileProgress';
