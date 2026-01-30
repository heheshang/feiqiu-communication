// IPC 封装 - 群组相关
// Phase 7: 群聊功能

import { invoke } from '@tauri-apps/api/core';
import type { GroupInfo, GroupMember } from '../types';

export const groupAPI = {
  /** 创建群组 */
  createGroup: async (groupName: string, creatorUid: number, memberUids: number[]) => {
    return await invoke<number>('create_group_handler', {
      groupName,
      creatorUid,
      memberUids,
    });
  },

  /** 获取群组信息 */
  getGroupInfo: async (gid: number) => {
    return await invoke<GroupInfo>('get_group_info_handler', { gid });
  },

  /** 获取群成员列表 */
  getGroupMembers: async (gid: number) => {
    return await invoke<GroupMember[]>('get_group_members_handler', { gid });
  },

  /** 添加群成员 */
  addGroupMember: async (gid: number, memberUid: number, role: number) => {
    return await invoke<void>('add_group_member_handler', {
      gid,
      memberUid,
      role,
    });
  },

  /** 移除群成员 */
  removeGroupMember: async (gid: number, memberUid: number) => {
    return await invoke<void>('remove_group_member_handler', {
      gid,
      memberUid,
    });
  },

  /** 更新成员角色 */
  updateMemberRole: async (gid: number, memberUid: number, role: number) => {
    return await invoke<void>('update_member_role_handler', {
      gid,
      memberUid,
      role,
    });
  },

  /** 获取用户加入的群组列表 */
  getUserGroups: async (userUid: number) => {
    return await invoke<GroupInfo[]>('get_user_groups_handler', { userUid });
  },

  /** 更新群组信息 */
  updateGroupInfo: async (gid: number, groupName: string, desc: string) => {
    return await invoke<void>('update_group_info_handler', {
      gid,
      groupName,
      desc,
    });
  },

  /** 删除群组 */
  deleteGroup: async (gid: number) => {
    return await invoke<void>('delete_group_handler', { gid });
  },
};
