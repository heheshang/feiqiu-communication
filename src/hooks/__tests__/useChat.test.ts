import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useChat } from '../useChat';
import * as store from '../../store';
import * as chatService from '../../services/chatService';
import type { ChatSession, ChatMessage } from '../../types';
import { MessageType, SessionType } from '../../types';

// Mock the store
vi.mock('../../store', () => {
  const mockChatStore = vi.fn() as any;
  mockChatStore.getState = vi.fn();
  const mockUserStore = vi.fn() as any;
  mockUserStore.getState = vi.fn();
  return {
    useChatStore: mockChatStore,
    useUserStore: mockUserStore,
  };
});

describe('useChat', () => {
  const mockSessions: ChatSession[] = [
    {
      sid: 1,
      owner_uid: 1,
      session_type: SessionType.Single,
      target_id: 2,
      last_msg_id: null,
      unread_count: 2,
      update_time: '2024-01-30T10:00:00Z',
      session_name: 'Bob',
      last_message: 'Hello',
      last_message_time: '2024-01-30T10:00:00Z',
    },
    {
      sid: 2,
      owner_uid: 1,
      session_type: SessionType.Group,
      target_id: 10,
      last_msg_id: null,
      unread_count: 0,
      update_time: '2024-01-30T11:00:00Z',
      session_name: 'Team Chat',
      last_message: 'Meeting at 3pm',
      last_message_time: '2024-01-30T11:00:00Z',
    },
  ];

  const mockMessages: ChatMessage[] = [
    {
      mid: 1,
      session_type: SessionType.Single,
      target_id: 2,
      sender_uid: 2,
      msg_type: MessageType.Text,
      content: 'Hello',
      send_time: '2024-01-30T10:00:00Z',
      status: 1,
    },
    {
      mid: 2,
      session_type: SessionType.Single,
      target_id: 2,
      sender_uid: 1,
      msg_type: MessageType.Text,
      content: 'Hi there!',
      send_time: '2024-01-30T10:01:00Z',
      status: 1,
    },
  ];

  const mockCurrentUser = {
    uid: 1,
    nickname: 'Alice',
    avatar: undefined,
    hostName: 'TEST-PC',
    loginTime: 1706616000,
  };

  const mockCurrentSession = mockSessions[0];

  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
    vi.spyOn(chatService.chatService, 'getSessionList').mockResolvedValue([]);
    vi.spyOn(chatService.chatService, 'getHistory').mockResolvedValue([]);
    vi.spyOn(chatService.chatService, 'sendMessage').mockResolvedValue(1);
    vi.spyOn(chatService.chatService, 'markMessagesRead').mockResolvedValue(undefined);
    vi.spyOn(chatService.chatService, 'retrySendMessage').mockResolvedValue(undefined);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  describe('getSessionList', () => {
    it('should fetch and return session list successfully', async () => {
      const fetchSessions = vi.fn().mockImplementation(async (fn: any) => {
        await fn();
        return mockSessions;
      });

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions,
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.getSessionList as any).mockResolvedValueOnce(mockSessions);

      const { result } = renderHook(() => useChat());
      let sessions;
      await act(async () => {
        sessions = await result.current.getSessionList();
      });

      expect(sessions).toEqual(mockSessions);
      expect(chatService.chatService.getSessionList).toHaveBeenCalledWith(1);
    });

    it('should return empty array when no current user', async () => {
      (store.useChatStore as any).mockReturnValue({
        sessions: [],
        fetchSessions: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: null,
      });

      const { result } = renderHook(() => useChat());
      const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      let sessions;
      await act(async () => {
        sessions = await result.current.getSessionList();
      });

      expect(sessions).toEqual([]);
      expect(consoleWarnSpy).toHaveBeenCalledWith('No current user found');
      consoleWarnSpy.mockRestore();
    });
  });

  describe('loadSessionMessages', () => {
    it('should load messages for session successfully', async () => {
      const fetchMessages = vi
        .fn()
        .mockImplementation(async (_sessionType: number, _targetId: number, fn: any) => {
          await fn();
          return mockMessages;
        });

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        fetchMessages,
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.getHistory as any).mockResolvedValueOnce(mockMessages);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.loadSessionMessages(SessionType.Single, 2, 1);
      });

      expect(fetchMessages).toHaveBeenCalledWith(SessionType.Single, 2, expect.any(Function));
      expect(chatService.chatService.getHistory).toHaveBeenCalledWith(SessionType.Single, 2, 1);
    });

    it('should handle pagination correctly', async () => {
      const fetchMessages = vi
        .fn()
        .mockImplementation(async (_sessionType: number, _targetId: number, fn: any) => {
          await fn();
          return mockMessages;
        });

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        fetchMessages,
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.getHistory as any).mockResolvedValueOnce(mockMessages);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.loadSessionMessages(SessionType.Single, 2, 2);
      });

      expect(chatService.chatService.getHistory).toHaveBeenCalledWith(SessionType.Single, 2, 2);
    });
  });

  describe('initializeSession', () => {
    it('should find existing session and load messages', async () => {
      const mockSession = mockSessions[0];
      const setCurrentSession = vi.fn();
      const getSessionByTarget = vi.fn().mockReturnValue(mockSession);
      const fetchMessages = vi
        .fn()
        .mockImplementation(async (_sessionType: number, _targetId: number, fn: any) => {
          await fn();
        });

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        setCurrentSession,
        getSessionByTarget,
        fetchMessages,
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.getHistory as any).mockResolvedValueOnce([]);

      const { result } = renderHook(() => useChat());
      const consoleLogSpy = vi.spyOn(console, 'log').mockImplementation(() => {});

      await act(async () => {
        await result.current.initializeSession(SessionType.Single, 2);
      });

      expect(getSessionByTarget).toHaveBeenCalledWith(SessionType.Single, 2);
      expect(setCurrentSession).toHaveBeenCalledWith(mockSession);
      expect(fetchMessages).toHaveBeenCalled();
      consoleLogSpy.mockRestore();
    });

    it('should handle when session not found', async () => {
      const setCurrentSession = vi.fn();
      const getSessionByTarget = vi.fn().mockReturnValue(null);
      const fetchMessages = vi.fn();

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        setCurrentSession,
        getSessionByTarget,
        fetchMessages,
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      const { result } = renderHook(() => useChat());
      const consoleLogSpy = vi.spyOn(console, 'log').mockImplementation(() => {});

      await act(async () => {
        await result.current.initializeSession(SessionType.Single, 999);
      });

      expect(getSessionByTarget).toHaveBeenCalledWith(SessionType.Single, 999);
      expect(setCurrentSession).not.toHaveBeenCalled();
      expect(consoleLogSpy).toHaveBeenCalledWith(
        'Session not found, will be created on first message'
      );
      consoleLogSpy.mockRestore();
    });
  });

  describe('sendMessage', () => {
    it('should send message and return message ID successfully', async () => {
      const addMessage = vi.fn();
      const mockMessageId = 12345;

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage,
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.sendMessage as any).mockResolvedValueOnce(mockMessageId);

      const { result } = renderHook(() => useChat());

      let messageId;
      await act(async () => {
        messageId = await result.current.sendMessage(SessionType.Single, 2, 'Test message');
      });

      expect(messageId).toBe(mockMessageId);
      expect(chatService.chatService.sendMessage).toHaveBeenCalledWith(
        SessionType.Single,
        2,
        'Test message',
        mockCurrentUser.uid
      );
      expect(addMessage).toHaveBeenCalledWith(
        NaN,
        expect.objectContaining({
          mid: mockMessageId,
          content: 'Test message',
          status: 0, // Sending status
        })
      );
    });

    it('should add optimistic update (temp message with status 0) before API call', async () => {
      const addMessage = vi.fn();
      const mockMessageId = 12346;

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage,
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.sendMessage as any).mockResolvedValueOnce(mockMessageId);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.sendMessage(SessionType.Single, 2, 'Optimistic message');
      });

      expect(addMessage).toHaveBeenCalledWith(
        NaN,
        expect.objectContaining({
          mid: mockMessageId,
          session_type: SessionType.Single,
          target_id: 2,
          sender_uid: mockCurrentUser.uid,
          msg_type: MessageType.Text,
          content: 'Optimistic message',
          status: 0, // Sending status
        })
      );
    });

    it('should throw error when no current user', async () => {
      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: null,
      });

      const { result } = renderHook(() => useChat());

      await expect(
        result.current.sendMessage(SessionType.Single, 2, 'Test message')
      ).rejects.toThrow('No current user found');
    });
  });

  describe('sendFileMessage', () => {
    it('should send file message successfully', async () => {
      const mockMessageId = 12345;

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.sendMessage as any).mockResolvedValueOnce(mockMessageId);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.sendFileMessage(
          SessionType.Single,
          2,
          '/path/to/file.pdf',
          'file.pdf'
        );
      });

      expect(chatService.chatService.sendMessage).toHaveBeenCalledWith(
        SessionType.Single,
        2,
        '[文件] file.pdf',
        mockCurrentUser.uid
      );
    });

    it('should throw error when no current user', async () => {
      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: null,
      });

      const { result } = renderHook(() => useChat());

      await expect(
        result.current.sendFileMessage(SessionType.Single, 2, '/path/to/file.pdf', 'file.pdf')
      ).rejects.toThrow('No current user found');
    });
  });

  describe('selectSession', () => {
    it('should select session and load messages', async () => {
      const mockSession = mockSessions[0];
      const setCurrentSession = vi.fn();
      const clearUnreadCount = vi.fn().mockResolvedValue(undefined);
      const fetchMessages = vi
        .fn()
        .mockImplementation(async (_sessionType: number, _targetId: number, fn: any) => {
          await fn();
        });

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages,
        setCurrentSession,
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount,
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.getHistory as any).mockResolvedValueOnce([]);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.selectSession(mockSession);
      });

      expect(setCurrentSession).toHaveBeenCalledWith(mockSession);
      expect(fetchMessages).toHaveBeenCalledWith(
        mockSession.session_type,
        mockSession.target_id,
        expect.any(Function)
      );
    });

    it('should clear unread count when > 0', async () => {
      const mockSession = { ...mockSessions[0], unread_count: 5 };
      const setCurrentSession = vi.fn();
      const clearUnreadCount = vi.fn().mockResolvedValue(undefined);
      const fetchMessages = vi
        .fn()
        .mockImplementation(async (_sessionType: number, _targetId: number, fn: any) => {
          await fn();
        });

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages,
        setCurrentSession,
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount,
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.getHistory as any).mockResolvedValueOnce([]);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.selectSession(mockSession);
      });

      expect(clearUnreadCount).toHaveBeenCalledWith(mockSession.sid);
    });
  });

  describe('markCurrentSessionAsRead', () => {
    it('should mark messages as read successfully', async () => {
      const clearUnreadCount = vi.fn().mockResolvedValue(undefined);

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: mockCurrentSession,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount,
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.markMessagesRead as any).mockResolvedValueOnce(undefined);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.markCurrentSessionAsRead();
      });

      expect(chatService.chatService.markMessagesRead).toHaveBeenCalledWith(
        mockCurrentSession.session_type,
        mockCurrentSession.target_id,
        mockCurrentUser.uid
      );
      expect(clearUnreadCount).toHaveBeenCalledWith(mockCurrentSession.sid);
    });

    it('should return early if no session or user', async () => {
      const clearUnreadCount = vi.fn();

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount,
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: null,
      });

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.markCurrentSessionAsRead();
      });

      expect(chatService.chatService.markMessagesRead).not.toHaveBeenCalled();
      expect(clearUnreadCount).not.toHaveBeenCalled();
    });
  });

  describe('retryMessage', () => {
    it('should retry failed message successfully', async () => {
      const mockMessage: ChatMessage = {
        mid: 1,
        session_type: SessionType.Single,
        target_id: 2,
        sender_uid: 1,
        msg_type: MessageType.Text,
        content: 'Failed message',
        send_time: '2024-01-30T10:00:00Z',
        status: 0, // Failed status
      };

      const retrySendMessage = vi
        .fn()
        .mockImplementation(async (_message: ChatMessage, fn: any) => {
          await fn();
        });

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage,
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (chatService.chatService.retrySendMessage as any).mockResolvedValueOnce(undefined);

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.retryMessage(mockMessage);
      });

      expect(retrySendMessage).toHaveBeenCalledWith(mockMessage, expect.any(Function));
      expect(chatService.chatService.retrySendMessage).toHaveBeenCalledWith(
        mockMessage.mid,
        mockMessage.session_type,
        mockMessage.target_id,
        mockCurrentUser.uid
      );
    });

    it('should throw error when no current user', async () => {
      const mockMessage: ChatMessage = {
        mid: 1,
        session_type: SessionType.Single,
        target_id: 2,
        sender_uid: 1,
        msg_type: MessageType.Text,
        content: 'Failed message',
        send_time: '2024-01-30T10:00:00Z',
        status: 0,
      };

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus: vi.fn(),
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: null,
      });

      const { result } = renderHook(() => useChat());

      await expect(result.current.retryMessage(mockMessage)).rejects.toThrow(
        'No current user found'
      );
    });
  });

  describe('setMessageStatus', () => {
    it('should update message status in store', async () => {
      const updateMessageStatus = vi.fn();

      (store.useChatStore as any).mockReturnValue({
        sessions: mockSessions,
        currentSession: null,
        messages: new Map(),
        isLoadingSessions: false,
        isLoadingMessages: false,
        fetchSessions: vi.fn(),
        fetchMessages: vi.fn(),
        setCurrentSession: vi.fn(),
        addMessage: vi.fn(),
        updateMessageStatus,
        clearUnreadCount: vi.fn(),
        retrySendMessage: vi.fn(),
        getMessagesBySession: vi.fn(),
        getSessionByTarget: vi.fn(),
      });

      (store.useUserStore as any).mockReturnValue({
        currentUser: mockCurrentUser,
      });

      const { result } = renderHook(() => useChat());

      await act(async () => {
        await result.current.setMessageStatus(1, 1);
      });

      expect(updateMessageStatus).toHaveBeenCalledWith(1, 1);
    });
  });
});
