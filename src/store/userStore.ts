// 状态管理 - 用户状态
// TODO: Phase 4 时完善用户状态管理

import { create } from 'zustand';
import type { UserInfo } from '../types';

interface UserState {
  currentUser: UserInfo | null;
  onlineUsers: UserInfo[];
  contacts: UserInfo[];
  setCurrentUser: (user: UserInfo | null) => void;
  setOnlineUsers: (users: UserInfo[]) => void;
  addOnlineUser: (user: UserInfo) => void;
  removeOnlineUser: (ip: string) => void;
  setContacts: (contacts: UserInfo[]) => void;
}

export const useUserStore = create<UserState>((set) => ({
  currentUser: null,
  onlineUsers: [],
  contacts: [],

  setCurrentUser: (user) => set({ currentUser: user }),

  setOnlineUsers: (users) => set({ onlineUsers: users }),

  addOnlineUser: (user) =>
    set((state) => ({
      onlineUsers: state.onlineUsers.some((u) => u.feiq_ip === user.feiq_ip)
        ? state.onlineUsers
        : [...state.onlineUsers, user],
    })),

  removeOnlineUser: (ip) =>
    set((state) => ({
      onlineUsers: state.onlineUsers.filter((u) => u.feiq_ip !== ip),
    })),

  setContacts: (contacts) => set({ contacts }),
}));
