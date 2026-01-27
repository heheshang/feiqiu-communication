// IPC 封装 - 文件传输相关
// TODO: Phase 4 时完善文件传输 IPC 接口

import { invoke } from '@tauri-apps/api/tauri';

export const fileAPI = {
  /** 上传文件 */
  uploadFile: async (filePath: string, sessionType: number, targetId: number) => {
    return await invoke<number>('upload_file_handler', {
      filePath,
      sessionType,
      targetId,
    });
  },

  /** 获取文件 */
  getFile: async (fid: number) => {
    return await invoke<string>('get_file_handler', { fid });
  },

  /** 取消上传 */
  cancelUpload: async (fid: number) => {
    return await invoke<void>('cancel_upload_handler', { fid });
  },
};
