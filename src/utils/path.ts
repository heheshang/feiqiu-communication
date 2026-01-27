// 工具函数 - 路径相关
// TODO: Phase 4 时完善路径处理

/** 获取文件扩展名 */
export function getFileExtension(filePath: string): string {
  const parts = filePath.split('.');
  return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : '';
}

/** 判断是否为图片文件 */
export function isImageFile(filePath: string): boolean {
  const imageExtensions = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'];
  return imageExtensions.includes(getFileExtension(filePath));
}

/** 判断是否为视频文件 */
export function isVideoFile(filePath: string): boolean {
  const videoExtensions = ['mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv', 'webm'];
  return videoExtensions.includes(getFileExtension(filePath));
}

/** 获取文件名（不含扩展名） */
export function getFileName(filePath: string): string {
  const parts = filePath.split(/[/\\]/);
  const fileName = parts[parts.length - 1];
  const dotIndex = fileName.lastIndexOf('.');
  return dotIndex > 0 ? fileName.substring(0, dotIndex) : fileName;
}

/** 获取文件名（含扩展名） */
export function getFileNameWithExtension(filePath: string): string {
  const parts = filePath.split(/[/\\]/);
  return parts[parts.length - 1];
}

/** 规范化路径分隔符 */
export function normalizePath(filePath: string): string {
  return filePath.replace(/\\/g, '/');
}
