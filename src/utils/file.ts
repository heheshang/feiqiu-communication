// 工具函数 - 文件相关
// Phase 6: 文件传输功能

/** 格式化文件大小 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  if (bytes < 1024) return `${bytes} B`;

  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const unitIndex = Math.floor(Math.log(bytes) / Math.log(1024));
  const size = bytes / Math.pow(1024, unitIndex);

  return `${size.toFixed(unitIndex > 1 ? 2 : 0)} ${units[unitIndex]}`;
}

/** 计算传输速度 */
export function calculateSpeed(transferred: number, elapsedMs: number): number {
  if (elapsedMs <= 0) return 0;
  return (transferred / elapsedMs) * 1000; // bytes per second
}

/** 格式化传输速度 */
export function formatSpeed(bytesPerSecond: number): string {
  return `${formatFileSize(bytesPerSecond)}/s`;
}

/** 获取文件图标 */
export function getFileIcon(fileName: string): string {
  const ext = fileName.split('.').pop()?.toLowerCase() || '';

  // 图片
  if (['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'].includes(ext)) {
    return '🖼️';
  }

  // 视频
  if (['mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv', 'webm'].includes(ext)) {
    return '🎬';
  }

  // 音频
  if (['mp3', 'wav', 'flac', 'aac', 'ogg', 'wma'].includes(ext)) {
    return '🎵';
  }

  // 文档
  if (['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx'].includes(ext)) {
    return '📄';
  }

  // 压缩文件
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) {
    return '📦';
  }

  // 代码
  if (['js', 'ts', 'py', 'java', 'c', 'cpp', 'go', 'rs'].includes(ext)) {
    return '💻';
  }

  // 默认文件图标
  return '📎';
}

/** 验证文件名 */
export function isValidFileName(fileName: string): boolean {
  // Windows 文件名非法字符
  // eslint-disable-next-line no-control-regex
  const invalidChars = /[<>:"/\\|?*\x00-\x1f]/;
  const reservedNames = [
    'CON',
    'PRN',
    'AUX',
    'NUL',
    'COM1',
    'COM2',
    'COM3',
    'COM4',
    'COM5',
    'COM6',
    'COM7',
    'COM8',
    'COM9',
    'LPT1',
    'LPT2',
    'LPT3',
    'LPT4',
    'LPT5',
    'LPT6',
    'LPT7',
    'LPT8',
    'LPT9',
  ];

  if (invalidChars.test(fileName)) {
    return false;
  }

  const nameWithoutExt = fileName.split('.')[0];
  if (reservedNames.includes(nameWithoutExt.toUpperCase())) {
    return false;
  }

  return fileName.length > 0 && fileName.length <= 255;
}
