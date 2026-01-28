// 组件 - Emoji 选择器

import React, { useState, useMemo, useEffect } from 'react';
import {
  EMOJI_CATEGORIES,
  EMOJIS_BY_CATEGORY,
  getRecentEmojis,
  saveRecentEmoji,
  type EmojiCategory,
} from '../../utils/emoji';

import './EmojiPicker.less';

interface EmojiPickerProps {
  onEmojiSelect: (emoji: string) => void;
  onClose?: () => void;
}

export const EmojiPicker: React.FC<EmojiPickerProps> = ({ onEmojiSelect, onClose }) => {
  const [activeCategory, setActiveCategory] = useState<EmojiCategory>('smileys');
  const [searchQuery, setSearchQuery] = useState('');
  const [recentEmojis, setRecentEmojis] = useState<string[]>([]);

  useEffect(() => {
    setRecentEmojis(getRecentEmojis());
  }, []);

  const handleEmojiClick = (emoji: string) => {
    onEmojiSelect(emoji);
    saveRecentEmoji(emoji);
    // 重新加载最近使用的 emoji
    setRecentEmojis(getRecentEmojis());
    onClose?.();
  };

  const categories = useMemo(() => {
    return Object.keys(EMOJI_CATEGORIES) as EmojiCategory[];
  }, []);

  const displayedEmojis = useMemo(() => {
    if (searchQuery.trim()) {
      // 搜索所有分类的 emoji
      const allEmojis = Object.values(EMOJIS_BY_CATEGORY).flat();
      return allEmojis;
    }
    return EMOJIS_BY_CATEGORY[activeCategory];
  }, [activeCategory, searchQuery]);

  return (
    <div className="emoji-picker">
      <div className="emoji-picker-header">
        <span>Emoji</span>
        <button className="close-btn" onClick={onClose} title="关闭">
          ×
        </button>
      </div>

      {/* 搜索框 */}
      <div className="emoji-picker-search">
        <input
          type="text"
          placeholder="搜索 emoji..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="emoji-search-input"
        />
      </div>

      {/* 分类标签 */}
      {!searchQuery && (
        <div className="emoji-picker-categories">
          {categories.map((category) => (
            <button
              key={category}
              className={`category-btn ${activeCategory === category ? 'active' : ''}`}
              onClick={() => setActiveCategory(category)}
              title={EMOJI_CATEGORIES[category]}
            >
              {EMOJI_CATEGORIES[category]}
            </button>
          ))}
        </div>
      )}

      {/* 最近使用 */}
      {!searchQuery && recentEmojis.length > 0 && activeCategory === 'smileys' && (
        <div className="emoji-picker-section">
          <div className="emoji-section-title">最近使用</div>
          <div className="emoji-grid">
            {recentEmojis.map((emoji) => (
              <div
                key={`recent-${emoji}`}
                className="emoji-item"
                onClick={() => handleEmojiClick(emoji)}
              >
                {emoji}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Emoji 列表 */}
      <div className="emoji-picker-body">
        <div className="emoji-grid">
          {displayedEmojis.map((emoji) => (
            <div
              key={emoji}
              className="emoji-item"
              onClick={() => handleEmojiClick(emoji)}
              title={emoji}
            >
              {emoji}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
