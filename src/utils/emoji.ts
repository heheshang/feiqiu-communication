// 工具函数 - Emoji 相关

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
} as const;

export type EmojiCategory = keyof typeof EMOJI_CATEGORIES;

/** 扩展的 Emoji 列表，按分类组织 */
export const EMOJIS_BY_CATEGORY: Record<EmojiCategory, string[]> = {
  smileys: [
    '😀',
    '😃',
    '😄',
    '😁',
    '😆',
    '😅',
    '😂',
    '🤣',
    '😊',
    '😇',
    '🙂',
    '🙃',
    '😉',
    '😌',
    '😍',
    '🥰',
    '😘',
    '😗',
    '😙',
    '😚',
    '😋',
    '😛',
    '😝',
    '😜',
    '🤪',
    '🤨',
    '🧐',
    '🤓',
    '😎',
    '🥸',
    '🤩',
    '🥳',
    '😏',
    '😒',
    '😞',
    '😔',
    '😟',
    '😕',
    '🙁',
    '😣',
    '😖',
    '😫',
    '😩',
    '🥺',
    '😢',
    '😭',
    '😤',
    '😠',
    '😡',
    '🤬',
    '🤯',
    '😳',
    '🥵',
    '🥶',
    '😱',
    '😨',
  ],
  people: [
    '👋',
    '🤚',
    '🖐️',
    '✋',
    '🖖',
    '👌',
    '🤌',
    '🤏',
    '✌️',
    '🤞',
    '🤝',
    '🙏',
    '👍',
    '👎',
    '👊',
    '✊',
    '🤛',
    '🤜',
    '🤟',
    '👆',
    '👇',
    '👉',
    '👈',
    '🖕',
    '🙏',
    '💪',
    '🦵',
    '🦶',
    '👂',
    '🦻',
    '👃',
    '🧠',
    '🦷',
    '🦴',
    '👀',
    '👁️',
    '👅',
    '👄',
    '💋',
    '🩸',
  ],
  animals: [
    '🐶',
    '🐱',
    '🐭',
    '🐹',
    '🐰',
    '🦊',
    '🐻',
    '🐼',
    '🐨',
    '🐯',
    '🦁',
    '🐮',
    '🐷',
    '🐸',
    '🐵',
    '🐔',
    '🐧',
    '🐦',
    '🐤',
    '🦆',
    '🦅',
    '🦉',
    '🦇',
    '🐺',
    '🐗',
    '🐴',
    '🦄',
    '🐝',
    '🐛',
    '🦋',
    '🐌',
    '🐞',
    '🐜',
    '🦟',
    '🦗',
    '🕷️',
    '🦂',
    '🐢',
    '🐍',
    '🦎',
  ],
  food: [
    '🍎',
    '🍏',
    '🍊',
    '🍋',
    '🍌',
    '🍉',
    '🍇',
    '🍓',
    '🫐',
    '🍈',
    '🍒',
    '🍑',
    '🥭',
    '🍍',
    '🥥',
    '🥝',
    '🍅',
    '🍆',
    '🥑',
    '🥦',
    '🥬',
    '🥒',
    '🌶️',
    '🫑',
    '🌽',
    '🥕',
    '🫒',
    '🧄',
    '🧅',
    '🥔',
    '🍠',
    '🥐',
    '🥯',
    '🍞',
    '🥖',
    '🥨',
    '🧀',
    '🥚',
    '🍳',
    '🧈',
  ],
  activities: [
    '⚽',
    '🏀',
    '🏈',
    '⚾',
    '🥎',
    '🎾',
    '🏐',
    '🏉',
    '🥏',
    '🎱',
    '🪀',
    '🏓',
    '🏸',
    '🏒',
    '🏑',
    '🥍',
    '🏏',
    '🥅',
    '⛳',
    '🪁',
    '🏹',
    '🎣',
    '🤿',
    '🥊',
    '🥋',
    '🎽',
    '🛹',
    '🛼',
    '🛷',
    '⛸️',
    '🥌',
    '🎿',
    '⛷️',
    '🏂',
    '🪂',
    '🏋️',
    '🤼',
    '🤸',
    '🤺',
    '⛹️',
  ],
  travel: [
    '🚗',
    '🚕',
    '🚙',
    '🚌',
    '🚎',
    '🏎️',
    '🚓',
    '🚑',
    '🚒',
    '🚐',
    '🛻',
    '🚚',
    '🚛',
    '🚜',
    '🦯',
    '🦽',
    '🦼',
    '🛴',
    '🚲',
    '🛵',
    '🏍️',
    '🛺',
    '🚨',
    '🚔',
    '🚍',
    '🚘',
    '🚖',
    '🚡',
    '🚠',
    '🚟',
    '🚃',
    '🚋',
    '🚞',
    '🚝',
    '🚄',
    '🚅',
    '🚈',
    '🚂',
    '🚆',
    '🚇',
  ],
  objects: [
    '⌚',
    '📱',
    '📲',
    '💻',
    '⌨️',
    '🖥️',
    '🖨️',
    '🖱️',
    '🖲️',
    '🕹️',
    '🗜️',
    '💾',
    '💿',
    '📀',
    '📼',
    '📷',
    '📸',
    '📹',
    '🎥',
    '📽️',
    '🎞️',
    '📞',
    '☎️',
    '📟',
    '📠',
    '📺',
    '📻',
    '🎙️',
    '🎚️',
    '🎛️',
    '🧭',
    '⏱️',
    '⏲️',
    '⏰',
    '🕰',
    '⌛',
    '⏳',
    '📡',
    '🔋',
    '🔌',
  ],
  symbols: [
    '💰',
    '💴',
    '💵',
    '💶',
    '💷',
    '💸',
    '💹',
    '💲',
    '💱',
    '™️',
    '©️',
    '®️',
    '〰️',
    '️#️⃣',
    '*️⃣',
    '0️⃣',
    '1️⃣',
    '2️⃣',
    '3️⃣',
    '4️⃣',
    '5️⃣',
    '6️⃣',
    '7️⃣',
    '8️⃣',
    '9️⃣',
    '🔟',
    '🔠',
    '🔡',
    '🔢',
    '🔣',
    '🔤',
    '🅰️',
    '🆎',
    '🆑',
    '🅾️',
    '🆘',
    '❌',
    '⭕',
    '🛑',
    '⛔',
  ],
};

/** 常用 Emoji 列表（保留向后兼容） */
export const COMMON_EMOJIS = EMOJIS_BY_CATEGORY.smileys.slice(0, 24);

/** 最近使用的 Emoji（从 localStorage 读取） */
export function getRecentEmojis(): string[] {
  try {
    const stored = localStorage.getItem('recent_emojis');
    if (stored) {
      return JSON.parse(stored);
    }
  } catch (e) {
    console.error('Failed to get recent emojis:', e);
  }
  return [];
}

/** 保存最近使用的 Emoji */
export function saveRecentEmoji(emoji: string): void {
  try {
    const recent = getRecentEmojis();
    // 移除重复的
    const filtered = recent.filter((e) => e !== emoji);
    // 添加到前面
    const updated = [emoji, ...filtered].slice(0, 20); // 最多保存 20 个
    localStorage.setItem('recent_emojis', JSON.stringify(updated));
  } catch (e) {
    console.error('Failed to save recent emoji:', e);
  }
}

/** 搜索 Emoji */
export function searchEmojis(query: string): string[] {
  if (!query.trim()) {
    return [];
  }

  const allEmojis = Object.values(EMOJIS_BY_CATEGORY).flat();
  // 简单返回所有匹配的 emoji（可以根据需要添加更复杂的搜索逻辑）
  return allEmojis;
}

/** 检测文本中的 Emoji */
export function hasEmoji(text: string): boolean {
  return /[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/u.test(
    text
  );
}

/** Emoji 短代码映射表 */
const EMOJI_SHORTCODE_MAP: Record<string, string> = {
  // 表情
  smile: '😀',
  laughing: '😂',
  wink: '😉',
  heart: '❤️',
  kiss: '😘',
  cry: '😢',
  angry: '😠',
  sad: '😞',
  thumbsup: '👍',
  thumbsdown: '👎',
  ok: '👌',
  victory: '✌️',
  clap: '👏',
  wave: '👋',
  muscle: '💪',

  // 动物
  dog: '🐶',
  cat: '🐱',
  mouse: '🐭',
  rabbit: '🐰',
  bear: '🐻',
  panda: '🐼',
  fox: '🦊',
  lion: '🦁',
  pig: '🐷',

  // 食物
  apple: '🍎',
  banana: '🍌',
  cherry: '🍒',
  grape: '🍇',
  watermelon: '🍉',
  peach: '🍑',
  pineapple: '🍍',

  // 活动
  soccer: '⚽',
  basketball: '🏀',
  football: '🏈',
  baseball: '⚾',
  tennis: '🎾',
  golf: '⛳',

  // 符号
  check: '✅',
  cross: '❌',
  star: '⭐',
  fire: '🔥',
  lightning: '⚡',
  moon: '🌙',
  sun: '☀️',
};

/** 转换 Emoji 短代码到实际 Emoji */
export function convertEmojiShortcode(text: string): string {
  // 匹配 :shortcode: 格式
  return text.replace(/:([a-z_]+):/gi, (match, shortcode) => {
    return EMOJI_SHORTCODE_MAP[shortcode] || match;
  });
}

/** 转换实际 Emoji 到短代码 */
export function convertEmojiToShortcode(text: string): string {
  let result = text;

  // 反向映射
  for (const [shortcode, emoji] of Object.entries(EMOJI_SHORTCODE_MAP)) {
    const regex = new RegExp(emoji.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g');
    result = result.replace(regex, `:${shortcode}:`);
  }

  return result;
}

/** 获取 Emoji 的短代码 */
export function getEmojiShortcode(emoji: string): string | undefined {
  for (const [shortcode, value] of Object.entries(EMOJI_SHORTCODE_MAP)) {
    if (value === emoji) {
      return shortcode;
    }
  }
  return undefined;
}
