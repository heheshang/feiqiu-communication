import React from 'react';
import { SessionList } from '../../SessionList/SessionList';
import { GroupList } from '../../GroupList';

interface SidebarProps {
  activeTab: 'chats' | 'groups';
  selectedUserId?: number;
  showBackButton: boolean;
  onBackToList: () => void;
  onTabChange: (tab: 'chats' | 'groups') => void;
  onSessionSelect: (sessionId: number, userId: number) => void;
  onCreateGroupOpen: () => void;
  onSelectGroup: (groupId: number) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({
  activeTab,
  selectedUserId,
  showBackButton,
  onBackToList,
  onTabChange,
  onSessionSelect,
  onCreateGroupOpen,
  onSelectGroup,
}) => {
  return (
    <div className="layout-sidebar">
      {showBackButton && (
        <div className="sidebar-back" onClick={onBackToList}>
          <svg viewBox="0 0 24 24" fill="none">
            <path
              d="M15 18L9 12L15 6"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </div>
      )}

      <div className="sidebar-tabs">
        <button
          className={`tab-button ${activeTab === 'chats' ? 'active' : ''}`}
          onClick={() => onTabChange('chats')}
        >
          Chats
        </button>
        <button
          className={`tab-button ${activeTab === 'groups' ? 'active' : ''}`}
          onClick={() => onTabChange('groups')}
        >
          Groups
        </button>
      </div>

      {activeTab === 'chats' ? (
        <SessionList selectedUserId={selectedUserId} onSessionSelect={onSessionSelect} />
      ) : (
        <div className="groups-container">
          <div className="groups-header">
            <button className="create-group-btn" onClick={onCreateGroupOpen}>
              + Create Group
            </button>
          </div>
          <GroupList onSelectGroup={onSelectGroup} />
        </div>
      )}
    </div>
  );
};
