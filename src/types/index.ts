// TypeScript 类型定义 - 统一导出
// TODO: Phase 4 时根据需要完善更多类型定义

export * from './chat';
export * from './user';

/** 文件传输信息 */
export interface FileInfo {
  fid: number;
  file_name: string;
  file_path: string;
  file_size: number;
  mime_type: string;
}

/** 文件传输状态 */
export enum TransferStatus {
  /** 等待中 */
  Pending = 0,
  /** 传输中 */
  Transferring = 1,
  /** 已完成 */
  Completed = 2,
  /** 已取消 */
  Cancelled = -2,
  /** 失败 */
  Failed = -1,
}

/** 文件传输进度 */
export interface TransferProgress {
  file_id: number;
  progress: number;
  total: number;
  speed: number;
  transferred: number;
  status?: TransferStatus; // 可选的状态字段
}

/** 待恢复的传输信息 */
export interface PendingTransfer {
  tid: number;
  file_id: number;
  file_name: string;
  file_path: string;
  transferred: number;
  file_size: number;
  status: TransferStatus;
  target_ip: string;
  direction: number; // 0=下载, 1=上传
}

/** 群组信息 */
export interface GroupInfo {
  gid: number;
  group_name: string;
  creator_uid: number;
  member_count: number;
  avatar?: string;
  create_time: string;
  update_time: string;
}

/** 群组成员 */
export interface GroupMember {
  uid: number;
  gid: number;
  nickname: string;
  role: number;
  join_time: string;
}
