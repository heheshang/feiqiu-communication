// 状态管理 - 群组状态
import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { groupService } from '../services/groupService';
import type { GroupInfo, GroupMember } from '../types';

interface GroupState {
  // 状态
  groups: GroupInfo[];
  currentGroup: GroupInfo | null;
  members: Record<number, GroupMember[]>;
  isLoadingGroups: boolean;
  isLoadingMembers: Set<number>;
  isCreatingGroup: boolean;

  // 操作方法 - 群组
  fetchGroups: (userUid: number) => Promise<void>;
  setGroups: (groups: GroupInfo[]) => void;
  setCurrentGroup: (group: GroupInfo | null) => void;
  createGroup: (groupName: string, creatorUid: number, memberUids: number[]) => Promise<number>;

  // 操作方法 - 成员
  fetchGroupMembers: (gid: number) => Promise<void>;
  addMember: (gid: number, memberUid: number, role: number) => Promise<void>;
  removeMember: (gid: number, memberUid: number) => Promise<void>;

  // 辅助方法
  getMembersByGroup: (gid: number) => GroupMember[];
}

export const useGroupStore = create<GroupState>()(
  devtools((set, get) => ({
    groups: [],
    currentGroup: null,
    members: {},
    isLoadingGroups: false,
    isLoadingMembers: new Set(),
    isCreatingGroup: false,

    // 获取群组列表
    fetchGroups: async (userUid: number) => {
      set({ isLoadingGroups: true });

      try {
        const groups = await groupService.getUserGroups(userUid);
        set({ groups, isLoadingGroups: false });
      } catch (error) {
        console.error('Failed to fetch groups:', error);
        set({ isLoadingGroups: false });
        throw error;
      }
    },

    // 设置群组列表
    setGroups: (groups) => set({ groups }),

    // 设置当前群组
    setCurrentGroup: (group) => set({ currentGroup: group }),

    // 创建群组
    createGroup: async (groupName: string, creatorUid: number, memberUids: number[]) => {
      set({ isCreatingGroup: true });

      try {
        const gid = await groupService.createGroup(groupName, creatorUid, memberUids);

        // 创建成功后重新获取群组列表
        await get().fetchGroups(creatorUid);

        set({ isCreatingGroup: false });
        return gid;
      } catch (error) {
        console.error('Failed to create group:', error);
        set({ isCreatingGroup: false });
        throw error;
      }
    },

    // 获取群组成员列表
    fetchGroupMembers: async (gid: number) => {
      set((state) => ({
        isLoadingMembers: new Set(state.isLoadingMembers).add(gid),
      }));

      try {
        const members = await groupService.getGroupMembers(gid);

        set((state) => ({
          members: { ...state.members, [gid]: members },
          isLoadingMembers: (() => {
            const newSet = new Set(state.isLoadingMembers);
            newSet.delete(gid);
            return newSet;
          })(),
        }));
      } catch (error) {
        console.error('Failed to fetch group members:', error);
        set((state) => ({
          isLoadingMembers: (() => {
            const newSet = new Set(state.isLoadingMembers);
            newSet.delete(gid);
            return newSet;
          })(),
        }));
        throw error;
      }
    },

    // 添加群成员
    addMember: async (gid: number, memberUid: number, role: number) => {
      try {
        await groupService.addGroupMember(gid, memberUid, role);

        // 添加成功后重新获取成员列表
        await get().fetchGroupMembers(gid);
      } catch (error) {
        console.error('Failed to add member:', error);
        throw error;
      }
    },

    // 移除群成员
    removeMember: async (gid: number, memberUid: number) => {
      try {
        await groupService.removeGroupMember(gid, memberUid);

        // 移除成功后重新获取成员列表
        await get().fetchGroupMembers(gid);
      } catch (error) {
        console.error('Failed to remove member:', error);
        throw error;
      }
    },

    // 获取群组成员
    getMembersByGroup: (gid: number) => {
      return get().members[gid] || [];
    },
  }))
);
