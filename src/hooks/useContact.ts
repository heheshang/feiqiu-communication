// 自定义钩子 - 联系人功能

import { useCallback, useEffect } from 'react';
import { useUserStore } from '../store';
import { contactService } from '../services';
import type { UserInfo, UserSearchParams } from '../types';

export function useContact() {
  const {
    onlineUsers,
    getOnlineUsers,
    setOnlineUsers,
    addOnlineUser,
    removeOnlineUser,
    findOnlineUser,
  } = useUserStore();

  /** 初始化在线用户列表 */
  useEffect(() => {
    getOnlineUsers();
  }, [getOnlineUsers]);

  /** 获取在线用户列表（转为数组） */
  const getOnlineUsersList = useCallback((): UserInfo[] => {
    return Array.from(onlineUsers.values());
  }, [onlineUsers]);

  /** 搜索用户 */
  const searchUsers = useCallback(
    (params: UserSearchParams): UserInfo[] => {
      let result = getOnlineUsersList();

      // 关键词过滤
      if (params.keyword) {
        const keyword = params.keyword.toLowerCase();
        result = result.filter(
          (user) =>
            user.nickname.toLowerCase().includes(keyword) ||
            user.feiq_machine_id.toLowerCase().includes(keyword)
        );
      }

      // 状态过滤
      if (params.status !== undefined) {
        result = result.filter((user) => user.status === params.status);
      }

      // 分页
      if (params.page && params.page_size) {
        const start = (params.page - 1) * params.page_size;
        const end = start + params.page_size;
        result = result.slice(start, end);
      }

      return result;
    },
    [getOnlineUsersList]
  );

  /** 根据 IP 查找用户 */
  const findUserByIp = useCallback(
    (ip: string): UserInfo | undefined => {
      return findOnlineUser(ip);
    },
    [findOnlineUser]
  );

  /** 添加在线用户 */
  const addOnline = useCallback(
    (user: UserInfo) => {
      addOnlineUser(user);
    },
    [addOnlineUser]
  );

  /** 移除在线用户 */
  const removeOnline = useCallback(
    (ip: string) => {
      removeOnlineUser(ip);
    },
    [removeOnlineUser]
  );

  /** 刷新在线用户列表 */
  const refreshOnlineUsers = useCallback(async () => {
    const { currentUser } = useUserStore.getState();
    if (!currentUser) {
      console.error('No current user found');
      return;
    }

    try {
      const users = await contactService.getOnlineUsers(currentUser.uid);
      setOnlineUsers(users);
    } catch (error) {
      console.error('Failed to refresh online users:', error);
    }
  }, [setOnlineUsers]);

  /** 获取在线用户数量 */
  const getOnlineCount = useCallback((): number => {
    return onlineUsers.size;
  }, [onlineUsers]);

  return {
    // 状态
    onlineUsers: getOnlineUsersList(),
    onlineCount: getOnlineCount(),

    // 操作
    searchUsers,
    findUserByIp,
    addOnline,
    removeOnline,
    refreshOnlineUsers,
  };
}
