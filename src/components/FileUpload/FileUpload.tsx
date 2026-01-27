// 组件 - 文件上传
// TODO: Phase 6 时完善文件上传组件

import React, { useRef } from 'react';
import { fileAPI } from '../../ipc';

import './FileUpload.less';

interface FileUploadProps {
  sessionType: number;
  targetId: number;
  onUploadStart?: (fileId: number) => void;
}

export const FileUpload: React.FC<FileUploadProps> = ({
  sessionType,
  targetId,
  onUploadStart,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileSelect = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) return;

    // TODO: 在 Tauri 中需要使用 dialog API 来获取真实文件路径
    // 目前先用文件名作为占位符
    const fileName = files[0].name;
    try {
      const fileId = await fileAPI.uploadFile(fileName, sessionType, targetId);
      onUploadStart?.(fileId);
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
      <input
        ref={fileInputRef}
        type="file"
        style={{ display: 'none' }}
        onChange={handleFileSelect}
      />
      <button
        className="upload-btn"
        onClick={() => fileInputRef.current?.click()}
      >
        选择文件
      </button>
    </div>
  );
};
