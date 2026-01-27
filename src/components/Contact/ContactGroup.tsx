// 组件 - 联系人分组
// TODO: Phase 4 时完善联系人分组组件

import React from 'react';

import './ContactGroup.less';

interface ContactGroupProps {
  groupName: string;
  children: React.ReactNode;
}

export const ContactGroup: React.FC<ContactGroupProps> = ({ groupName, children }) => {
  return (
    <div className="contact-group">
      <div className="contact-group-header">{groupName}</div>
      <div className="contact-group-body">{children}</div>
    </div>
  );
};
