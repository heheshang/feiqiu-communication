// 自定义钩子 - 聊天功能
// TODO: Phase 4 时完善聊天功能钩子

import { useState, useCallback } from 'react';
import { useIPC } from './useIPC';
import type { ChatMessage, ChatSession } from '../types';

export function useChat() {
  const { invoke } = useIPC();
  const [messages] = useState<ChatMessage[]>([]);
  const [sessions, setSessions] = useState<ChatSession[]>([]);

  /** 获取历史消息 */
  const getHistory = useCallback(async (sessionType: number, targetId: number, page: number) => {
    return await invoke<ChatMessage[]>('get_chat_history_handler', {
      sessionType,
      targetId,
      page,
      pageSize: 50,
    });
  }, [invoke]);

  /** 发送消息 */
  const sendMessage = useCallback(async (sessionType: number, targetId: number, content: string) => {
    return await invoke<number>('send_text_message_handler', {
      sessionType,
      targetId,
      content,
      ownerUid: 0, // TODO: 从用户状态获取
    });
  }, [invoke]);

  /** 获取会话列表 */
  const getSessionList = useCallback(async () => {
    const result = await invoke<ChatSession[]>('get_session_list_handler', { ownerUid: 0 });
    setSessions(result);
    return result;
  }, [invoke]);

  return {
    messages,
    sessions,
    getHistory,
    sendMessage,
    getSessionList,
  };
}
