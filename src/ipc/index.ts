// IPC 封装 - 统一导出
// TODO: Phase 4 时根据需要完善更多 IPC 接口

export { chatAPI } from './chat';
export { contactAPI } from './contact';
export { fileAPI } from './file';

// 群组相关 IPC 接口
import { invoke } from '@tauri-apps/api/tauri';
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

  /** 获取群组成员列表 */
  getGroupMembers: async (gid: number) => {
    return await invoke<GroupMember[]>('get_group_members_handler', { gid });
  },
};
