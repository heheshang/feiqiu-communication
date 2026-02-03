// src/components/AddMemberDialog.tsx
//
/// Add member dialog for existing groups

import React, { useState, useEffect } from 'react';
import { contactAPI } from '../ipc/contact';
import { groupService } from '../services/groupService';
import type { UserInfo } from '../types';
import './AddMemberDialog.less';

interface AddMemberDialogProps {
  isOpen: boolean;
  onClose: () => void;
  gid: number;
  currentMemberUids: number[];
  onMembersAdded: () => void;
  currentUserId: number;
}

export const AddMemberDialog: React.FC<AddMemberDialogProps> = ({
  isOpen,
  onClose,
  gid,
  currentMemberUids,
  onMembersAdded,
  currentUserId,
}) => {
  const [availableUsers, setAvailableUsers] = useState<UserInfo[]>([]);
  const [selectedUsers, setSelectedUsers] = useState<Set<number>>(new Set());
  const [isLoading, setIsLoading] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Fetch available contacts when dialog opens
  useEffect(() => {
    if (isOpen) {
      fetchAvailableUsers();
    }
  }, [isOpen]);

  const fetchAvailableUsers = async () => {
    setIsLoading(true);
    try {
      // Try to get online users first, fall back to contact list
      let users: UserInfo[] = [];
      try {
        users = await contactAPI.getOnlineUsers();
      } catch {
        try {
          users = await contactAPI.getContactList(currentUserId);
        } catch (error) {
          console.error('Failed to fetch users:', error);
        }
      }

      // Filter out users who are already members
      const filteredUsers = users.filter((user) => !currentMemberUids.includes(user.uid));
      setAvailableUsers(filteredUsers);
    } catch (error) {
      console.error('Failed to fetch available users:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleUserToggle = (userUid: number) => {
    const newSelected = new Set(selectedUsers);
    if (newSelected.has(userUid)) {
      newSelected.delete(userUid);
    } else {
      newSelected.add(userUid);
    }
    setSelectedUsers(newSelected);
  };

  const handleAddMembers = async () => {
    if (selectedUsers.size === 0) {
      alert('Please select at least one member');
      return;
    }

    setIsSubmitting(true);
    try {
      // Add each selected user to the group
      for (const userUid of selectedUsers) {
        await groupService.addGroupMember(gid, userUid, 0); // role=0 means regular member
      }

      // Reset state
      setSelectedUsers(new Set());
      onMembersAdded(); // Refresh member list
      onClose();
    } catch (error) {
      console.error('Failed to add members:', error);
      alert('Failed to add members. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="add-member-dialog-overlay">
      <div className="add-member-dialog">
        <div className="dialog-header">
          <h2 className="dialog-title">Add Members</h2>
          <button
            type="button"
            onClick={onClose}
            className="dialog-close"
            aria-label="Close dialog"
          >
            ✕
          </button>
        </div>

        <div className="dialog-body">
          {isLoading ? (
            <div className="loading-state">Loading available users...</div>
          ) : availableUsers.length === 0 ? (
            <div className="empty-state">
              <p>No users available to add.</p>
              <p className="hint">Users who are already group members are filtered out.</p>
            </div>
          ) : (
            <div className="users-list">
              {availableUsers.map((user) => (
                <label key={user.uid} className="user-item">
                  <input
                    type="checkbox"
                    checked={selectedUsers.has(user.uid)}
                    onChange={() => handleUserToggle(user.uid)}
                  />
                  <div className="user-info">
                    <div className="user-avatar">{user.nickname.charAt(0)}</div>
                    <div className="user-details">
                      <span className="user-name">{user.nickname}</span>
                      <span className="user-ip">{user.feiq_ip || 'Unknown'}</span>
                    </div>
                  </div>
                </label>
              ))}
            </div>
          )}
        </div>

        <div className="dialog-footer">
          <button type="button" onClick={onClose} className="btn-cancel" disabled={isSubmitting}>
            Cancel
          </button>
          <button
            type="button"
            onClick={handleAddMembers}
            className="btn-confirm"
            disabled={isSubmitting || selectedUsers.size === 0}
          >
            {isSubmitting
              ? 'Adding...'
              : `Add ${selectedUsers.size} Member${selectedUsers.size !== 1 ? 's' : ''}`}
          </button>
        </div>
      </div>
    </div>
  );
};
