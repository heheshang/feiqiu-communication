// TypeScript 类型定义 - 用户相关

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
  /** 隐身 */
  Invisible = 4,
}

/** 用户信息 */
export interface UserInfo {
  /** 用户 ID */
  uid: number;
  /** 飞秋 IP 地址 */
  feiq_ip: string;
  /** 飞秋端口 */
  feiq_port: number;
  /** 飞秋机器 ID */
  feiq_machine_id: string;
  /** 昵称 */
  nickname: string;
  /** 头像 */
  avatar?: string;
  /** 在线状态 */
  status: OnlineStatus;
  /** 创建时间 */
  create_time?: string;
  /** 更新时间 */
  update_time?: string;
}

/** 联系人信息 */
export interface ContactInfo extends UserInfo {
  /** 联系人 ID */
  id: number;
  /** 所有者 UID */
  owner_uid: number;
  /** 联系人 UID */
  contact_uid: number;
  /** 分组 ID */
  group_id?: number;
  /** 备注 */
  remark?: string;
  /** 标签 */
  tag?: string;
  /** 创建时间 */
  create_time?: string;
  /** 更新时间 */
  update_time?: string;
}

/** 联系人分组 */
export interface ContactGroup {
  /** 分组 ID */
  id: number;
  /** 所有者 UID */
  owner_uid: number;
  /** 分组名称 */
  group_name: string;
  /** 排序顺序 */
  sort_order: number;
  /** 创建时间 */
  create_time?: string;
  /** 更新时间 */
  update_time?: string;
}

/** 用户状态更新参数 */
export interface UpdateUserStatusParams {
  /** 用户 ID */
  uid: number;
  /** 在线状态 */
  status?: OnlineStatus;
  /** 昵称 */
  nickname?: string;
  /** 头像 */
  avatar?: string;
}

/** 用户搜索参数 */
export interface UserSearchParams {
  /** 搜索关键词 */
  keyword?: string;
  /** 在线状态过滤 */
  status?: OnlineStatus;
  /** 分组 ID 过滤 */
  group_id?: number;
  /** 页码 */
  page?: number;
  /** 每页数量 */
  page_size?: number;
}
