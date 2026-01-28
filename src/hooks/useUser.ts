// 自定义钩子 - 用户功能

import { useState, useCallback } from 'react';
import { useIPC } from './useIPC';
import type { UserInfo } from '../types';

export function useUser() {
  const { invoke } = useIPC();
  const [currentUser, setCurrentUser] = useState<UserInfo | undefined>(undefined);

  /** 获取当前用户信息 */
  const getCurrentUser = useCallback(async () => {
    try {
      // TODO: 实现获取当前用户的 IPC 调用
      // const user = await invoke<UserInfo>('get_current_user_handler');
      // setCurrentUser(user);
      // return user;

      // 临时返回默认用户
      const now = new Date().toISOString();
      const defaultUser: UserInfo = {
        uid: 0,
        nickname: '我',
        feiq_ip: '127.0.0.1',
        feiq_port: 2425,
        feiq_machine_id: 'default',
        avatar: undefined,
        status: 1,
        create_time: now,
        update_time: now,
      };
      setCurrentUser(defaultUser);
      return defaultUser;
    } catch (error) {
      console.error('Failed to get current user:', error);
      return undefined;
    }
  }, [invoke]);

  return {
    currentUser,
    getCurrentUser,
  };
}
