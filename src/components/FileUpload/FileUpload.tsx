// 组件 - 文件上传

import React, { useState, useRef, useCallback } from 'react';
import type { SessionType } from '../../types';
import { getFileIconType } from '../../utils/path';

import './FileUpload.less';

interface FileUploadProps {
  sessionType: SessionType;
  targetId: number;
  onUpload?: (file: File) => Promise<void>;
  onClose?: () => void;
}

interface SelectedFile {
  file: File;
  preview?: string;
}

export const FileUpload: React.FC<FileUploadProps> = ({
  sessionType: _sessionType,
  targetId: _targetId,
  onUpload,
  onClose,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [selectedFile, setSelectedFile] = useState<SelectedFile | null>(null);
  const [isUploading, setIsUploading] = useState(false);
  const [dragActive, setDragActive] = useState(false);

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const handleFileSelect = useCallback((file: File) => {
    // Check if file is an image and create preview
    if (file.type.startsWith('image/')) {
      const reader = new FileReader();
      reader.onloadend = () => {
        setSelectedFile({
          file,
          preview: reader.result as string,
        });
      };
      reader.readAsDataURL(file);
    } else {
      setSelectedFile({ file });
    }
  }, []);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) return;

    handleFileSelect(files[0]);
  };

  const handleDrop = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      e.stopPropagation();
      setDragActive(false);

      const files = e.dataTransfer.files;
      if (!files || files.length === 0) return;

      handleFileSelect(files[0]);
    },
    [handleFileSelect]
  );

  const handleDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
  }, []);

  const handleUpload = async () => {
    if (!selectedFile) return;

    setIsUploading(true);
    try {
      await onUpload?.(selectedFile.file);
      onClose?.();
    } catch (error) {
      console.error('文件上传失败:', error);
      setIsUploading(false);
    }
  };

  const handleClear = () => {
    setSelectedFile(null);
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const getFileIconClass = (): string => {
    if (!selectedFile) return '';
    const iconType = getFileIconType(selectedFile.file.name);
    return `file-icon-${iconType}`;
  };

  return (
    <div className="file-upload">
      <div className="file-upload-header">
        <span className="header-title">发送文件</span>
        <button className="close-btn" onClick={onClose} aria-label="关闭">
          <svg viewBox="0 0 24 24" fill="none" width="16" height="16">
            <path
              d="M18 6L6 18M6 6L18 18"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
            />
          </svg>
        </button>
      </div>

      <div
        className={`file-upload-body ${dragActive ? 'drag-active' : ''}`}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
      >
        <input
          ref={fileInputRef}
          type="file"
          style={{ display: 'none' }}
          onChange={handleInputChange}
        />

        {selectedFile ? (
          <div className="file-preview">
            {selectedFile.preview ? (
              <div className="file-preview-image">
                <img src={selectedFile.preview} alt={selectedFile.file.name} />
              </div>
            ) : (
              <div className={`file-preview-icon ${getFileIconClass()}`}>
                <svg viewBox="0 0 24 24" fill="none" width="48" height="48">
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
            )}

            <div className="file-info">
              <div className="file-name" title={selectedFile.file.name}>
                {selectedFile.file.name}
              </div>
              <div className="file-size">{formatFileSize(selectedFile.file.size)}</div>
            </div>

            <button className="clear-btn" onClick={handleClear} aria-label="清除">
              <svg viewBox="0 0 24 24" fill="none" width="16" height="16">
                <path
                  d="M18 6L6 18M6 6L18 18"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                />
              </svg>
            </button>
          </div>
        ) : (
          <div className="upload-placeholder">
            <svg viewBox="0 0 24 24" fill="none" width="48" height="48">
              <path
                d="M21 15V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V15"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
              <path
                d="M17 8L12 3M12 3L7 8M12 3V15"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
            <p className="upload-hint">拖拽文件到此处或点击选择</p>
          </div>
        )}

        <div className="file-upload-actions">
          {!selectedFile && (
            <button className="upload-btn select-btn" onClick={() => fileInputRef.current?.click()}>
              选择文件
            </button>
          )}
          {selectedFile && (
            <button className="upload-btn send-btn" onClick={handleUpload} disabled={isUploading}>
              {isUploading ? (
                <>
                  <span className="loading-spinner"></span>
                  发送中...
                </>
              ) : (
                <>
                  <svg viewBox="0 0 24 24" fill="none" width="16" height="16">
                    <path
                      d="M22 2L11 13M22 2L15 22M22 2H11M22 2V13"
                      stroke="currentColor"
                      strokeWidth="2"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                  </svg>
                  发送
                </>
              )}
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
