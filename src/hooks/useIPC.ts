// 自定义钩子 - IPC 通信
import { invoke } from '@tauri-apps/api/core';
import type {
  UserInfo,
  ChatMessage,
  ChatSession,
  Contact,
  PendingTransfer,
  GroupInfo,
  GroupMember,
} from '../types';

/** IPC 调用错误类 */
export class IPCError extends Error {
  constructor(
    public command: string,
    public originalError: unknown,
    message?: string
  ) {
    super(message || `IPC call failed: ${command}`);
    this.name = 'IPCError';
  }
}

/** IPC 调用选项 */
interface InvokeOptions {
  /** 是否静默错误（不打印到控制台） */
  silent?: boolean;
  /** 重试次数 */
  retries?: number;
  /** 超时时间（毫秒） */
  timeout?: number;
}

/** IPC 调用结果 */
interface InvokeResult<T> {
  data?: T;
  error?: Error;
  isLoading: boolean;
}

/**
 * IPC 通信钩子
 * 提供统一的 IPC 调用接口和错误处理
 */
export function useIPC() {
  /**
   * 基础 IPC 调用方法
   * @param command Tauri 命令名称
   * @param args 参数对象
   * @param options 调用选项
   * @returns Promise<T>
   */
  const invokeCommand = async <T>(
    command: string,
    args?: Record<string, unknown>,
    options?: InvokeOptions
  ): Promise<T> => {
    const { silent = false, retries = 0, timeout = 30000 } = options || {};

    let lastError: unknown;

    // 重试逻辑
    for (let attempt = 0; attempt <= retries; attempt++) {
      try {
        // 创建超时 Promise
        const timeoutPromise = new Promise<never>((_, reject) => {
          setTimeout(() => reject(new Error(`IPC timeout after ${timeout}ms`)), timeout);
        });

        // 执行 IPC 调用
        const result = await Promise.race([invoke<T>(command, args), timeoutPromise]);

        return result;
      } catch (error) {
        lastError = error;

        // 最后一次尝试失败，不再重试
        if (attempt === retries) {
          break;
        }

        // 指数退避重试
        await new Promise((resolve) => setTimeout(resolve, Math.pow(2, attempt) * 100));
      }
    }

    // 所有尝试都失败
    const ipcError = new IPCError(command, lastError);
    if (!silent) {
      console.error(`[IPC Error] ${command}:`, lastError);
    }
    throw ipcError;
  };

  /**
   * 带加载状态的 IPC 调用
   * @param command Tauri 命令名称
   * @param args 参数对象
   * @param options 调用选项
   * @returns Promise<InvokeResult<T>>
   */
  const invokeWithLoading = async <T>(
    command: string,
    args?: Record<string, unknown>,
    options?: InvokeOptions
  ): Promise<InvokeResult<T>> => {
    try {
      const data = await invokeCommand<T>(command, args, options);
      return { data, isLoading: false };
    } catch (error) {
      return {
        error: error instanceof Error ? error : new Error(String(error)),
        isLoading: false,
      };
    }
  };

  /**
   * 批量 IPC 调用
   * @param calls 调用数组
   * @returns Promise<Array<T>>
   */
  const invokeBatch = async <T>(
    calls: Array<{ command: string; args?: Record<string, unknown> }>
  ): Promise<Array<T | Error>> => {
    const results = await Promise.allSettled(
      calls.map((call) => invokeCommand<T>(call.command, call.args))
    );

    return results.map((result) => (result.status === 'fulfilled' ? result.value : result.reason));
  };

  // 用户相关 IPC 调用
  const userApi = {
    /** 获取当前用户 */
    getCurrentUser: () => invokeCommand<UserInfo>('get_current_user_handler'),

    /** 更新当前用户信息 */
    updateCurrentUser: (uid: number, nickname?: string, avatar?: string) =>
      invokeCommand<UserInfo>('update_current_user_handler', {
        uid,
        nickname,
        avatar,
      }),
  };

  // 聊天相关 IPC 调用
  const chatApi = {
    /** 获取聊天历史 */
    getHistory: (sessionType: number, targetId: number, page: number, pageSize = 50) =>
      invokeCommand<ChatMessage[]>('get_chat_history_handler', {
        sessionType,
        targetId,
        page,
        pageSize,
      }),

    /** 发送文本消息 */
    sendMessage: (sessionType: number, targetId: number, content: string, ownerUid: number) =>
      invokeCommand<number>('send_text_message_handler', {
        sessionType,
        targetId,
        content,
        ownerUid,
      }),

    /** 获取会话列表 */
    getSessionList: (ownerUid: number) =>
      invokeCommand<ChatSession[]>('get_session_list_handler', { ownerUid }),

    /** 标记消息已读 */
    markMessagesRead: (sessionType: number, targetId: number, ownerUid: number) =>
      invokeCommand<void>('mark_messages_read_handler', {
        sessionType,
        targetId,
        ownerUid,
      }),

    /** 标记单条消息已读并发送回执 */
    markMessageReadAndSendReceipt: (mid: number, msgNo: string, targetIp: string) =>
      invokeCommand<void>('mark_message_read_and_send_receipt', {
        mid,
        msgNo,
        targetIp,
      }),

    /** 重试发送消息 */
    retrySendMessage: (mid: number, sessionType: number, targetId: number, ownerUid: number) =>
      invokeCommand<void>('retry_send_message', {
        mid,
        sessionType,
        targetId,
        ownerUid,
      }),
  };

  // 联系人相关 IPC 调用
  const contactApi = {
    /** 获取联系人列表 */
    getContactList: () => invokeCommand<Contact[]>('get_contact_list_handler'),

    /** 获取在线用户 */
    getOnlineUsers: () => invokeCommand<UserInfo[]>('get_online_users_handler'),
  };

  // 文件相关 IPC 调用
  const fileApi = {
    /** 发送文件请求 */
    sendFileRequest: (filePaths: string[], targetIp: string, ownerUid: number) =>
      invokeCommand<number>('send_file_request_handler', {
        file_paths: filePaths,
        target_ip: targetIp,
        owner_uid: ownerUid,
      }),

    /** 接受文件请求 */
    acceptFileRequest: (packetNo: string, fileId: number, offset: number, targetIp: string) =>
      invokeCommand<void>('accept_file_request_handler', {
        packet_no: packetNo,
        file_id: fileId,
        offset,
        target_ip: targetIp,
      }),

    /** 拒绝文件请求 */
    rejectFileRequest: (packetNo: string, targetIp: string) =>
      invokeCommand<void>('reject_file_request_handler', {
        packet_no: packetNo,
        target_ip: targetIp,
      }),

    /** 取消文件传输 */
    cancelUpload: (fid: number) => invokeCommand<void>('cancel_upload_handler', { fid }),

    /** 获取待恢复的传输列表 */
    getPendingTransfers: () => invokeCommand<PendingTransfer[]>('get_pending_transfers_handler'),

    /** 恢复传输 */
    resumeTransfer: (tid: number) => invokeCommand<void>('resume_transfer_handler', { tid }),
  };

  // 群组相关 IPC 调用
  const groupApi = {
    /** 创建群组 */
    createGroup: (groupName: string, creatorUid: number, memberIds: number[]) =>
      invokeCommand<number>('create_group_handler', {
        group_name: groupName,
        creator_uid: creatorUid,
        member_ids: memberIds,
      }),

    /** 获取群组信息 */
    getGroupInfo: (gid: number) => invokeCommand<GroupInfo>('get_group_info_handler', { gid }),

    /** 获取群组成员 */
    getGroupMembers: (gid: number) =>
      invokeCommand<GroupMember[]>('get_group_members_handler', { gid }),

    /** 添加群成员 */
    addGroupMember: (gid: number, memberUid: number) =>
      invokeCommand<void>('add_group_member_handler', {
        gid,
        member_uid: memberUid,
      }),

    /** 移除群成员 */
    removeGroupMember: (gid: number, memberUid: number) =>
      invokeCommand<void>('remove_group_member_handler', {
        gid,
        member_uid: memberUid,
      }),

    /** 更新成员角色 */
    updateMemberRole: (gid: number, memberUid: number, role: number) =>
      invokeCommand<void>('update_member_role_handler', {
        gid,
        member_uid: memberUid,
        role,
      }),

    /** 获取用户的群组列表 */
    getUserGroups: (uid: number) => invokeCommand<GroupInfo[]>('get_user_groups_handler', { uid }),
  };

  return {
    // 基础方法
    invoke: invokeCommand,
    invokeWithLoading,
    invokeBatch,

    // API 模块
    user: userApi,
    chat: chatApi,
    contact: contactApi,
    file: fileApi,
    group: groupApi,
  };
}

// 导出类型
export type { InvokeOptions, InvokeResult };
