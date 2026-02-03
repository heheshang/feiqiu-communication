import { invoke } from '@tauri-apps/api/core';
import type { Contact, UserInfo } from '../types';

export const contactAPI = {
  /** 获取联系人列表 */
  getContactList: async (ownerUid: number) => {
    return await invoke<Contact[]>('get_contact_list_handler', { ownerUid });
  },

  /** 获取在线用户列表 */
  getOnlineUsers: async () => {
    return await invoke<UserInfo[]>('get_online_users_handler');
  },

  /** 添加联系人 */
  addContact: async (ownerUid: number, contactUid: number, remark?: string, tag?: string) => {
    return await invoke<number>('add_contact_handler', { ownerUid, contactUid, remark, tag });
  },

  /** 更新联系人信息 */
  updateContact: async (id: number, remark?: string, tag?: string) => {
    return await invoke<void>('update_contact_handler', { id, remark, tag });
  },

  /** 删除联系人 */
  deleteContact: async (id: number) => {
    return await invoke<void>('delete_contact_handler', { id });
  },

  /** 检查是否已添加联系人 */
  isContact: async (ownerUid: number, contactUid: number) => {
    return await invoke<boolean>('is_contact_handler', { ownerUid, contactUid });
  },
};
