// 组件 - 联系人项
// TODO: Phase 4 时完善联系人项组件

import React from 'react';
import type { UserInfo } from '../../types';

import './ContactItem.less';

interface ContactItemProps {
  user: UserInfo;
  onClick?: (user: UserInfo) => void;
}

export const ContactItem: React.FC<ContactItemProps> = ({ user, onClick }) => {
  const handleClick = () => {
    onClick?.(user);
  };

  return (
    <div className="contact-item" onClick={handleClick}>
      <div className="contact-avatar">
        {user.avatar ? (
          <img src={user.avatar} alt={user.nickname} />
        ) : (
          <div className="avatar-placeholder">{user.nickname[0]}</div>
        )}
      </div>
      <div className="contact-info">
        <div className="contact-name">{user.nickname}</div>
        <div className="contact-machine">{user.feiq_machine_id}</div>
      </div>
      <div className={`contact-status ${user.status === 1 ? 'online' : 'offline'}`}></div>
    </div>
  );
};
