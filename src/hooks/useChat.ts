// 自定义钩子 - 聊天功能

import { useState, useCallback, useRef } from 'react';
import { useIPC } from './useIPC';
import { chatAPI } from '../ipc/chat';
import type { ChatMessage, ChatSession, MessageStatus } from '../types';

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
  const getHistory = useCallback(
    async (sessionType: number, targetId: number, page: number, pageSize: number = 50) => {
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
    },
    [invoke]
  );

  /** 加载更多历史消息 */
  const loadMoreMessages = useCallback(
    async (sessionType: number, targetId: number) => {
      if (!paginationRef.current.hasMore || paginationRef.current.isLoading) {
        return;
      }

      const nextPage = paginationRef.current.currentPage + 1;
      const newMessages = await getHistory(sessionType, targetId, nextPage);

      if (newMessages.length > 0) {
        // 将新消息添加到前面（更旧的消息）
        setMessages((prev) => [...newMessages, ...prev]);
      }
    },
    [getHistory]
  );

  /** 初始化加载消息 */
  const loadInitialMessages = useCallback(
    async (sessionType: number, targetId: number) => {
      resetPagination();
      const initialMessages = await getHistory(sessionType, targetId, 1);
      setMessages(initialMessages);
      return initialMessages;
    },
    [getHistory, resetPagination]
  );

  /** 发送消息 */
  const sendMessage = useCallback(
    async (sessionType: number, targetId: number, content: string) => {
      return await invoke<number>('send_text_message_handler', {
        sessionType,
        targetId,
        content,
        ownerUid: 0, // TODO: 从用户状态获取
      });
    },
    [invoke]
  );

  /** 获取会话列表 */
  const getSessionList = useCallback(async () => {
    const result = await invoke<ChatSession[]>('get_session_list_handler', { ownerUid: 0 });
    setSessions(result);
    return result;
  }, [invoke]);

  /** 标记单条消息已读并发送已读回执 */
  const markMessageRead = useCallback(async (message: ChatMessage) => {
    if (!message.msg_no || !message.sender_ip) {
      console.warn('Message missing msg_no or sender_ip, cannot send read receipt');
      return;
    }

    try {
      await chatAPI.markMessageReadAndSendReceipt(message.mid, message.msg_no, message.sender_ip);

      // Update local message status
      setMessages((prev) =>
        prev.map((msg) => (msg.mid === message.mid ? { ...msg, status: 2 as MessageStatus } : msg))
      );
    } catch (error) {
      console.error('Failed to mark message as read:', error);
    }
  }, []);

  /** 更新消息状态 */
  const updateMessageStatus = useCallback((mid: number, status: MessageStatus) => {
    setMessages((prev) => prev.map((msg) => (msg.mid === mid ? { ...msg, status } : msg)));
  }, []);

  /** 重试发送失败的消息 */
  const retrySendMessage = useCallback(
    async (message: ChatMessage) => {
      try {
        await chatAPI.retrySendMessage(
          message.mid,
          message.session_type,
          message.target_id,
          0 // TODO: 从用户状态获取
        );

        // 更新本地状态为发送中
        updateMessageStatus(message.mid, 0 as MessageStatus);
      } catch (error) {
        console.error('Failed to retry message:', error);
      }
    },
    [updateMessageStatus]
  );

  return {
    messages,
    sessions,
    getHistory,
    loadInitialMessages,
    loadMoreMessages,
    resetPagination,
    sendMessage,
    getSessionList,
    markMessageRead,
    updateMessageStatus,
    retrySendMessage,
    pagination: paginationRef.current,
  };
}
