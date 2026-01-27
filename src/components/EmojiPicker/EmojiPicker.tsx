// 组件 - Emoji 选择器
// TODO: Phase 5 时完善 Emoji 选择器组件

import React from 'react';
import { COMMON_EMOJIS } from '../../utils/emoji';

import './EmojiPicker.less';

interface EmojiPickerProps {
  onEmojiSelect: (emoji: string) => void;
  onClose?: () => void;
}

export const EmojiPicker: React.FC<EmojiPickerProps> = ({ onEmojiSelect, onClose }) => {
  const handleEmojiClick = (emoji: string) => {
    onEmojiSelect(emoji);
    onClose?.();
  };

  return (
    <div className="emoji-picker">
      <div className="emoji-picker-header">
        <span>Emoji</span>
        <button className="close-btn" onClick={onClose}>
          ×
        </button>
      </div>
      <div className="emoji-picker-body">
        {COMMON_EMOJIS.map((emoji) => (
          <div
            key={emoji}
            className="emoji-item"
            onClick={() => handleEmojiClick(emoji)}
          >
            {emoji}
          </div>
        ))}
      </div>
    </div>
  );
};
