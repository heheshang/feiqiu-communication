// 工具函数 - 路径相关

/** 获取文件扩展名 */
export function getFileExtension(filePath: string): string {
  const parts = filePath.split('.');
  return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : '';
}

/** 判断是否为图片文件 */
export function isImageFile(filePath: string): boolean {
  const imageExtensions = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg', 'ico'];
  return imageExtensions.includes(getFileExtension(filePath));
}

/** 判断是否为视频文件 */
export function isVideoFile(filePath: string): boolean {
  const videoExtensions = ['mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv', 'webm', 'm4v'];
  return videoExtensions.includes(getFileExtension(filePath));
}

/** 判断是否为音频文件 */
export function isAudioFile(filePath: string): boolean {
  const audioExtensions = ['mp3', 'wav', 'ogg', 'flac', 'aac', 'm4a', 'wma'];
  return audioExtensions.includes(getFileExtension(filePath));
}

/** 判断是否为文档文件 */
export function isDocumentFile(filePath: string): boolean {
  const docExtensions = ['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt'];
  return docExtensions.includes(getFileExtension(filePath));
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

/** 获取目录路径 */
export function getDirectoryPath(filePath: string): string {
  const normalized = normalizePath(filePath);
  const lastSlashIndex = normalized.lastIndexOf('/');
  return lastSlashIndex > 0 ? normalized.substring(0, lastSlashIndex) : '.';
}

/** 规范化路径分隔符 */
export function normalizePath(filePath: string): string {
  return filePath.replace(/\\/g, '/');
}

/** 拼接路径 */
export function joinPaths(...paths: string[]): string {
  const normalized = paths.map(normalizePath);
  return normalized.join('/').replace(/\/+/g, '/');
}

/** 获取 MIME 类型 */
export function getMimeType(filePath: string): string {
  const ext = getFileExtension(filePath);

  const mimeTypes: Record<string, string> = {
    // 图片
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    png: 'image/png',
    gif: 'image/gif',
    bmp: 'image/bmp',
    webp: 'image/webp',
    svg: 'image/svg+xml',
    ico: 'image/x-icon',

    // 视频
    mp4: 'video/mp4',
    avi: 'video/x-msvideo',
    mov: 'video/quicktime',
    wmv: 'video/x-ms-wmv',
    mkv: 'video/x-matroska',
    webm: 'video/webm',

    // 音频
    mp3: 'audio/mpeg',
    wav: 'audio/wav',
    ogg: 'audio/ogg',
    flac: 'audio/flac',
    aac: 'audio/aac',
    m4a: 'audio/mp4',

    // 文档
    pdf: 'application/pdf',
    doc: 'application/msword',
    docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    xls: 'application/vnd.ms-excel',
    xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    ppt: 'application/vnd.ms-powerpoint',
    pptx: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    txt: 'text/plain',

    // 其他
    json: 'application/json',
    xml: 'application/xml',
    zip: 'application/zip',
    rar: 'application/x-rar-compressed',
    '7z': 'application/x-7z-compressed',
  };

  return mimeTypes[ext] || 'application/octet-stream';
}

/** 判断文件是否可预览 */
export function isPreviewable(filePath: string): boolean {
  return (
    isImageFile(filePath) ||
    isVideoFile(filePath) ||
    isAudioFile(filePath) ||
    getFileExtension(filePath) === 'pdf'
  );
}

/** 获取文件图标类型 */
export function getFileIconType(
  filePath: string
): 'image' | 'video' | 'audio' | 'document' | 'archive' | 'other' {
  if (isImageFile(filePath)) return 'image';
  if (isVideoFile(filePath)) return 'video';
  if (isAudioFile(filePath)) return 'audio';
  if (isDocumentFile(filePath)) return 'document';
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(getFileExtension(filePath))) return 'archive';
  return 'other';
}

/** 格式化文件大小 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}
