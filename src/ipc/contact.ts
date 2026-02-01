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
