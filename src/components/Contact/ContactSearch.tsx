// 组件 - 联系人搜索
// TODO: Phase 4 时完善联系人搜索组件

import React, { useState } from 'react';

import './ContactSearch.less';

interface ContactSearchProps {
  onSearch?: (keyword: string) => void;
}

export const ContactSearch: React.FC<ContactSearchProps> = ({ onSearch }) => {
  const [keyword, setKeyword] = useState('');

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setKeyword(value);
    onSearch?.(value);
  };

  return (
    <div className="contact-search">
      <input
        type="text"
        className="search-input"
        placeholder="搜索联系人..."
        value={keyword}
        onChange={handleChange}
      />
    </div>
  );
};
