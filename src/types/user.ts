// TypeScript 类型定义 - 用户相关
// TODO: Phase 4 时根据实际 IPC 接口完善类型定义

/** 在线状态 */
export enum OnlineStatus {
  /** 离线 */
  Offline = 0,
  /** 在线 */
  Online = 1,
  /** 忙碌 */
  Busy = 2,
  /** 离开 */
  Away = 3,
}

/** 用户信息 */
export interface UserInfo {
  uid: number;
  feiq_ip: string;
  feiq_port: number;
  feiq_machine_id: string;
  nickname: string;
  avatar?: string;
  status: OnlineStatus;
  create_time: string;
  update_time: string;
}

/** 联系人信息 */
export interface ContactInfo extends UserInfo {
  group_id?: number;
  remark?: string;
}
