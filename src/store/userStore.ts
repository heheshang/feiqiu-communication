// 状态管理 - 用户状态
import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import type { UserInfo } from '../types';

interface UserState {
  // 状态
  currentUser: UserInfo | null;
  onlineUsers: Map<string, UserInfo>; // 使用 Map 以 IP 为键，提高查找效率
  contacts: UserInfo[];
  isInitialized: boolean;

  // 操作方法 - 当前用户
  fetchCurrentUser: () => Promise<void>;
  updateCurrentUser: (updates: Partial<Pick<UserInfo, 'nickname' | 'avatar'>>) => Promise<void>;
  setCurrentUser: (user: UserInfo | null) => void;

  // 操作方法 - 在线用户
  setOnlineUsers: (users: UserInfo[]) => void;
  getOnlineUsers: () => UserInfo[];
  addOnlineUser: (user: UserInfo) => void;
  removeOnlineUser: (ip: string) => void;
  updateOnlineUser: (ip: string, updates: Partial<UserInfo>) => void;
  findOnlineUser: (ip: string) => UserInfo | undefined;
  findOnlineUserById: (uid: number) => UserInfo | undefined;

  // 操作方法 - 联系人
  setContacts: (contacts: UserInfo[]) => void;
  addContact: (contact: UserInfo) => void;
  removeContact: (uid: number) => void;
  findContactById: (uid: number) => UserInfo | undefined;

  // 初始化
  initialize: () => Promise<void>;
}

export const useUserStore = create<UserState>()(
  devtools(
    persist(
      (set, get) => ({
        currentUser: null,
        onlineUsers: new Map(),
        contacts: [],
        isInitialized: false,

        // 初始化用户状态
        initialize: async () => {
          if (get().isInitialized) return;

          try {
            await get().fetchCurrentUser();
            set({ isInitialized: true });
          } catch (error) {
            console.error('Failed to initialize user store:', error);
          }
        },

        // 获取当前用户信息
        fetchCurrentUser: async () => {
          try {
            const user = await invoke<UserInfo>('get_current_user_handler');
            set({ currentUser: user });
          } catch (error) {
            console.error('Failed to fetch current user:', error);
            throw error;
          }
        },

        // 更新当前用户信息
        updateCurrentUser: async (updates) => {
          const { currentUser } = get();
          if (!currentUser) {
            throw new Error('No current user found');
          }

          try {
            const updatedUser = await invoke<UserInfo>('update_current_user_handler', {
              uid: currentUser.uid,
              nickname: updates.nickname,
              avatar: updates.avatar,
            });

            set({ currentUser: updatedUser });
          } catch (error) {
            console.error('Failed to update current user:', error);
            throw error;
          }
        },

        // 设置当前用户
        setCurrentUser: (user) => set({ currentUser: user }),

        // 设置在线用户列表
        setOnlineUsers: (users) => {
          const userMap = new Map(users.map((u) => [`${u.feiq_ip}:${u.feiq_port}`, u]));
          set({ onlineUsers: userMap });
        },

        // 获取在线用户列表（转换为数组）
        getOnlineUsers: () => {
          return Array.from(get().onlineUsers.values());
        },

        // 添加在线用户
        addOnlineUser: (user) => {
          set((state) => {
            const newMap = new Map(state.onlineUsers);
            newMap.set(`${user.feiq_ip}:${user.feiq_port}`, user);
            return { onlineUsers: newMap };
          });
        },

        // 移除在线用户
        removeOnlineUser: (ip) => {
          set((state) => {
            const newMap = new Map(state.onlineUsers);
            // 查找并移除匹配 IP 的用户
            for (const [key, user] of state.onlineUsers) {
              if (user.feiq_ip === ip) {
                newMap.delete(key);
              }
            }
            return { onlineUsers: newMap };
          });
        },

        // 更新在线用户信息
        updateOnlineUser: (ip, updates) => {
          set((state) => {
            const newMap = new Map(state.onlineUsers);
            for (const [key, user] of state.onlineUsers) {
              if (user.feiq_ip === ip) {
                newMap.set(key, { ...user, ...updates });
              }
            }
            return { onlineUsers: newMap };
          });
        },

        // 查找在线用户
        findOnlineUser: (ip) => {
          const { onlineUsers } = get();
          for (const user of onlineUsers.values()) {
            if (user.feiq_ip === ip) {
              return user;
            }
          }
          return undefined;
        },

        // 根据 ID 查找在线用户
        findOnlineUserById: (uid) => {
          const { onlineUsers } = get();
          for (const user of onlineUsers.values()) {
            if (user.uid === uid) {
              return user;
            }
          }
          return undefined;
        },

        // 设置联系人列表
        setContacts: (contacts) => set({ contacts }),

        // 添加联系人
        addContact: (contact) =>
          set((state) => ({
            contacts: state.contacts.some((c) => c.uid === contact.uid)
              ? state.contacts
              : [...state.contacts, contact],
          })),

        // 移除联系人
        removeContact: (uid) =>
          set((state) => ({
            contacts: state.contacts.filter((c) => c.uid !== uid),
          })),

        // 根据 ID 查找联系人
        findContactById: (uid) => {
          const { contacts } = get();
          return contacts.find((c) => c.uid === uid);
        },
      }),
      {
        name: 'feiqiu-user-storage',
        partialize: (state) => ({
          currentUser: state.currentUser,
          contacts: state.contacts,
        }),
        // 不持久化 onlineUsers 和 isInitialized
      }
    )
  )
);

// 选择器辅助函数
export const selectCurrentUser = (state: UserState) => state.currentUser;
export const selectOnlineUsers = (state: UserState) => Array.from(state.onlineUsers.values());
export const selectContacts = (state: UserState) => state.contacts;
