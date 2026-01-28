// 自定义钩子 - 聊天功能

import { useState, useCallback, useRef } from 'react';
import { useIPC } from './useIPC';
import type { ChatMessage, ChatSession } from '../types';

interface PaginationState {
  currentPage: number;
  hasMore: boolean;
  isLoading: boolean;
}

export function useChat() {
  const { invoke } = useIPC();
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [sessions, setSessions] = useState<ChatSession[]>([]);
  const paginationRef = useRef<PaginationState>({
    currentPage: 1,
    hasMore: true,
    isLoading: false,
  });

  /** 重置分页状态 */
  const resetPagination = useCallback(() => {
    paginationRef.current = {
      currentPage: 1,
      hasMore: true,
      isLoading: false,
    };
    setMessages([]);
  }, []);

  /** 获取历史消息（分页） */
  const getHistory = useCallback(async (sessionType: number, targetId: number, page: number, pageSize: number = 50) => {
    if (paginationRef.current.isLoading) {
      return [];
    }

    paginationRef.current.isLoading = true;

    try {
      const result = await invoke<ChatMessage[]>('get_chat_history_handler', {
        sessionType,
        targetId,
        page,
        pageSize,
      });

      // 如果返回的消息少于请求的数量，说明没有更多了
      if (result.length < pageSize) {
        paginationRef.current.hasMore = false;
      } else {
        paginationRef.current.hasMore = true;
      }

      paginationRef.current.currentPage = page;
      paginationRef.current.isLoading = false;

      return result;
    } catch (error) {
      paginationRef.current.isLoading = false;
      console.error('Failed to load messages:', error);
      return [];
    }
  }, [invoke]);

  /** 加载更多历史消息 */
  const loadMoreMessages = useCallback(async (sessionType: number, targetId: number) => {
    if (!paginationRef.current.hasMore || paginationRef.current.isLoading) {
      return;
    }

    const nextPage = paginationRef.current.currentPage + 1;
    const newMessages = await getHistory(sessionType, targetId, nextPage);

    if (newMessages.length > 0) {
      // 将新消息添加到前面（更旧的消息）
      setMessages(prev => [...newMessages, ...prev]);
    }
  }, [getHistory]);

  /** 初始化加载消息 */
  const loadInitialMessages = useCallback(async (sessionType: number, targetId: number) => {
    resetPagination();
    const initialMessages = await getHistory(sessionType, targetId, 1);
    setMessages(initialMessages);
    return initialMessages;
  }, [getHistory, resetPagination]);

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
    loadInitialMessages,
    loadMoreMessages,
    resetPagination,
    sendMessage,
    getSessionList,
    pagination: paginationRef.current,
  };
}
