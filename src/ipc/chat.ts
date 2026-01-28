// IPC 封装 - 聊天相关
// TODO: Phase 4 时完善聊天 IPC 接口

import { invoke } from '@tauri-apps/api/core';
import type { ChatMessage, ChatSession } from '../types';

export const chatAPI = {
  /** 获取历史消息 */
  getHistory: async (sessionType: number, targetId: number, page: number) => {
    return await invoke<ChatMessage[]>('get_chat_history_handler', {
      sessionType,
      targetId,
      page,
      pageSize: 50,
    });
  },

  /** 发送消息 */
  sendMessage: async (sessionType: number, targetId: number, content: string, ownerUid: number) => {
    return await invoke<number>('send_text_message_handler', {
      sessionType,
      targetId,
      content,
      ownerUid,
    });
  },

  /** 获取会话列表 */
  getSessionList: async (ownerUid: number) => {
    return await invoke<ChatSession[]>('get_session_list_handler', { ownerUid });
  },

  /** 标记消息已读 */
  markMessagesRead: async (sessionType: number, targetId: number, ownerUid: number) => {
    return await invoke<void>('mark_messages_read_handler', {
      sessionType,
      targetId,
      ownerUid,
    });
  },

  /** 标记单条消息已读并发送已读回执 */
  markMessageReadAndSendReceipt: async (mid: number, msgNo: string, targetIp: string) => {
    return await invoke<void>('mark_message_read_and_send_receipt', {
      mid,
      msgNo,
      targetIp,
    });
  },

  /** 重试发送失败的消息 */
  retrySendMessage: async (
    mid: number,
    sessionType: number,
    targetId: number,
    ownerUid: number
  ) => {
    return await invoke<void>('retry_send_message', {
      mid,
      sessionType,
      targetId,
      ownerUid,
    });
  },
};
