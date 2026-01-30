import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useContact } from '../useContact';
import * as store from '../../store';
import * as contactService from '../../services/contactService';

// Mock the store
vi.mock('../../store', () => {
  const mockStore = vi.fn() as any;
  mockStore.getState = vi.fn();
  return {
    useUserStore: mockStore,
  };
});

describe('useContact', () => {
  const mockOnlineUsers = new Map([
    [
      '192.168.1.100',
      {
        uid: 1,
        feiq_ip: '192.168.1.100',
        feiq_port: 2425,
        feiq_machine_id: 'PC-001',
        nickname: 'Alice',
        avatar: undefined,
        status: 1,
        create_time: '2024-01-30T10:00:00Z',
        update_time: '2024-01-30T10:00:00Z',
      },
    ],
    [
      '192.168.1.101',
      {
        uid: 2,
        feiq_ip: '192.168.1.101',
        feiq_port: 2425,
        feiq_machine_id: 'PC-002',
        nickname: 'Bob',
        avatar: undefined,
        status: 1,
        create_time: '2024-01-30T10:00:00Z',
        update_time: '2024-01-30T10:00:00Z',
      },
    ],
  ]);

  const mockCurrentUser = {
    uid: 1,
    feiq_ip: '192.168.1.100',
    feiq_port: 2425,
    feiq_machine_id: 'PC-001',
    nickname: 'Alice',
    avatar: undefined,
    status: 1,
    create_time: '2024-01-30T10:00:00Z',
    update_time: '2024-01-30T10:00:00Z',
  };

  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
    vi.spyOn(contactService.contactService, 'getOnlineUsers').mockResolvedValue([]);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  describe('initialization', () => {
    it('should initialize and get online users on mount', async () => {
      const mockGetOnlineUsers = vi.fn();
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: mockGetOnlineUsers,
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      renderHook(() => useContact());

      expect(mockGetOnlineUsers).toHaveBeenCalled();
    });
  });

  describe('getOnlineUsersList', () => {
    it('should return online users as array', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      expect(result.current.onlineUsers).toHaveLength(2);
      expect(result.current.onlineUsers[0].nickname).toBe('Alice');
      expect(result.current.onlineUsers[1].nickname).toBe('Bob');
    });

    it('should return empty array when no online users', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: new Map(),
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      expect(result.current.onlineUsers).toEqual([]);
      expect(result.current.onlineCount).toBe(0);
    });
  });

  describe('searchUsers', () => {
    it('should filter users by keyword in nickname', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      const searchResults = result.current.searchUsers({ keyword: 'alice' });

      expect(searchResults).toHaveLength(1);
      expect(searchResults[0].nickname).toBe('Alice');
    });

    it('should filter users by keyword in machine_id', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      const searchResults = result.current.searchUsers({ keyword: 'PC-002' });

      expect(searchResults).toHaveLength(1);
      expect(searchResults[0].feiq_machine_id).toBe('PC-002');
    });

    it('should be case-insensitive when filtering by keyword', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      const searchResults = result.current.searchUsers({ keyword: 'ALICE' });

      expect(searchResults).toHaveLength(1);
      expect(searchResults[0].nickname).toBe('Alice');
    });

    it('should filter users by status', () => {
      const usersWithDifferentStatus = new Map([
        ['192.168.1.100', { ...mockOnlineUsers.get('192.168.1.100'), status: 1 }],
        ['192.168.1.101', { ...mockOnlineUsers.get('192.168.1.101'), status: 0 }],
      ]);

      (store.useUserStore as any).mockReturnValue({
        onlineUsers: usersWithDifferentStatus,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      const onlineUsers = result.current.searchUsers({ status: 1 });

      expect(onlineUsers).toHaveLength(1);
      expect(onlineUsers[0].status).toBe(1);
    });

    it('should support pagination', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      const page1 = result.current.searchUsers({ page: 1, page_size: 1 });

      expect(page1).toHaveLength(1);
      expect(page1[0].nickname).toBe('Alice');
    });

    it('should combine multiple filters', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      const searchResults = result.current.searchUsers({
        keyword: 'alice',
        status: 1,
      });

      expect(searchResults).toHaveLength(1);
      expect(searchResults[0].nickname).toBe('Alice');
      expect(searchResults[0].status).toBe(1);
    });
  });

  describe('findUserByIp', () => {
    it('should find user by IP address', () => {
      const mockFindOnlineUser = vi.fn((ip: string) => mockOnlineUsers.get(ip));
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: mockFindOnlineUser,
      });

      const { result } = renderHook(() => useContact());

      const user = result.current.findUserByIp('192.168.1.100');

      expect(user).toBeDefined();
      expect(user?.nickname).toBe('Alice');
      expect(mockFindOnlineUser).toHaveBeenCalledWith('192.168.1.100');
    });

    it('should return undefined when user not found', () => {
      const mockFindOnlineUser = vi.fn((ip: string) => mockOnlineUsers.get(ip));
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: mockFindOnlineUser,
      });

      const { result } = renderHook(() => useContact());

      const user = result.current.findUserByIp('192.168.1.999');

      expect(user).toBeUndefined();
    });
  });

  describe('addOnline', () => {
    it('should add user to online list', () => {
      const mockAddOnlineUser = vi.fn();
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: mockAddOnlineUser,
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      const newUser = {
        uid: 3,
        feiq_ip: '192.168.1.102',
        feiq_port: 2425,
        feiq_machine_id: 'PC-003',
        nickname: 'Charlie',
        avatar: undefined,
        status: 1,
        create_time: '2024-01-30T10:00:00Z',
        update_time: '2024-01-30T10:00:00Z',
      };

      act(() => {
        result.current.addOnline(newUser);
      });

      expect(mockAddOnlineUser).toHaveBeenCalledWith(newUser);
    });
  });

  describe('removeOnline', () => {
    it('should remove user from online list', () => {
      const mockRemoveOnlineUser = vi.fn();
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: mockRemoveOnlineUser,
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      act(() => {
        result.current.removeOnline('192.168.1.100');
      });

      expect(mockRemoveOnlineUser).toHaveBeenCalledWith('192.168.1.100');
    });
  });

  describe('refreshOnlineUsers', () => {
    it('should refresh online users from service', async () => {
      const mockSetOnlineUsers = vi.fn();
      const refreshedUsers = Array.from(mockOnlineUsers.values());

      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: mockSetOnlineUsers,
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      (store.useUserStore as any).getState = vi.fn().mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (contactService.contactService.getOnlineUsers as any).mockResolvedValueOnce(refreshedUsers);

      const { result } = renderHook(() => useContact());

      await act(async () => {
        await result.current.refreshOnlineUsers();
      });

      expect(contactService.contactService.getOnlineUsers).toHaveBeenCalledWith(
        mockCurrentUser.uid
      );
      expect(mockSetOnlineUsers).toHaveBeenCalledWith(refreshedUsers);
    });

    it('should handle error when refresh fails', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const mockSetOnlineUsers = vi.fn();

      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: mockSetOnlineUsers,
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      (store.useUserStore as any).getState = vi.fn().mockReturnValue({
        currentUser: mockCurrentUser,
      });

      (contactService.contactService.getOnlineUsers as any).mockRejectedValueOnce(
        new Error('Network error')
      );

      const { result } = renderHook(() => useContact());

      await act(async () => {
        await result.current.refreshOnlineUsers();
      });

      expect(consoleErrorSpy).toHaveBeenCalledWith(
        'Failed to refresh online users:',
        expect.any(Error)
      );

      consoleErrorSpy.mockRestore();
    });

    it('should handle error when no current user', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: null,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      (store.useUserStore as any).getState = vi.fn().mockReturnValue({
        currentUser: null,
      });

      const { result } = renderHook(() => useContact());

      await act(async () => {
        await result.current.refreshOnlineUsers();
      });

      expect(consoleErrorSpy).toHaveBeenCalledWith('No current user found');

      consoleErrorSpy.mockRestore();
    });
  });

  describe('getOnlineCount', () => {
    it('should return correct count of online users', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: mockOnlineUsers,
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      expect(result.current.onlineCount).toBe(2);
    });

    it('should return 0 when no online users', () => {
      (store.useUserStore as any).mockReturnValue({
        onlineUsers: new Map(),
        currentUser: mockCurrentUser,
        getOnlineUsers: vi.fn(),
        setOnlineUsers: vi.fn(),
        addOnlineUser: vi.fn(),
        removeOnlineUser: vi.fn(),
        findOnlineUser: vi.fn(),
      });

      const { result } = renderHook(() => useContact());

      expect(result.current.onlineCount).toBe(0);
    });
  });
});
