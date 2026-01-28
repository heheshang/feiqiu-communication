// 组件 - 联系人分组

import React, { useState } from 'react';

import './ContactGroup.less';

interface ContactGroupProps {
  groupName: string;
  children: React.ReactNode;
  defaultExpanded?: boolean;
  count?: number;
}

export const ContactGroup: React.FC<ContactGroupProps> = ({
  groupName,
  children,
  defaultExpanded = true,
  count,
}) => {
  const [isExpanded, setIsExpanded] = useState(defaultExpanded);

  const handleToggle = () => {
    setIsExpanded(!isExpanded);
  };

  return (
    <div className="contact-group">
      <div className="contact-group-header" onClick={handleToggle}>
        <div className="group-header-left">
          <svg
            className={`group-expand-icon ${isExpanded ? 'expanded' : ''}`}
            viewBox="0 0 24 24"
            fill="none"
          >
            <path
              d="M9 18L15 12L9 6"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
          <span className="group-name">{groupName}</span>
          {count !== undefined && <span className="group-count">({count})</span>}
        </div>
      </div>
      {isExpanded && <div className="contact-group-body">{children}</div>}
    </div>
  );
};
