import { chatAPI } from '../ipc/chat';

export const chatService = {
  async getHistory(sessionType: number, targetId: number, page: number) {
    return await chatAPI.getHistory(sessionType, targetId, page);
  },

  async sendMessage(sessionType: number, targetId: number, content: string, ownerUid: number) {
    return await chatAPI.sendMessage(sessionType, targetId, content, ownerUid);
  },

  async getSessionList(ownerUid: number) {
    return await chatAPI.getSessionList(ownerUid);
  },

  async markMessagesRead(sessionType: number, targetId: number, ownerUid: number) {
    return await chatAPI.markMessagesRead(sessionType, targetId, ownerUid);
  },

  async markMessageReadAndSendReceipt(mid: number, msgNo: string, targetIp: string) {
    return await chatAPI.markMessageReadAndSendReceipt(mid, msgNo, targetIp);
  },

  async retrySendMessage(mid: number, sessionType: number, targetId: number, ownerUid: number) {
    return await chatAPI.retrySendMessage(mid, sessionType, targetId, ownerUid);
  },
};
