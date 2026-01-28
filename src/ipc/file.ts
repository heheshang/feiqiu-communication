// IPC 封装 - 文件传输相关

import { invoke } from '@tauri-apps/api/core';

export const fileAPI = {
  /** 发送文件请求 */
  sendFileRequest: async (filePaths: string[], targetIp: string, ownerUid: number) => {
    return await invoke<number>('send_file_request_handler', {
      filePaths,
      targetIp,
      ownerUid,
    });
  },

  /** 接受文件请求 */
  acceptFileRequest: async (packetNo: string, fileId: number, offset: number, targetIp: string) => {
    return await invoke<void>('accept_file_request_handler', {
      packetNo,
      fileId,
      offset,
      targetIp,
    });
  },

  /** 拒绝文件请求 */
  rejectFileRequest: async (packetNo: string, targetIp: string) => {
    return await invoke<void>('reject_file_request_handler', {
      packetNo,
      targetIp,
    });
  },

  /** 获取文件 */
  getFile: async (fid: number) => {
    return await invoke<any>('get_file_handler', { fid });
  },

  /** 取消上传 */
  cancelUpload: async (fid: number) => {
    return await invoke<void>('cancel_upload_handler', { fid });
  },

  /** 获取待恢复的传输列表 */
  getPendingTransfers: async () => {
    return await invoke<any[]>('get_pending_transfers_handler');
  },

  /** 恢复传输 */
  resumeTransfer: async (tid: number) => {
    return await invoke<void>('resume_transfer_handler', { tid });
  },
};
