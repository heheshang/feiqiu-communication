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
}
