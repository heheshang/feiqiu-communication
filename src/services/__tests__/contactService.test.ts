import { describe, it, expect, beforeEach, vi } from 'vitest';
import { contactService } from '../contactService';
import * as contactAPI from '../../ipc/contact';

vi.mock('../../ipc/contact', () => ({
  contactAPI: {
    getContactList: vi.fn(),
    getOnlineUsers: vi.fn(),
  },
}));

describe('contactService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getContactList', () => {
    it('should return contact list for user', async () => {
      const mockContacts = [
        {
          id: 1,
          uid: 123,
          owner_uid: 456,
          contact_uid: 123,
          feiq_ip: '192.168.1.100',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE001',
          nickname: 'John Doe',
          status: 1,
          create_time: '2024-01-30T10:00:00Z',
          update_time: '2024-01-30T10:00:00Z',
        },
        {
          id: 2,
          uid: 789,
          owner_uid: 456,
          contact_uid: 789,
          feiq_ip: '192.168.1.101',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE002',
          nickname: 'Jane Smith',
          status: 1,
          create_time: '2024-01-30T09:00:00Z',
          update_time: '2024-01-30T09:00:00Z',
        },
      ];

      (contactAPI.contactAPI.getContactList as any).mockResolvedValueOnce(mockContacts);

      const result = await contactService.getContactList(456);

      expect(contactAPI.contactAPI.getContactList).toHaveBeenCalledWith(456);
      expect(result).toEqual(mockContacts);
      expect(result).toHaveLength(2);
    });

    it('should handle error when fetching contact list fails', async () => {
      const error = new Error('Database error');
      (contactAPI.contactAPI.getContactList as any).mockRejectedValueOnce(error);

      await expect(contactService.getContactList(456)).rejects.toThrow('Database error');
      expect(contactAPI.contactAPI.getContactList).toHaveBeenCalledWith(456);
    });

    it('should return empty array when user has no contacts', async () => {
      (contactAPI.contactAPI.getContactList as any).mockResolvedValueOnce([]);

      const result = await contactService.getContactList(456);

      expect(result).toEqual([]);
      expect(contactAPI.contactAPI.getContactList).toHaveBeenCalledWith(456);
    });

    it('should handle multiple contacts with different statuses', async () => {
      const mockContacts = [
        {
          id: 1,
          uid: 123,
          owner_uid: 456,
          contact_uid: 123,
          feiq_ip: '192.168.1.100',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE001',
          nickname: 'Online User',
          status: 1,
          create_time: '2024-01-30T10:00:00Z',
          update_time: '2024-01-30T10:00:00Z',
        },
        {
          id: 2,
          uid: 789,
          owner_uid: 456,
          contact_uid: 789,
          feiq_ip: '192.168.1.101',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE002',
          nickname: 'Offline User',
          status: 0,
          create_time: '2024-01-30T09:00:00Z',
          update_time: '2024-01-30T09:00:00Z',
        },
      ];

      (contactAPI.contactAPI.getContactList as any).mockResolvedValueOnce(mockContacts);

      const result = await contactService.getContactList(456);

      expect(result).toHaveLength(2);
      expect(result[0].status).toBe(1);
      expect(result[1].status).toBe(0);
    });

    it('should handle contacts with optional fields', async () => {
      const mockContacts = [
        {
          id: 1,
          uid: 123,
          owner_uid: 456,
          contact_uid: 123,
          feiq_ip: '192.168.1.100',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE001',
          nickname: 'User with avatar',
          status: 1,
          avatar: 'https://example.com/avatar.jpg',
          create_time: '2024-01-30T10:00:00Z',
          update_time: '2024-01-30T10:00:00Z',
        },
      ];

      (contactAPI.contactAPI.getContactList as any).mockResolvedValueOnce(mockContacts);

      const result = await contactService.getContactList(456);

      expect(result[0].avatar).toBe('https://example.com/avatar.jpg');
    });
  });

  describe('getOnlineUsers', () => {
    it('should return list of online users', async () => {
      const mockOnlineUsers = [
        {
          uid: 123,
          feiq_ip: '192.168.1.100',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE001',
          nickname: 'John Doe',
          status: 1,
          create_time: '2024-01-30T10:00:00Z',
          update_time: '2024-01-30T10:00:00Z',
        },
        {
          uid: 789,
          feiq_ip: '192.168.1.101',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE002',
          nickname: 'Jane Smith',
          status: 1,
          create_time: '2024-01-30T09:00:00Z',
          update_time: '2024-01-30T09:00:00Z',
        },
      ];

      (contactAPI.contactAPI.getOnlineUsers as any).mockResolvedValueOnce(mockOnlineUsers);

      const result = await contactService.getOnlineUsers(456);

      expect(contactAPI.contactAPI.getOnlineUsers).toHaveBeenCalledWith(456);
      expect(result).toEqual(mockOnlineUsers);
      expect(result).toHaveLength(2);
    });

    it('should handle error when fetching online users fails', async () => {
      const error = new Error('Network error');
      (contactAPI.contactAPI.getOnlineUsers as any).mockRejectedValueOnce(error);

      await expect(contactService.getOnlineUsers(456)).rejects.toThrow('Network error');
      expect(contactAPI.contactAPI.getOnlineUsers).toHaveBeenCalledWith(456);
    });

    it('should return empty array when no users are online', async () => {
      (contactAPI.contactAPI.getOnlineUsers as any).mockResolvedValueOnce([]);

      const result = await contactService.getOnlineUsers(456);

      expect(result).toEqual([]);
      expect(contactAPI.contactAPI.getOnlineUsers).toHaveBeenCalledWith(456);
    });

    it('should handle multiple online users', async () => {
      const mockOnlineUsers = Array.from({ length: 10 }, (_, i) => ({
        uid: 100 + i,
        feiq_ip: `192.168.1.${100 + i}`,
        feiq_port: 2425,
        feiq_machine_id: `MACHINE${String(i).padStart(3, '0')}`,
        nickname: `User ${i + 1}`,
        status: 1,
        create_time: '2024-01-30T10:00:00Z',
        update_time: '2024-01-30T10:00:00Z',
      }));

      (contactAPI.contactAPI.getOnlineUsers as any).mockResolvedValueOnce(mockOnlineUsers);

      const result = await contactService.getOnlineUsers(456);

      expect(result).toHaveLength(10);
      expect(result[0].nickname).toBe('User 1');
      expect(result[9].nickname).toBe('User 10');
    });

    it('should handle users with different online statuses', async () => {
      const mockUsers = [
        {
          uid: 123,
          feiq_ip: '192.168.1.100',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE001',
          nickname: 'Online User',
          status: 1,
          create_time: '2024-01-30T10:00:00Z',
          update_time: '2024-01-30T10:00:00Z',
        },
        {
          uid: 789,
          feiq_ip: '192.168.1.101',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE002',
          nickname: 'Busy User',
          status: 2,
          create_time: '2024-01-30T09:00:00Z',
          update_time: '2024-01-30T09:00:00Z',
        },
        {
          uid: 999,
          feiq_ip: '192.168.1.102',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE003',
          nickname: 'Away User',
          status: 3,
          create_time: '2024-01-30T08:00:00Z',
          update_time: '2024-01-30T08:00:00Z',
        },
      ];

      (contactAPI.contactAPI.getOnlineUsers as any).mockResolvedValueOnce(mockUsers);

      const result = await contactService.getOnlineUsers(456);

      expect(result).toHaveLength(3);
      expect(result[0].status).toBe(1);
      expect(result[1].status).toBe(2);
      expect(result[2].status).toBe(3);
    });

    it('should handle users with optional avatar field', async () => {
      const mockUsers = [
        {
          uid: 123,
          feiq_ip: '192.168.1.100',
          feiq_port: 2425,
          feiq_machine_id: 'MACHINE001',
          nickname: 'User with avatar',
          status: 1,
          avatar: 'https://example.com/avatar.jpg',
          create_time: '2024-01-30T10:00:00Z',
          update_time: '2024-01-30T10:00:00Z',
        },
      ];

      (contactAPI.contactAPI.getOnlineUsers as any).mockResolvedValueOnce(mockUsers);

      const result = await contactService.getOnlineUsers(456);

      expect(result[0].avatar).toBe('https://example.com/avatar.jpg');
    });
  });
});
