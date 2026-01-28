// src/components/Contact/ContactList.tsx
//
/// 联系人列表组件
/// 显示在线用户，支持搜索过滤

import React, { useState, useEffect } from 'react';
import ContactItem from './ContactItem';
import ContactSearch from './ContactSearch';
import './ContactList.less';
import type { UserInfo } from '../../types';

interface ContactListProps {
  users?: UserInfo[];
  onUserClick?: (user: UserInfo) => void;
}

const ContactList: React.FC<ContactListProps> = ({ users = [], onUserClick }) => {
  const [filteredUsers, setFilteredUsers] = useState<UserInfo[]>(users);
  const [searchQuery, setSearchQuery] = useState('');

  // 更新过滤后的用户列表
  useEffect(() => {
    if (!searchQuery) {
      setFilteredUsers(users);
    } else {
      const query = searchQuery.toLowerCase();
      const filtered = users.filter(
        (user) =>
          user.nickname.toLowerCase().includes(query) ||
          user.feiq_machine_id.toLowerCase().includes(query)
      );
      setFilteredUsers(filtered);
    }
  }, [users, searchQuery]);

  const handleSearch = (query: string) => {
    setSearchQuery(query);
  };

  const onlineCount = users.filter((u) => u.status === 1).length;

  return (
    <div className="contact-list">
      {/* 头部：标题和搜索 */}
      <div className="contact-list-header">
        <h3 className="contact-list-title">通讯录</h3>
        <span className="online-count">{onlineCount} 人在线</span>
      </div>

      {/* 搜索框 */}
      <div className="contact-list-search">
        <ContactSearch onSearch={handleSearch} placeholder="搜索联系人..." />
      </div>

      {/* 用户列表 */}
      <div className="contact-list-body">
        {filteredUsers.length === 0 ? (
          <div className="contact-empty">{searchQuery ? '未找到匹配的联系人' : '暂无在线用户'}</div>
        ) : (
          filteredUsers.map((user) => (
            <ContactItem key={user.uid} user={user} onClick={() => onUserClick?.(user)} />
          ))
        )}
      </div>
    </div>
  );
};

export default ContactList;
