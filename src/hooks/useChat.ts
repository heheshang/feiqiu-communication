// 自定义钩子 - 聊天功能

import { useCallback } from 'react';
import { useChatStore } from '../store';
import { useUserStore } from '../store';
import type { ChatMessage, ChatSession, SessionType } from '../types';
import { MessageType } from '../types';
import { chatService } from '../services';

export function useChat() {
  const {
    sessions,
    currentSession,
    messages,
    isLoadingSessions,
    isLoadingMessages,
    fetchSessions,
    fetchMessages,
    setCurrentSession,
    addMessage,
    updateMessageStatus,
    clearUnreadCount,
    retrySendMessage,
    getMessagesBySession,
    getSessionByTarget,
  } = useChatStore();

  const { currentUser } = useUserStore();

  /** 获取会话列表 */
  const getSessionList = useCallback(async () => {
    if (!currentUser) {
      console.warn('No current user found');
      return [];
    }

    await fetchSessions(async () => await chatService.getSessionList(currentUser.uid));
    return sessions;
  }, [currentUser, fetchSessions, sessions]);

  /** 加载会话消息 */
  const loadSessionMessages = useCallback(
    async (sessionType: SessionType, targetId: number, page = 1) => {
      await fetchMessages(
        sessionType,
        targetId,
        async () => await chatService.getHistory(sessionType, targetId, page)
      );
    },
    [fetchMessages]
  );

  /** 初始化会话 */
  const initializeSession = useCallback(
    async (sessionType: SessionType, targetId: number) => {
      // 查找或创建会话
      let session = getSessionByTarget(sessionType, targetId);

      if (!session) {
        // 需要通过发送一条消息来创建会话（后端自动创建）
        // 或者等待后端推送会话信息
        console.log('Session not found, will be created on first message');
      } else {
        setCurrentSession(session);
        // 加载消息
        await loadSessionMessages(sessionType, targetId, 1);
      }
    },
    [getSessionByTarget, setCurrentSession, loadSessionMessages]
  );

  /** 发送消息 */
  const sendMessage = useCallback(
    async (sessionType: SessionType, targetId: number, content: string) => {
      if (!currentUser) {
        throw new Error('No current user found');
      }

      const mid = await chatService.sendMessage(sessionType, targetId, content, currentUser.uid);

      // 乐观更新：添加一个临时的发送中消息到列表
      const tempMessage: ChatMessage = {
        mid,
        session_type: sessionType,
        target_id: targetId,
        sender_uid: currentUser.uid,
        msg_type: MessageType.Text,
        content,
        send_time: new Date().toISOString(),
        status: 0 as any, // 发送中
      };

      const sessionId = `${sessionType}-${targetId}`;
      addMessage(Number(sessionId), tempMessage);

      return mid;
    },
    [currentUser, addMessage]
  );

  /** 发送文件消息 */
  const sendFileMessage = useCallback(
    async (sessionType: SessionType, targetId: number, _filePath: string, fileName: string) => {
      if (!currentUser) {
        throw new Error('No current user found');
      }

      // 文件发送需要先上传或获取文件信息
      // 这里暂时简化处理
      const content = `[文件] ${fileName}`;
      return sendMessage(sessionType, targetId, content);
    },
    [currentUser, sendMessage]
  );

  /** 选择会话 */
  const selectSession = useCallback(
    async (session: ChatSession) => {
      setCurrentSession(session);

      // 清除未读消息数
      if (session.unread_count > 0) {
        await clearUnreadCount(session.sid);
      }

      // 加载消息
      await loadSessionMessages(session.session_type, session.target_id, 1);
    },
    [setCurrentSession, clearUnreadCount, loadSessionMessages]
  );

  /** 标记当前会话已读 */
  const markCurrentSessionAsRead = useCallback(async () => {
    if (!currentSession || !currentUser) {
      return;
    }

    await chatService.markMessagesRead(
      currentSession.session_type,
      currentSession.target_id,
      currentUser.uid
    );
    clearUnreadCount(currentSession.sid);
  }, [currentSession, currentUser]);

  /** 重试发送消息 */
  const retryMessage = useCallback(
    async (message: ChatMessage) => {
      if (!currentUser) {
        throw new Error('No current user found');
      }

      await retrySendMessage(
        message,
        async () =>
          await chatService.retrySendMessage(
            message.mid,
            message.session_type,
            message.target_id,
            currentUser.uid
          )
      );
    },
    [currentUser, retrySendMessage]
  );

  /** 更新消息状态 */
  const setMessageStatus = useCallback(
    async (msgId: number, status: number) => {
      updateMessageStatus(msgId, status as any);
    },
    [updateMessageStatus]
  );

  return {
    // 状态
    sessions,
    currentSession,
    messages,
    isLoadingSessions,
    isLoadingMessages,
    currentUser,

    // 操作
    getSessionList,
    loadSessionMessages,
    initializeSession,
    sendMessage,
    sendFileMessage,
    selectSession,
    markCurrentSessionAsRead,
    retryMessage,
    setMessageStatus,
    getMessagesBySession,
  };
}
