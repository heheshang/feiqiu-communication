// src/components/GroupSettingsDialog.tsx
//
/// Group settings dialog - edit group name, description, avatar, delete group

import React, { useState } from 'react';
import { groupService } from '../services/groupService';
import { useUserStore } from '../store/userStore';
import type { GroupInfo } from '../types';
import './GroupSettingsDialog.less';

interface GroupSettingsDialogProps {
  isOpen: boolean;
  onClose: () => void;
  group: GroupInfo;
  onGroupUpdated: () => void;
}

export const GroupSettingsDialog: React.FC<GroupSettingsDialogProps> = ({
  isOpen,
  onClose,
  group,
  onGroupUpdated,
}) => {
  const currentUser = useUserStore((state) => state.currentUser);
  const [groupName, setGroupName] = useState(group.group_name);
  const [groupDesc, setGroupDesc] = useState(group.desc || '');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [showLeaveConfirm, setShowLeaveConfirm] = useState(false);
  const [activeTab, setActiveTab] = useState<'info' | 'notifications'>('info');

  const isOwner = currentUser && currentUser.uid === group.creator_uid;

  const handleSaveInfo = async () => {
    if (!groupName.trim()) {
      alert('Group name cannot be empty');
      return;
    }

    setIsSubmitting(true);
    try {
      await groupService.updateGroupInfo(group.gid, groupName, groupDesc);
      onGroupUpdated();
      onClose();
    } catch (error) {
      console.error('Failed to update group:', error);
      alert('Failed to update group. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDeleteGroup = async () => {
    if (!showDeleteConfirm) {
      setShowDeleteConfirm(true);
      return;
    }

    setIsSubmitting(true);
    try {
      await groupService.deleteGroup(group.gid);
      onGroupUpdated();
      onClose();
    } catch (error) {
      console.error('Failed to delete group:', error);
      alert('Failed to delete group. Please try again.');
    } finally {
      setIsSubmitting(false);
      setShowDeleteConfirm(false);
    }
  };

  const handleLeaveGroup = async () => {
    if (!showLeaveConfirm) {
      setShowLeaveConfirm(true);
      return;
    }

    if (!currentUser) return;

    setIsSubmitting(true);
    try {
      await groupService.removeGroupMember(group.gid, currentUser.uid);
      onGroupUpdated();
      onClose();
    } catch (error) {
      console.error('Failed to leave group:', error);
      alert('Failed to leave group. Please try again.');
    } finally {
      setIsSubmitting(false);
      setShowLeaveConfirm(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="group-settings-dialog-overlay">
      <div className="group-settings-dialog">
        <div className="dialog-header">
          <h2 className="dialog-title">Group Settings</h2>
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
          {/* Tabs */}
          <div className="settings-tabs">
            <button
              className={`tab-btn ${activeTab === 'info' ? 'active' : ''}`}
              onClick={() => setActiveTab('info')}
            >
              Group Info
            </button>
            <button
              className={`tab-btn ${activeTab === 'notifications' ? 'active' : ''}`}
              onClick={() => setActiveTab('notifications')}
            >
              Notifications
            </button>
          </div>

          {/* Info Tab */}
          {activeTab === 'info' && (
            <div className="tab-content">
              <div className="form-group">
                <label>Group Name</label>
                <input
                  type="text"
                  value={groupName}
                  onChange={(e) => setGroupName(e.target.value)}
                  placeholder="Enter group name"
                  maxLength={50}
                />
                <span className="char-count">{groupName.length}/50</span>
              </div>

              <div className="form-group">
                <label>Description (Optional)</label>
                <textarea
                  value={groupDesc}
                  onChange={(e) => setGroupDesc(e.target.value)}
                  placeholder="Add a group description..."
                  rows={4}
                  maxLength={200}
                />
                <span className="char-count">{groupDesc.length}/200</span>
              </div>

              {/* Avatar placeholder - future enhancement */}
              <div className="form-group">
                <label>Group Avatar</label>
                <div className="avatar-upload-placeholder">
                  <div className="current-avatar">
                    {group.avatar ? (
                      <img src={group.avatar} alt={group.group_name} />
                    ) : (
                      <div className="avatar-placeholder">{group.group_name.charAt(0)}</div>
                    )}
                  </div>
                  <button type="button" className="btn-upload" disabled>
                    Change Avatar (Coming Soon)
                  </button>
                </div>
              </div>

              {/* Delete Group Section (Owner Only) */}
              {isOwner && (
                <div className="danger-zone">
                  <h4>Danger Zone</h4>
                  <p>Once you delete a group, there is no going back. Please be certain.</p>
                  {!showDeleteConfirm ? (
                    <button
                      type="button"
                      className="btn-delete"
                      onClick={() => setShowDeleteConfirm(true)}
                    >
                      Delete Group
                    </button>
                  ) : (
                    <div className="delete-confirm">
                      <p className="confirm-text">
                        Are you sure? This will delete <strong>{group.group_name}</strong> for all
                        members.
                      </p>
                      <div className="confirm-actions">
                        <button
                          type="button"
                          className="btn-cancel-delete"
                          onClick={() => setShowDeleteConfirm(false)}
                          disabled={isSubmitting}
                        >
                          Cancel
                        </button>
                        <button
                          type="button"
                          className="btn-confirm-delete"
                          onClick={handleDeleteGroup}
                          disabled={isSubmitting}
                        >
                          {isSubmitting ? 'Deleting...' : 'Yes, Delete Group'}
                        </button>
                      </div>
                    </div>
                  )}
                </div>
              )}

              {/* Leave Group Section (Non-Owner) */}
              {!isOwner && (
                <div className="leave-zone">
                  <h4>Leave Group</h4>
                  <p>
                    You can leave this group at any time. You won't receive any more messages from
                    it.
                  </p>
                  {!showLeaveConfirm ? (
                    <button
                      type="button"
                      className="btn-leave"
                      onClick={() => setShowLeaveConfirm(true)}
                    >
                      Leave Group
                    </button>
                  ) : (
                    <div className="leave-confirm">
                      <p className="confirm-text">
                        Are you sure you want to leave <strong>{group.group_name}</strong>?
                      </p>
                      <div className="confirm-actions">
                        <button
                          type="button"
                          className="btn-cancel-leave"
                          onClick={() => setShowLeaveConfirm(false)}
                          disabled={isSubmitting}
                        >
                          Cancel
                        </button>
                        <button
                          type="button"
                          className="btn-confirm-leave"
                          onClick={handleLeaveGroup}
                          disabled={isSubmitting}
                        >
                          {isSubmitting ? 'Leaving...' : 'Yes, Leave Group'}
                        </button>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>
          )}

          {/* Notifications Tab */}
          {activeTab === 'notifications' && (
            <div className="tab-content">
              <div className="notification-settings">
                <p className="info-text">
                  Notification settings are coming soon. You'll be able to configure:
                </p>
                <ul className="feature-list">
                  <li>Mute/unmute group notifications</li>
                  <li>@mention notifications only</li>
                  <li>Do not disturb mode</li>
                  <li>Notification sound preferences</li>
                </ul>
              </div>
            </div>
          )}
        </div>

        <div className="dialog-footer">
          <button type="button" onClick={onClose} className="btn-cancel" disabled={isSubmitting}>
            Cancel
          </button>
          {activeTab === 'info' && (
            <button
              type="button"
              onClick={handleSaveInfo}
              className="btn-save"
              disabled={
                isSubmitting || (groupName === group.group_name && groupDesc === (group.desc || ''))
              }
            >
              {isSubmitting ? 'Saving...' : 'Save Changes'}
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
