import React, { useState } from 'react';
import './CreateGroupDialog.less';

interface CreateGroupDialogProps {
  isOpen: boolean;
  onClose: () => void;
  availableMembers: Array<{ id: string; name: string }>;
}

export const CreateGroupDialog: React.FC<CreateGroupDialogProps> = ({
  isOpen,
  onClose,
  availableMembers,
}) => {
  const [groupName, setGroupName] = useState('');
  const [selectedMembers, setSelectedMembers] = useState<Set<string>>(new Set());

  const handleMemberToggle = (memberId: string) => {
    const newSelected = new Set(selectedMembers);
    if (newSelected.has(memberId)) {
      newSelected.delete(memberId);
    } else {
      newSelected.add(memberId);
    }
    setSelectedMembers(newSelected);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!groupName.trim()) {
      alert('Please enter a group name');
      return;
    }

    if (selectedMembers.size === 0) {
      alert('Please select at least one member');
      return;
    }

    try {
      setGroupName('');
      setSelectedMembers(new Set());
      onClose();
    } catch (error) {
      console.error('Failed to create group:', error);
      alert('Failed to create group');
    }
  };

  if (!isOpen) return null;

  return (
    <div className="create-group-dialog-overlay">
      <div className="create-group-dialog">
        <div className="dialog-header">
          <h2 className="dialog-title">Create New Group</h2>
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
          <form onSubmit={handleSubmit}>
            <div className="form-group">
              <label>Group Name</label>
              <input
                type="text"
                value={groupName}
                onChange={(e) => setGroupName(e.target.value)}
                placeholder="Enter group name"
              />
            </div>

            <div className="form-group">
              <label>Select Members</label>
              {availableMembers.length === 0 ? (
                <p style={{ color: '#999', fontSize: '14px' }}>No members available</p>
              ) : (
                <div className="members-list">
                  {availableMembers.map((member) => (
                    <label key={member.id} className="member-item">
                      <input
                        type="checkbox"
                        checked={selectedMembers.has(member.id)}
                        onChange={() => handleMemberToggle(member.id)}
                      />
                      <div className="member-info">
                        <span className="member-name">{member.name}</span>
                      </div>
                    </label>
                  ))}
                </div>
              )}
            </div>
          </form>
        </div>

        <div className="dialog-footer">
          <button type="button" onClick={onClose} className="btn-cancel">
            Cancel
          </button>
          <button type="submit" onClick={handleSubmit} className="btn-confirm">
            Create Group
          </button>
        </div>
      </div>
    </div>
  );
};
