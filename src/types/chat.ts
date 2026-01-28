// TypeScript 类型定义 - 聊天相关

/** 会话类型 */
export enum SessionType {
  /** 单聊 */
  Single = 0,
  /** 群聊 */
  Group = 1,
}

/** 消息类型 */
export enum MessageType {
  /** 文字消息 */
  Text = 0,
  /** 文件消息 */
  File = 1,
  /** Emoji 消息 */
  Emoji = 2,
  /** 图片消息 */
  Image = 3,
  /** 语音消息 */
  Voice = 4,
  /** 视频消息 */
  Video = 5,
}

/** 消息状态 */
export enum MessageStatus {
  /** 发送中 */
  Sending = 0,
  /** 已发送 */
  Sent = 1,
  /** 已读 */
  Read = 2,
  /** 发送失败 */
  Failed = -1,
}

/** 聊天消息 */
export interface ChatMessage {
  /** 消息 ID */
  mid: number;
  /** 会话类型 */
  session_type: SessionType;
  /** 目标 ID */
  target_id: number;
  /** 发送者 UID */
  sender_uid: number;
  /** 消息编号（用于已读回执） */
  msg_no?: string;
  /** 发送者 IP（用于发送已读回执） */
  sender_ip?: string;
  /** 消息类型 */
  msg_type: MessageType;
  /** 消息内容 */
  content: string;
  /** 发送时间 */
  send_time: string;
  /** 消息状态 */
  status: MessageStatus;
  /** 文件信息（当 msg_type = File 时） */
  file_info?: FileAttachment;
  /** 是否撤回 */
  is_revoked?: boolean;
  /** 扩展数据 */
  extra?: Record<string, unknown>;
}

/** 文件附件信息 */
export interface FileAttachment {
  /** 文件 ID */
  fid: number;
  /** 文件名 */
  file_name: string;
  /** 文件大小 */
  file_size: number;
  /** 文件类型 */
  file_type: string;
  /** 文件路径（本地） */
  file_path?: string;
  /** 下载地址 */
  download_url?: string;
  /** 缩略图（图片用） */
  thumbnail?: string;
}

/** 聊天会话 */
export interface ChatSession {
  /** 会话 ID */
  sid: number;
  /** 所有者 UID */
  owner_uid: number;
  /** 会话类型 */
  session_type: SessionType;
  /** 目标 ID */
  target_id: number;
  /** 最后消息 ID */
  last_msg_id: number | null;
  /** 未读数量 */
  unread_count: number;
  /** 更新时间 */
  update_time: string;
  /** 会话名称（缓存的，用于显示） */
  session_name?: string;
  /** 会话头像（缓存的，用于显示） */
  session_avatar?: string;
  /** 最后消息内容（缓存的，用于显示） */
  last_message?: string;
  /** 最后消息时间 */
  last_message_time?: string;
}

/** 发送消息参数 */
export interface SendMessageParams {
  /** 会话类型 */
  session_type: SessionType;
  /** 目标 ID */
  target_id: number;
  /** 消息内容 */
  content: string;
  /** 发送者 UID */
  owner_uid: number;
  /** 消息类型 */
  msg_type?: MessageType;
  /** 文件列表（发送文件时） */
  files?: FileAttachment[];
}

/** 获取消息历史参数 */
export interface GetHistoryParams {
  /** 会话类型 */
  session_type: SessionType;
  /** 目标 ID */
  target_id: number;
  /** 页码 */
  page: number;
  /** 每页数量 */
  page_size?: number;
}
