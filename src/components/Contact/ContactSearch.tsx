// src/components/Contact/ContactSearch.tsx
//
/// 联系人搜索组件

import React, { useState, useCallback } from 'react';
import './ContactSearch.less';

interface ContactSearchProps {
  placeholder?: string;
  onSearch?: (query: string) => void;
}

const ContactSearch: React.FC<ContactSearchProps> = ({
  placeholder = '搜索...',
  onSearch
}) => {
  const [value, setValue] = useState('');

  const handleChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    setValue(newValue);
    onSearch?.(newValue);
  }, [onSearch]);

  const handleClear = useCallback(() => {
    setValue('');
    onSearch?.('');
  }, [onSearch]);

  return (
    <div className="contact-search">
      <div className="search-input-wrapper">
        {/* 搜索图标 */}
        <svg className="search-icon" viewBox="0 0 24 24" fill="none">
          <circle cx="11" cy="11" r="8" stroke="currentColor" strokeWidth="2"/>
          <path d="M21 21L16.65 16.65" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
        </svg>

        {/* 输入框 */}
        <input
          type="text"
          className="search-input"
          placeholder={placeholder}
          value={value}
          onChange={handleChange}
        />

        {/* 清除按钮 */}
        {value && (
          <button
            className="search-clear"
            onClick={handleClear}
            aria-label="清除搜索"
          >
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
            </svg>
          </button>
        )}
      </div>
    </div>
  );
};

export default ContactSearch;
