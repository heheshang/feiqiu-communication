// 自定义钩子 - 用户功能

import { useCallback } from 'react';
import { useUserStore } from '../store';

export function useUser() {
  const { currentUser, fetchCurrentUser, updateCurrentUser } = useUserStore();

  /** 获取当前用户信息 */
  const getCurrentUser = useCallback(async () => {
    try {
      await fetchCurrentUser();
      return currentUser;
    } catch (error) {
      console.error('Failed to get current user:', error);
      return undefined;
    }
  }, [currentUser, fetchCurrentUser]);

  /** 更新当前用户信息 */
  const updateUser = useCallback(
    async (updates: { nickname?: string; avatar?: string }) => {
      try {
        await updateCurrentUser(updates);
        return currentUser;
      } catch (error) {
        console.error('Failed to update user:', error);
        return undefined;
      }
    },
    [currentUser, updateCurrentUser]
  );

  return {
    currentUser,
    getCurrentUser,
    updateUser,
  };
}
