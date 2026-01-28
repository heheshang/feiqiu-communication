// 组件 - 文件上传
// TODO: Phase 6 时完善文件上传组件

import React, { useRef } from 'react';
import type { SessionType } from '../../types';

import './FileUpload.less';

interface FileUploadProps {
  sessionType: SessionType;
  targetId: number;
  onUpload?: (file: File) => Promise<void>;
  onClose?: () => void;
}

export const FileUpload: React.FC<FileUploadProps> = ({
  // sessionType, // TODO: Phase 6 时使用
  // targetId,    // TODO: Phase 6 时使用
  onUpload,
  onClose,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileSelect = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) return;

    const file = files[0];
    try {
      await onUpload?.(file);
      onClose?.();
    } catch (error) {
      console.error('文件上传失败:', error);
    }

    // Reset input
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  return (
    <div className="file-upload">
      <div className="file-upload-header">
        <span>发送文件</span>
        <button className="close-btn" onClick={onClose}>
          ×
        </button>
      </div>
      <div className="file-upload-body">
        <input
          ref={fileInputRef}
          type="file"
          style={{ display: 'none' }}
          onChange={handleFileSelect}
        />
        <button className="upload-btn" onClick={() => fileInputRef.current?.click()}>
          选择文件
        </button>
      </div>
    </div>
  );
};
