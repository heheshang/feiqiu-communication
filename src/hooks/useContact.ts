// 自定义钩子 - 联系人功能
// TODO: Phase 4 时完善联系人功能钩子

import { useState, useCallback } from 'react';
import { useIPC } from './useIPC';
import type { UserInfo } from '../types';

export function useContact() {
  const { invoke } = useIPC();
  const [contacts, setContacts] = useState<UserInfo[]>([]);
  const [onlineUsers, setOnlineUsers] = useState<UserInfo[]>([]);

  /** 获取联系人列表 */
  const getContactList = useCallback(async () => {
    const result = await invoke<UserInfo[]>('get_contact_list_handler', { ownerUid: 0 });
    setContacts(result);
    return result;
  }, [invoke]);

  /** 获取在线用户列表 */
  const getOnlineUsers = useCallback(async () => {
    const result = await invoke<UserInfo[]>('get_online_users_handler', { ownerUid: 0 });
    setOnlineUsers(result);
    return result;
  }, [invoke]);

  return {
    contacts,
    onlineUsers,
    getContactList,
    getOnlineUsers,
  };
}
