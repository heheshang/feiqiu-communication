import { describe, it, expect, beforeEach, vi } from 'vitest';
import { chatService } from '../chatService';
import * as chatAPI from '../../ipc/chat';

// Mock the chatAPI module
vi.mock('../../ipc/chat', () => ({
  chatAPI: {
    getHistory: vi.fn(),
    sendMessage: vi.fn(),
    getSessionList: vi.fn(),
    markMessagesRead: vi.fn(),
    markMessageReadAndSendReceipt: vi.fn(),
    retrySendMessage: vi.fn(),
  },
}));

describe('chatService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getHistory', () => {
    it('should call chatAPI.getHistory with correct parameters', async () => {
      const mockMessages = [
        {
          mid: 1,
          session_type: 0,
          target_id: 123,
          sender_uid: 456,
          msg_type: 0,
          content: 'Hello',
          send_time: '2024-01-30T10:00:00Z',
          status: 1,
        },
      ];

      (chatAPI.chatAPI.getHistory as any).mockResolvedValueOnce(mockMessages);

      const result = await chatService.getHistory(0, 123, 1);

      expect(chatAPI.chatAPI.getHistory).toHaveBeenCalledWith(0, 123, 1);
      expect(result).toEqual(mockMessages);
    });

    it('should handle error when getHistory fails', async () => {
      const error = new Error('Database error');
      (chatAPI.chatAPI.getHistory as any).mockRejectedValueOnce(error);

      await expect(chatService.getHistory(0, 123, 1)).rejects.toThrow('Database error');
      expect(chatAPI.chatAPI.getHistory).toHaveBeenCalledWith(0, 123, 1);
    });

    it('should return empty array when no messages found', async () => {
      (chatAPI.chatAPI.getHistory as any).mockResolvedValueOnce([]);

      const result = await chatService.getHistory(0, 123, 1);

      expect(result).toEqual([]);
      expect(chatAPI.chatAPI.getHistory).toHaveBeenCalledWith(0, 123, 1);
    });

    it('should handle pagination correctly', async () => {
      const mockMessages = Array.from({ length: 50 }, (_, i) => ({
        mid: i + 1,
        session_type: 0,
        target_id: 123,
        sender_uid: 456,
        msg_type: 0,
        content: `Message ${i + 1}`,
        send_time: '2024-01-30T10:00:00Z',
        status: 1,
      }));

      (chatAPI.chatAPI.getHistory as any).mockResolvedValueOnce(mockMessages);

      const result = await chatService.getHistory(0, 123, 2);

      expect(chatAPI.chatAPI.getHistory).toHaveBeenCalledWith(0, 123, 2);
      expect(result).toHaveLength(50);
    });
  });

  describe('sendMessage', () => {
    it('should send message and return message ID', async () => {
      const messageId = 789;
      (chatAPI.chatAPI.sendMessage as any).mockResolvedValueOnce(messageId);

      const result = await chatService.sendMessage(0, 123, 'Hello World', 456);

      expect(chatAPI.chatAPI.sendMessage).toHaveBeenCalledWith(0, 123, 'Hello World', 456);
      expect(result).toBe(messageId);
    });

    it('should handle error when sending message fails', async () => {
      const error = new Error('Network error');
      (chatAPI.chatAPI.sendMessage as any).mockRejectedValueOnce(error);

      await expect(chatService.sendMessage(0, 123, 'Hello', 456)).rejects.toThrow('Network error');
      expect(chatAPI.chatAPI.sendMessage).toHaveBeenCalledWith(0, 123, 'Hello', 456);
    });

    it('should handle empty message content', async () => {
      const messageId = 790;
      (chatAPI.chatAPI.sendMessage as any).mockResolvedValueOnce(messageId);

      const result = await chatService.sendMessage(0, 123, '', 456);

      expect(chatAPI.chatAPI.sendMessage).toHaveBeenCalledWith(0, 123, '', 456);
      expect(result).toBe(messageId);
    });

    it('should handle long message content', async () => {
      const longContent = 'a'.repeat(10000);
      const messageId = 791;
      (chatAPI.chatAPI.sendMessage as any).mockResolvedValueOnce(messageId);

      const result = await chatService.sendMessage(0, 123, longContent, 456);

      expect(chatAPI.chatAPI.sendMessage).toHaveBeenCalledWith(0, 123, longContent, 456);
      expect(result).toBe(messageId);
    });

    it('should support group chat (sessionType = 1)', async () => {
      const messageId = 792;
      (chatAPI.chatAPI.sendMessage as any).mockResolvedValueOnce(messageId);

      const result = await chatService.sendMessage(1, 999, 'Group message', 456);

      expect(chatAPI.chatAPI.sendMessage).toHaveBeenCalledWith(1, 999, 'Group message', 456);
      expect(result).toBe(messageId);
    });
  });

  describe('getSessionList', () => {
    it('should return session list for user', async () => {
      const mockSessions = [
        {
          sid: 1,
          owner_uid: 456,
          session_type: 0,
          target_id: 123,
          last_msg_id: 100,
          unread_count: 0,
          update_time: '2024-01-30T10:00:00Z',
          session_name: 'User 123',
          last_message: 'Last message',
          last_message_time: '2024-01-30T10:00:00Z',
        },
      ];

      (chatAPI.chatAPI.getSessionList as any).mockResolvedValueOnce(mockSessions);

      const result = await chatService.getSessionList(456);

      expect(chatAPI.chatAPI.getSessionList).toHaveBeenCalledWith(456);
      expect(result).toEqual(mockSessions);
    });

    it('should handle error when fetching session list fails', async () => {
      const error = new Error('Database error');
      (chatAPI.chatAPI.getSessionList as any).mockRejectedValueOnce(error);

      await expect(chatService.getSessionList(456)).rejects.toThrow('Database error');
      expect(chatAPI.chatAPI.getSessionList).toHaveBeenCalledWith(456);
    });

    it('should return empty array when user has no sessions', async () => {
      (chatAPI.chatAPI.getSessionList as any).mockResolvedValueOnce([]);

      const result = await chatService.getSessionList(456);

      expect(result).toEqual([]);
      expect(chatAPI.chatAPI.getSessionList).toHaveBeenCalledWith(456);
    });

    it('should handle multiple sessions', async () => {
      const mockSessions = [
        {
          sid: 1,
          owner_uid: 456,
          session_type: 0,
          target_id: 123,
          last_msg_id: 100,
          unread_count: 5,
          update_time: '2024-01-30T10:00:00Z',
        },
        {
          sid: 2,
          owner_uid: 456,
          session_type: 1,
          target_id: 999,
          last_msg_id: 200,
          unread_count: 0,
          update_time: '2024-01-30T09:00:00Z',
        },
      ];

      (chatAPI.chatAPI.getSessionList as any).mockResolvedValueOnce(mockSessions);

      const result = await chatService.getSessionList(456);

      expect(result).toHaveLength(2);
      expect(result[0].unread_count).toBe(5);
      expect(result[1].session_type).toBe(1);
    });
  });

  describe('markMessagesRead', () => {
    it('should mark messages as read', async () => {
      (chatAPI.chatAPI.markMessagesRead as any).mockResolvedValueOnce(undefined);

      await chatService.markMessagesRead(0, 123, 456);

      expect(chatAPI.chatAPI.markMessagesRead).toHaveBeenCalledWith(0, 123, 456);
    });

    it('should handle error when marking messages as read fails', async () => {
      const error = new Error('Database error');
      (chatAPI.chatAPI.markMessagesRead as any).mockRejectedValueOnce(error);

      await expect(chatService.markMessagesRead(0, 123, 456)).rejects.toThrow('Database error');
      expect(chatAPI.chatAPI.markMessagesRead).toHaveBeenCalledWith(0, 123, 456);
    });

    it('should handle group chat session type', async () => {
      (chatAPI.chatAPI.markMessagesRead as any).mockResolvedValueOnce(undefined);

      await chatService.markMessagesRead(1, 999, 456);

      expect(chatAPI.chatAPI.markMessagesRead).toHaveBeenCalledWith(1, 999, 456);
    });

    it('should be idempotent (can be called multiple times)', async () => {
      (chatAPI.chatAPI.markMessagesRead as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.markMessagesRead as any).mockResolvedValueOnce(undefined);

      await chatService.markMessagesRead(0, 123, 456);
      await chatService.markMessagesRead(0, 123, 456);

      expect(chatAPI.chatAPI.markMessagesRead).toHaveBeenCalledTimes(2);
    });
  });

  describe('markMessageReadAndSendReceipt', () => {
    it('should mark message as read and send receipt', async () => {
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);

      await chatService.markMessageReadAndSendReceipt(789, '12345', '192.168.1.100');

      expect(chatAPI.chatAPI.markMessageReadAndSendReceipt).toHaveBeenCalledWith(
        789,
        '12345',
        '192.168.1.100'
      );
    });

    it('should handle error when sending receipt fails', async () => {
      const error = new Error('Network error');
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockRejectedValueOnce(error);

      await expect(
        chatService.markMessageReadAndSendReceipt(789, '12345', '192.168.1.100')
      ).rejects.toThrow('Network error');
    });

    it('should handle various IP addresses', async () => {
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);

      const ips = ['192.168.1.1', '10.0.0.1', '172.16.0.1', '127.0.0.1'];

      for (const ip of ips) {
        await chatService.markMessageReadAndSendReceipt(789, '12345', ip);
      }

      expect(chatAPI.chatAPI.markMessageReadAndSendReceipt).toHaveBeenCalledTimes(4);
    });

    it('should handle various message numbers', async () => {
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.markMessageReadAndSendReceipt as any).mockResolvedValueOnce(undefined);

      const msgNos = ['1', '12345', '999999', 'abc123'];

      for (const msgNo of msgNos) {
        await chatService.markMessageReadAndSendReceipt(789, msgNo, '192.168.1.100');
      }

      expect(chatAPI.chatAPI.markMessageReadAndSendReceipt).toHaveBeenCalledTimes(4);
    });
  });

  describe('retrySendMessage', () => {
    it('should retry sending a failed message', async () => {
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);

      await chatService.retrySendMessage(789, 0, 123, 456);

      expect(chatAPI.chatAPI.retrySendMessage).toHaveBeenCalledWith(789, 0, 123, 456);
    });

    it('should handle error when retry fails', async () => {
      const error = new Error('Network error');
      (chatAPI.chatAPI.retrySendMessage as any).mockRejectedValueOnce(error);

      await expect(chatService.retrySendMessage(789, 0, 123, 456)).rejects.toThrow('Network error');
      expect(chatAPI.chatAPI.retrySendMessage).toHaveBeenCalledWith(789, 0, 123, 456);
    });

    it('should handle group chat retry', async () => {
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);

      await chatService.retrySendMessage(789, 1, 999, 456);

      expect(chatAPI.chatAPI.retrySendMessage).toHaveBeenCalledWith(789, 1, 999, 456);
    });

    it('should handle multiple retries', async () => {
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);

      await chatService.retrySendMessage(789, 0, 123, 456);
      await chatService.retrySendMessage(790, 0, 123, 456);
      await chatService.retrySendMessage(791, 0, 123, 456);

      expect(chatAPI.chatAPI.retrySendMessage).toHaveBeenCalledTimes(3);
    });

    it('should handle various message IDs', async () => {
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);
      (chatAPI.chatAPI.retrySendMessage as any).mockResolvedValueOnce(undefined);

      const messageIds = [1, 100, 999999, 1000000];

      for (const mid of messageIds) {
        await chatService.retrySendMessage(mid, 0, 123, 456);
      }

      expect(chatAPI.chatAPI.retrySendMessage).toHaveBeenCalledTimes(4);
    });
  });
});
