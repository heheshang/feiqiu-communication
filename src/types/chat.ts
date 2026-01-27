// TypeScript 类型定义 - 聊天相关
// TODO: Phase 4 时根据实际 IPC 接口完善类型定义

/** 会话类型 */
export enum SessionType {
  /** 单聊 */
  Single = 0,
  /** 群聊 */
  Group = 1,
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
  msg_id: number;
  session_type: SessionType;
  target_id: number;
  sender_uid: number;
  content: string;
  msg_type: number;
  status: MessageStatus;
  create_time: string;
  update_time: string;
}

/** 聊天会话 */
export interface ChatSession {
  session_id: number;
  session_type: SessionType;
  target_id: number;
  target_name: string;
  last_message: string;
  last_time: string;
  unread_count: number;
}
