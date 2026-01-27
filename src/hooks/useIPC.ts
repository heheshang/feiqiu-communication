// 自定义钩子 - IPC 通信
// TODO: Phase 4 时完善 IPC 通信钩子

import { invoke } from '@tauri-apps/api/tauri';

/** IPC 调用错误处理 */
export function useIPC() {
  const handleError = (error: unknown, context: string) => {
    console.error(`[IPC Error] ${context}:`, error);
    throw error;
  };

  return {
    invoke: async <T>(command: string, args?: Record<string, unknown>): Promise<T> => {
      try {
        return await invoke<T>(command, args);
      } catch (error) {
        handleError(error, command);
        throw error;
      }
    },
  };
}
