// 状态管理 - 聊天状态
// TODO: Phase 4 时完善聊天状态管理

import { create } from 'zustand';
import type { ChatMessage, ChatSession } from '../types';

interface ChatState {
  sessions: ChatSession[];
  currentSession: ChatSession | null;
  messages: Record<number, ChatMessage[]>;
  setSessions: (sessions: ChatSession[]) => void;
  setCurrentSession: (session: ChatSession | null) => void;
  addMessage: (sessionId: number, message: ChatMessage) => void;
  updateMessageStatus: (msgId: number, status: number) => void;
}

export const useChatStore = create<ChatState>((set) => ({
  sessions: [],
  currentSession: null,
  messages: {},

  setSessions: (sessions) => set({ sessions }),

  setCurrentSession: (session) => set({ currentSession: session }),

  addMessage: (sessionId, message) =>
    set((state) => ({
      messages: {
        ...state.messages,
        [sessionId]: [...(state.messages[sessionId] || []), message],
      },
    })),

  updateMessageStatus: (msgId, status) =>
    set((state) => {
      const messages = { ...state.messages };
      for (const sessionId in messages) {
        messages[sessionId] = messages[sessionId].map((msg) =>
          msg.msg_id === msgId ? { ...msg, status } : msg
        );
      }
      return { messages };
    }),
}));
