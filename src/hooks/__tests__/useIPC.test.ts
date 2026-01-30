import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useIPC, IPCError } from '../useIPC';
import { invoke } from '@tauri-apps/api/core';

// Mock Tauri's invoke function
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('useIPC', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {});

  describe('invoke', () => {
    it('should execute command successfully', async () => {
      const mockData = { uid: 1, nickname: 'Alice' };
      (invoke as any).mockResolvedValueOnce(mockData);

      const { result } = renderHook(() => useIPC());
      let data;
      await act(async () => {
        data = await result.current.invoke('get_current_user_handler', undefined, { timeout: 100 });
      });

      expect(data).toEqual(mockData);
      expect(invoke).toHaveBeenCalledWith('get_current_user_handler', undefined);
    });

    it('should handle errors and throw IPCError', async () => {
      const mockError = new Error('RPC call failed');
      (invoke as any).mockRejectedValueOnce(mockError);

      const { result } = renderHook(() => useIPC());

      await expect(
        result.current.invoke('test_command', undefined, { timeout: 100 })
      ).rejects.toThrow(IPCError);
    });

    it('should retry on failure with exponential backoff', async () => {
      (invoke as any)
        .mockRejectedValueOnce(new Error('Attempt 1'))
        .mockRejectedValueOnce(new Error('Attempt 2'))
        .mockResolvedValueOnce('success');

      const { result } = renderHook(() => useIPC());

      let data;
      await act(async () => {
        data = await result.current.invoke('test_command', undefined, { retries: 2, timeout: 100 });
      });

      expect(data).toBe('success');
      expect(invoke).toHaveBeenCalledTimes(3);
    });
  });

  describe('invokeWithLoading', () => {
    it('should return data and isLoading false on success', async () => {
      (invoke as any).mockResolvedValueOnce('success');

      const { result } = renderHook(() => useIPC());
      let response: any;
      await act(async () => {
        response = await result.current.invokeWithLoading('test_command', undefined, {
          timeout: 100,
        });
      });

      expect(response).toEqual({
        data: 'success',
        isLoading: false,
      });
    });

    it('should return error and isLoading false on failure', async () => {
      (invoke as any).mockRejectedValueOnce(new Error('Test error'));

      const { result } = renderHook(() => useIPC());
      let response: any;
      await act(async () => {
        response = await result.current.invokeWithLoading('test_command', undefined, {
          timeout: 100,
        });
      });

      expect(response.error).toBeInstanceOf(Error);
      expect(response.isLoading).toBe(false);
    });
  });

  describe('invokeBatch', () => {
    it('should execute all calls in parallel', async () => {
      (invoke as any)
        .mockResolvedValueOnce('result1')
        .mockResolvedValueOnce('result2')
        .mockResolvedValueOnce('result3');

      const { result } = renderHook(() => useIPC());
      let results: any;
      await act(async () => {
        results = await result.current.invokeBatch([
          { command: 'cmd1' },
          { command: 'cmd2', args: { key: 'value' } },
          { command: 'cmd3' },
        ]);
      });

      expect(results).toEqual(['result1', 'result2', 'result3']);
      expect(invoke).toHaveBeenCalledTimes(3);
    });

    it('should handle mixed success and failure results', async () => {
      (invoke as any)
        .mockResolvedValueOnce('success')
        .mockRejectedValueOnce(new Error('failure'))
        .mockResolvedValueOnce('another success');

      const { result } = renderHook(() => useIPC());
      let results: any;
      await act(async () => {
        results = await result.current.invokeBatch([
          { command: 'cmd1' },
          { command: 'cmd2' },
          { command: 'cmd3' },
        ]);
      });

      expect(results[0]).toBe('success');
      expect(results[1]).toBeInstanceOf(Error);
      expect(results[2]).toBe('another success');
    });
  });

  describe('userApi', () => {
    it('should call getCurrentUser command', async () => {
      const mockUser = { uid: 1, nickname: 'Alice' };
      (invoke as any).mockResolvedValueOnce(mockUser);

      const { result } = renderHook(() => useIPC());
      let user;
      await act(async () => {
        user = await result.current.user.getCurrentUser();
      });

      expect(user).toEqual(mockUser);
      expect(invoke).toHaveBeenCalledWith('get_current_user_handler', undefined);
    });

    it('should call updateCurrentUser with correct params', async () => {
      const mockUser = { uid: 1, nickname: 'Bob' };
      (invoke as any).mockResolvedValueOnce(mockUser);

      const { result } = renderHook(() => useIPC());
      let user;
      await act(async () => {
        user = await result.current.user.updateCurrentUser(1, 'Bob', undefined);
      });

      expect(user).toEqual(mockUser);
      expect(invoke).toHaveBeenCalledWith('update_current_user_handler', {
        uid: 1,
        nickname: 'Bob',
        avatar: undefined,
      });
    });
  });

  describe('chatApi', () => {
    it('should call getHistory with correct params', async () => {
      const mockMessages = [{ mid: 1, content: 'Hello' }];
      (invoke as any).mockResolvedValueOnce(mockMessages);

      const { result } = renderHook(() => useIPC());
      let messages;
      await act(async () => {
        messages = await result.current.chat.getHistory(0, 2, 1, 50);
      });

      expect(messages).toEqual(mockMessages);
      expect(invoke).toHaveBeenCalledWith('get_chat_history_handler', {
        sessionType: 0,
        targetId: 2,
        page: 1,
        pageSize: 50,
      });
    });

    it('should call sendMessage with correct params', async () => {
      (invoke as any).mockResolvedValueOnce(123);

      const { result } = renderHook(() => useIPC());
      let messageId;
      await act(async () => {
        messageId = await result.current.chat.sendMessage(0, 2, 'Hello', 1);
      });

      expect(messageId).toBe(123);
      expect(invoke).toHaveBeenCalledWith('send_text_message_handler', {
        sessionType: 0,
        targetId: 2,
        content: 'Hello',
        ownerUid: 1,
      });
    });
  });

  describe('contactApi', () => {
    it('should call getOnlineUsers command', async () => {
      const mockUsers = [
        { uid: 1, nickname: 'Alice' },
        { uid: 2, nickname: 'Bob' },
      ];
      (invoke as any).mockResolvedValueOnce(mockUsers);

      const { result } = renderHook(() => useIPC());
      let users;
      await act(async () => {
        users = await result.current.contact.getOnlineUsers();
      });

      expect(users).toEqual(mockUsers);
      expect(invoke).toHaveBeenCalledWith('get_online_users_handler', undefined);
    });
  });

  describe('fileApi', () => {
    it('should call sendFileRequest with correct params', async () => {
      (invoke as any).mockResolvedValueOnce(1);

      const { result } = renderHook(() => useIPC());
      let requestId;
      await act(async () => {
        requestId = await result.current.file.sendFileRequest(
          ['/path/to/file.pdf'],
          '192.168.1.100',
          1
        );
      });

      expect(requestId).toBe(1);
      expect(invoke).toHaveBeenCalledWith('send_file_request_handler', {
        file_paths: ['/path/to/file.pdf'],
        target_ip: '192.168.1.100',
        owner_uid: 1,
      });
    });
  });

  describe('groupApi', () => {
    it('should call createGroup with correct params', async () => {
      (invoke as any).mockResolvedValueOnce(10);

      const { result } = renderHook(() => useIPC());
      let groupId;
      await act(async () => {
        groupId = await result.current.group.createGroup('Test Group', 1, [1, 2, 3]);
      });

      expect(groupId).toBe(10);
      expect(invoke).toHaveBeenCalledWith('create_group_handler', {
        group_name: 'Test Group',
        creator_uid: 1,
        member_ids: [1, 2, 3],
      });
    });

    it('should call getGroupMembers command', async () => {
      const mockMembers = [{ uid: 1, nickname: 'Alice', role: 1 }];
      (invoke as any).mockResolvedValueOnce(mockMembers);

      const { result } = renderHook(() => useIPC());
      let members;
      await act(async () => {
        members = await result.current.group.getGroupMembers(10);
      });

      expect(members).toEqual(mockMembers);
      expect(invoke).toHaveBeenCalledWith('get_group_members_handler', { gid: 10 });
    });
  });
});
