// 工具函数 - Emoji 相关
// TODO: Phase 5 时完善 Emoji 支持

/** 常用 Emoji 列表 */
export const COMMON_EMOJIS = [
  '😀', '😃', '😄', '😁', '😆', '😅', '😂', '🤣',
  '😊', '😇', '🙂', '🙃', '😉', '😌', '😍', '🥰',
  '😘', '😗', '😙', '😚', '😋', '😛', '😝', '😜',
  '🤪', '🤨', '🧐', '🤓', '😎', '🥸', '🤩', '🥳',
  '👍', '👎', '👌', '✌️', '🤞', '🤝', '🙏', '❤️',
];

/** Emoji 分类 */
export const EMOJI_CATEGORIES = {
  smileys: '表情',
  people: '人物',
  animals: '动物',
  food: '食物',
  activities: '活动',
  travel: '旅行',
  objects: '物品',
  symbols: '符号',
};

/** 检测文本中的 Emoji */
export function hasEmoji(text: string): boolean {
  return /[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/u.test(
    text
  );
}

/** 转换 Emoji 短代码 */
export function convertEmojiShortcode(text: string): string {
  // TODO: 实现 Emoji 短代码到实际 Emoji 的转换
  return text;
}
