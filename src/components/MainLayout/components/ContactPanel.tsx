import React from 'react';
import ContactList from '../../Contact/ContactList';
import type { UserInfo } from '../../../types';

interface ContactPanelProps {
  users: UserInfo[];
  showBackButton: boolean;
  onBackToList: () => void;
  onUserClick: (user: UserInfo) => void;
}

export const ContactPanel: React.FC<ContactPanelProps> = ({
  users,
  showBackButton,
  onBackToList,
  onUserClick,
}) => {
  return (
    <div className="layout-contact">
      {showBackButton && (
        <div className="contact-back" onClick={onBackToList}>
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
      <ContactList users={users} onUserClick={onUserClick} />
    </div>
  );
};
