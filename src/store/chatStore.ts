import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import type { ChatMessage, ChatSession } from '../types';
import { MessageStatus, SessionType } from '../types/chat';

interface ChatState {
  // 状态
  sessions: ChatSession[];
  currentSession: ChatSession | null;
  messages: Record<number, ChatMessage[]>; // 按 sessionId 分组存储消息
  isLoadingSessions: boolean;
  isLoadingMessages: Set<number>; // 正在加载消息的会话 ID 集合
  sendingMessages: Set<number>; // 正在发送的消息 ID 集合

  // 操作方法 - 会话
  fetchSessions: (fetchFn: () => Promise<ChatSession[]>) => Promise<void>;
  setSessions: (sessions: ChatSession[]) => void;
  setCurrentSession: (session: ChatSession | null) => void;
  updateSession: (sessionId: number, updates: Partial<ChatSession>) => void;
  clearUnreadCount: (sessionId: number) => void;

  // 操作方法 - 消息
  fetchMessages: (
    sessionType: SessionType,
    targetId: number,
    fetchFn: () => Promise<ChatMessage[]>
  ) => Promise<void>;
  addMessage: (sessionId: number, message: ChatMessage) => void;
  addMessages: (sessionId: number, messages: ChatMessage[]) => void;
  updateMessageStatus: (msgId: number, status: MessageStatus) => void;
  markMessagesAsRead: () => void;
  retrySendMessage: (message: ChatMessage, retrySendFn: () => Promise<void>) => Promise<void>;

  // 辅助方法
  getMessagesBySession: (sessionId: number) => ChatMessage[];
  getSessionByTarget: (sessionType: SessionType, targetId: number) => ChatSession | undefined;
  clearAllMessages: () => void;
}

export const useChatStore = create<ChatState>()(
  devtools((set, get) => ({
    sessions: [],
    currentSession: null,
    messages: {},
    isLoadingSessions: false,
    isLoadingMessages: new Set(),
    sendingMessages: new Set(),

    // 获取会话列表
    fetchSessions: async (fetchFn: () => Promise<ChatSession[]>) => {
      set({ isLoadingSessions: true });

      try {
        const sessions = await fetchFn();
        set({ sessions, isLoadingSessions: false });
      } catch (error) {
        console.error('Failed to fetch sessions:', error);
        set({ isLoadingSessions: false });
        throw error;
      }
    },

    // 设置会话列表
    setSessions: (sessions) => set({ sessions }),

    // 设置当前会话
    setCurrentSession: (session) => set({ currentSession: session }),

    // 更新会话
    updateSession: (sessionId, updates) =>
      set((state) => ({
        sessions: state.sessions.map((s) => (s.sid === sessionId ? { ...s, ...updates } : s)),
        currentSession:
          state.currentSession?.sid === sessionId
            ? { ...state.currentSession, ...updates }
            : state.currentSession,
      })),

    // 清除未读消息数
    clearUnreadCount: (sessionId) => {
      get().updateSession(sessionId, { unread_count: 0 });
    },

    // 获取消息历史
    fetchMessages: async (
      sessionType: SessionType,
      targetId: number,
      fetchFn: () => Promise<ChatMessage[]>
    ) => {
      const sessionId = `${sessionType}-${targetId}`;

      set((state) => ({
        isLoadingMessages: new Set(state.isLoadingMessages).add(Number(sessionId)),
      }));

      try {
        const messages = await fetchFn();

        // 更新消息列表
        set((state) => {
          const existingMessages = state.messages[Number(sessionId)] || [];
          // 避免重复添加消息
          const existingIds = new Set(existingMessages.map((m) => m.mid));
          const newMessages = messages.filter((m) => !existingIds.has(m.mid));

          return {
            messages: {
              ...state.messages,
              [Number(sessionId)]: [...existingMessages, ...newMessages],
            },
            isLoadingMessages: new Set(
              Array.from(state.isLoadingMessages).filter((id) => id !== Number(sessionId))
            ),
          };
        });
      } catch (error) {
        console.error('Failed to fetch messages:', error);
        set((state) => ({
          isLoadingMessages: new Set(
            Array.from(state.isLoadingMessages).filter((id) => id !== Number(sessionId))
          ),
        }));
        throw error;
      }
    },

    // 添加单条消息
    addMessage: (sessionId, message) =>
      set((state) => {
        const existingMessages = state.messages[sessionId] || [];
        // 避免重复添加
        if (existingMessages.some((m) => m.mid === message.mid)) {
          return state;
        }

        return {
          messages: {
            ...state.messages,
            [sessionId]: [...existingMessages, message],
          },
        };
      }),

    // 添加多条消息
    addMessages: (sessionId, messages) =>
      set((state) => {
        const existingMessages = state.messages[sessionId] || [];
        const existingIds = new Set(existingMessages.map((m) => m.mid));
        const newMessages = messages.filter((m) => !existingIds.has(m.mid));

        return {
          messages: {
            ...state.messages,
            [sessionId]: [...existingMessages, ...newMessages],
          },
        };
      }),

    // 更新消息状态
    updateMessageStatus: (msgId, status) =>
      set((state) => {
        const messages = { ...state.messages };

        for (const sessionId in messages) {
          messages[sessionId] = messages[sessionId].map((msg) =>
            msg.mid === msgId ? { ...msg, status } : msg
          );
        }

        return { messages };
      }),

    // 标记消息已读
    markMessagesAsRead: () => {
      // Store only manages state, IPC calls are handled by hooks
    },

    // 重试发送消息
    retrySendMessage: async (message: ChatMessage, retrySendFn: () => Promise<void>) => {
      set((state) => ({
        sendingMessages: new Set(state.sendingMessages).add(message.mid),
      }));

      try {
        await retrySendFn();
        get().updateMessageStatus(message.mid, MessageStatus.Sent);
      } catch (error) {
        console.error('Failed to retry send message:', error);
        get().updateMessageStatus(message.mid, MessageStatus.Failed);
        throw error;
      } finally {
        set((state) => ({
          sendingMessages: new Set(
            Array.from(state.sendingMessages).filter((id) => id !== message.mid)
          ),
        }));
      }
    },

    // 获取指定会话的消息
    getMessagesBySession: (sessionId) => {
      return get().messages[sessionId] || [];
    },

    // 根据会话类型和目标 ID 获取会话
    getSessionByTarget: (sessionType, targetId) => {
      const { sessions } = get();
      return sessions.find((s) => s.session_type === sessionType && s.target_id === targetId);
    },

    // 清空所有消息
    clearAllMessages: () => set({ messages: {} }),
  }))
);

// 选择器辅助函数
export const selectSessions = (state: ChatState) => state.sessions;
export const selectCurrentSession = (state: ChatState) => state.currentSession;
export const selectMessagesBySession = (sessionId: number) => (state: ChatState) =>
  state.messages[sessionId] || [];
export const selectIsLoadingSessions = (state: ChatState) => state.isLoadingSessions;
export const selectIsLoadingMessages = (sessionId: number) => (state: ChatState) =>
  state.isLoadingMessages.has(sessionId);
