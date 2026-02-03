import { contactAPI } from '../ipc/contact';

export const contactService = {
  async getContactList(ownerUid: number) {
    return await contactAPI.getContactList(ownerUid);
  },

  async getOnlineUsers() {
    return await contactAPI.getOnlineUsers();
  },
};
