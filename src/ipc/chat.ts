// IPC 封装 - 聊天相关
// TODO: Phase 4 时完善聊天 IPC 接口

import { invoke } from '@tauri-apps/api/core';
import type { ChatMessage, ChatSession } from '../types';

export const chatAPI = {
  /**
   * 获取历史消息
   *
   * 从数据库中分页获取指定会话的历史消息。返回 Promise，调用者应该使用 .catch() 处理错误。
   *
   * @param sessionType - 会话类型（0: 单聊, 1: 群聊）
   * @param targetId - 目标用户/群组 ID
   * @param page - 页码（从 1 开始）
   * @returns Promise<ChatMessage[]> - 历史消息列表
   *
   * @throws 会抛出错误字符串，调用者应该使用 .catch() 处理
   *
   * @example
   * ```typescript
   * import { chatAPI } from '@/ipc/chat';
   * import { parseError, showError, ErrorCode } from '@/utils/error';
   *
   * // 简单错误处理 - 显示通用错误提示
   * chatAPI.getHistory(0, 123, 1)
   *   .then(messages => {
   *     console.log('获取消息成功:', messages);
   *     displayMessages(messages);
   *   })
   *   .catch((e: string) => {
   *     const error = parseError(e);
   *     showError(error);
   *   });
   *
   * // 条件错误处理 - 根据错误类型显示不同提示
   * chatAPI.getHistory(0, 123, 1)
   *   .then(messages => displayMessages(messages))
   *   .catch((e: string) => {
   *     const error = parseError(e);
   *     if (error.code === ErrorCode.Database) {
   *       showError({
   *         code: error.code,
   *         message: '数据库错误，无法加载消息历史',
   *         details: error.details,
   *       });
   *     } else if (error.code === ErrorCode.NotFound) {
   *       showError({
   *         code: error.code,
   *         message: '会话不存在',
   *         details: error.details,
   *       });
   *     } else {
   *       showError(error);
   *     }
   *   });
   * ```
   */
  getHistory: async (sessionType: number, targetId: number, page: number) => {
    return await invoke<ChatMessage[]>('get_chat_history_handler', {
      sessionType,
      targetId,
      page,
      pageSize: 50,
    });
  },

  /**
   * 发送消息
   *
   * 发送文本消息到指定的用户或群组。返回新创建的消息 ID。
   *
   * @param sessionType - 会话类型（0: 单聊, 1: 群聊）
   * @param targetId - 目标用户/群组 ID
   * @param content - 消息内容
   * @param ownerUid - 发送者 UID
   * @returns Promise<number> - 新消息的 ID
   *
   * @throws 会抛出错误字符串，调用者应该使用 .catch() 处理
   *
   * @example
   * ```typescript
   * import { chatAPI } from '@/ipc/chat';
   * import { parseError, showError } from '@/utils/error';
   *
   * // 简单错误处理
   * chatAPI.sendMessage(0, 123, 'Hello', 456)
   *   .then(messageId => {
   *     console.log('消息已发送，ID:', messageId);
   *     addMessageToUI(messageId);
   *   })
   *   .catch((e: string) => {
   *     const error = parseError(e);
   *     showError(error);
   *     // 可选：标记消息为发送失败状态
   *     markMessageAsFailed();
   *   });
   * ```
   */
  sendMessage: async (sessionType: number, targetId: number, content: string, ownerUid: number) => {
    return await invoke<number>('send_text_message_handler', {
      sessionType,
      targetId,
      content,
      ownerUid,
    });
  },

  /**
   * 获取会话列表
   *
   * 获取指定用户的所有聊天会话列表。包括单聊和群聊会话。
   *
   * @param ownerUid - 用户 UID
   * @returns Promise<ChatSession[]> - 会话列表
   *
   * @throws 会抛出错误字符串，调用者应该使用 .catch() 处理
   *
   * @example
   * ```typescript
   * import { chatAPI } from '@/ipc/chat';
   * import { parseError, showError, ErrorCode, isErrorCode } from '@/utils/error';
   *
   * // 条件错误处理 - 根据错误类型采取不同行动
   * chatAPI.getSessionList(456)
   *   .then(sessions => {
   *     console.log('会话列表:', sessions);
   *     renderSessionList(sessions);
   *   })
   *   .catch((e: string) => {
   *     const error = parseError(e);
   *
   *     // 数据库错误 - 可能需要重试或离线模式
   *     if (isErrorCode(error, ErrorCode.Database)) {
   *       showError({
   *         code: error.code,
   *         message: '无法加载会话列表，请检查数据库连接',
   *         details: error.details,
   *       });
   *       // 可选：启用离线模式或显示缓存数据
   *       enableOfflineMode();
   *     }
   *     // 权限错误 - 用户无权访问
   *     else if (isErrorCode(error, ErrorCode.Permission)) {
   *       showError({
   *         code: error.code,
   *         message: '权限不足，无法访问会话列表',
   *         details: error.details,
   *       });
   *     }
   *     // 其他错误
   *     else {
   *       showError(error);
   *     }
   *   });
   * ```
   */
  getSessionList: async (ownerUid: number) => {
    return await invoke<ChatSession[]>('get_session_list_handler', { ownerUid });
  },

  /**
   * 标记消息已读
   *
   * 将指定会话中的所有消息标记为已读。
   *
   * @param sessionType - 会话类型（0: 单聊, 1: 群聊）
   * @param targetId - 目标用户/群组 ID
   * @param ownerUid - 用户 UID
   * @returns Promise<void>
   *
   * @throws 会抛出错误字符串，调用者应该使用 .catch() 处理
   *
   * @example
   * ```typescript
   * import { chatAPI } from '@/ipc/chat';
   * import { parseError, showError } from '@/utils/error';
   *
   * chatAPI.markMessagesRead(0, 123, 456)
   *   .catch((e: string) => {
   *     const error = parseError(e);
   *     console.warn('标记已读失败:', error.message);
   *     // 标记已读失败通常不需要显示给用户，只记录日志
   *   });
   * ```
   */
  markMessagesRead: async (sessionType: number, targetId: number, ownerUid: number) => {
    return await invoke<void>('mark_messages_read_handler', {
      sessionType,
      targetId,
      ownerUid,
    });
  },

  /**
   * 标记单条消息已读并发送已读回执
   *
   * 标记指定消息为已读，并向发送者发送已读回执。
   *
   * @param mid - 消息 ID
   * @param msgNo - 消息编号（来自协议）
   * @param targetIp - 目标 IP 地址（用于发送回执）
   * @returns Promise<void>
   *
   * @throws 会抛出错误字符串，调用者应该使用 .catch() 处理
   *
   * @example
   * ```typescript
   * import { chatAPI } from '@/ipc/chat';
   * import { parseError } from '@/utils/error';
   *
   * chatAPI.markMessageReadAndSendReceipt(789, '12345', '192.168.1.100')
   *   .catch((e: string) => {
   *     const error = parseError(e);
   *     console.warn('发送已读回执失败:', error.message);
   *     // 回执失败不影响本地已读状态，只记录日志
   *   });
   * ```
   */
  markMessageReadAndSendReceipt: async (mid: number, msgNo: string, targetIp: string) => {
    return await invoke<void>('mark_message_read_and_send_receipt', {
      mid,
      msgNo,
      targetIp,
    });
  },

  /**
   * 重试发送失败的消息
   *
   * 重新发送之前发送失败的消息。
   *
   * @param mid - 消息 ID
   * @param sessionType - 会话类型（0: 单聊, 1: 群聊）
   * @param targetId - 目标用户/群组 ID
   * @param ownerUid - 发送者 UID
   * @returns Promise<void>
   *
   * @throws 会抛出错误字符串，调用者应该使用 .catch() 处理
   *
   * @example
   * ```typescript
   * import { chatAPI } from '@/ipc/chat';
   * import { parseError, showError } from '@/utils/error';
   *
   * chatAPI.retrySendMessage(789, 0, 123, 456)
   *   .then(() => {
   *     console.log('消息重试发送成功');
   *     updateMessageStatus(789, 'sent');
   *   })
   *   .catch((e: string) => {
   *     const error = parseError(e);
   *     showError({
   *       code: error.code,
   *       message: '重试发送失败，请稍后再试',
   *       details: error.details,
   *     });
   *   });
   * ```
   */
  retrySendMessage: async (
    mid: number,
    sessionType: number,
    targetId: number,
    ownerUid: number
  ) => {
    return await invoke<void>('retry_send_message', {
      mid,
      sessionType,
      targetId,
      ownerUid,
    });
  },
};
