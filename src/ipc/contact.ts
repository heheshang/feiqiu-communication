// IPC 封装 - 联系人相关
// TODO: Phase 4 时完善联系人 IPC 接口

import { invoke } from '@tauri-apps/api/core';
import type { UserInfo } from '../types';

export const contactAPI = {
  /** 获取联系人列表 */
  getContactList: async (ownerUid: number) => {
    return await invoke<UserInfo[]>('get_contact_list_handler', { ownerUid });
  },

  /** 获取在线用户列表 */
  getOnlineUsers: async (ownerUid: number) => {
    return await invoke<UserInfo[]>('get_online_users_handler', { ownerUid });
  },
};
