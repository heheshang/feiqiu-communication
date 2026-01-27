// 组件 - 联系人列表
// TODO: Phase 4 时完善联系人列表组件

import React from 'react';
import { useContact } from '../../hooks/useContact';
import type { UserInfo } from '../../types';

import './ContactList.less';

export const ContactList: React.FC = () => {
  const { onlineUsers, getOnlineUsers } = useContact();

  React.useEffect(() => {
    getOnlineUsers();
  }, [getOnlineUsers]);

  return (
    <div className="contact-list">
      <div className="contact-list-header">
        <h3>通讯录</h3>
        <span className="online-count">{onlineUsers.length} 人在线</span>
      </div>
      <div className="contact-list-body">
        {onlineUsers.map((user) => (
          <ContactItem key={user.uid} user={user} />
        ))}
      </div>
    </div>
  );
};

interface ContactItemProps {
  user: UserInfo;
}

const ContactItem: React.FC<ContactItemProps> = ({ user }) => {
  return (
    <div className="contact-item">
      <div className="contact-avatar">
        {user.avatar ? <img src={user.avatar} alt={user.nickname} /> : <div className="avatar-placeholder">{user.nickname[0]}</div>}
      </div>
      <div className="contact-info">
        <div className="contact-name">{user.nickname}</div>
        <div className="contact-machine">{user.feiq_machine_id}</div>
      </div>
      <div className={`contact-status ${user.status === 1 ? 'online' : 'offline'}`}></div>
    </div>
  );
};
