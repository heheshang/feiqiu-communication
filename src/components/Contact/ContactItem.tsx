// src/components/Contact/ContactItem.tsx
//
/// 单个联系人项组件

import React from 'react';
import './ContactItem.less';
import type { UserInfo } from '../../types';

interface ContactItemProps {
  user: UserInfo;
  onClick?: () => void;
  className?: string;
}

const ContactItem: React.FC<ContactItemProps> = ({ user, onClick, className = '' }) => {
  const statusClass = user.status === 1 ? 'online' : user.status === 2 ? 'busy' : 'offline';

  return (
    <div
      className={`contact-item ${className} ${statusClass}`}
      onClick={onClick}
      title={`IP: ${user.feiq_ip}\n端口: ${user.feiq_port}`}
    >
      {/* 头像 */}
      <div className="contact-avatar">
        {user.avatar ? (
          <img src={user.avatar} alt={user.nickname} />
        ) : (
          <div className="avatar-placeholder">
            {user.nickname.charAt(0).toUpperCase()}
          </div>
        )}
      </div>

      {/* 用户信息 */}
      <div className="contact-info">
        <div className="contact-name">{user.nickname}</div>
        <div className="contact-machine">{user.feiq_machine_id}</div>
      </div>

      {/* 在线状态指示器 */}
      <div className={`contact-status-indicator ${statusClass}`} />
    </div>
  );
};

export default ContactItem;
