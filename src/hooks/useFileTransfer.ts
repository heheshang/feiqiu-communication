// Hook - 文件传输
// Phase 6: 文件传输功能

import { useState, useCallback } from 'react';
import { fileAPI } from '../ipc/file';
import type { TransferProgress, TransferStatus } from '../types';

interface FileTransfer {
  fileId: number;
  fileName: string;
  progress: TransferProgress;
  status: TransferStatus;
}

export function useFileTransfer() {
  const [transfers, setTransfers] = useState<Map<number, FileTransfer>>(new Map());

  // 发送文件请求
  const sendFile = useCallback(async (filePaths: string[], targetIp: string, ownerUid: number) => {
    try {
      const fid = await fileAPI.sendFileRequest(filePaths, targetIp, ownerUid);

      // 创建初始传输记录
      const transfer: FileTransfer = {
        fileId: fid,
        fileName: filePaths[0].split(/[/\\]/).pop() || 'Unknown',
        progress: {
          file_id: fid,
          progress: 0,
          total: 0,
          speed: 0,
          transferred: 0,
        },
        status: 0 as TransferStatus, // Pending
      };

      setTransfers((prev) => new Map(prev).set(fid, transfer));
      return fid;
    } catch (error) {
      console.error('Failed to send file request:', error);
      throw error;
    }
  }, []);

  // 接受文件请求
  const acceptFile = useCallback(
    async (packetNo: string, fileId: number, offset: number, targetIp: string) => {
      try {
        await fileAPI.acceptFileRequest(packetNo, fileId, offset, targetIp);

        // 更新状态为传输中
        setTransfers((prev) => {
          const transfer = prev.get(fileId);
          if (transfer) {
            const updated = { ...transfer, status: 1 as TransferStatus };
            return new Map(prev).set(fileId, updated);
          }
          return prev;
        });
      } catch (error) {
        console.error('Failed to accept file:', error);
        throw error;
      }
    },
    []
  );

  // 拒绝文件请求
  const rejectFile = useCallback(async (packetNo: string, targetIp: string) => {
    try {
      await fileAPI.rejectFileRequest(packetNo, targetIp);
    } catch (error) {
      console.error('Failed to reject file:', error);
      throw error;
    }
  }, []);

  // 取消传输
  const cancelTransfer = useCallback(async (fileId: number) => {
    try {
      await fileAPI.cancelUpload(fileId);

      // 更新状态为已取消
      setTransfers((prev) => {
        const transfer = prev.get(fileId);
        if (transfer) {
          const updated = { ...transfer, status: 3 as TransferStatus };
          return new Map(prev).set(fileId, updated);
        }
        return prev;
      });
    } catch (error) {
      console.error('Failed to cancel transfer:', error);
      throw error;
    }
  }, []);

  // 更新传输进度
  const updateProgress = useCallback((fileId: number, progress: Partial<TransferProgress>) => {
    setTransfers((prev) => {
      const transfer = prev.get(fileId);
      if (transfer) {
        const updated = {
          ...transfer,
          progress: { ...transfer.progress, ...progress },
        };
        return new Map(prev).set(fileId, updated);
      }
      return prev;
    });
  }, []);

  // 获取传输列表
  const getTransfers = useCallback(() => {
    return Array.from(transfers.values());
  }, [transfers]);

  // 获取单个传输状态
  const getTransfer = useCallback(
    (fileId: number) => {
      return transfers.get(fileId);
    },
    [transfers]
  );

  return {
    transfers: getTransfers(),
    sendFile,
    acceptFile,
    rejectFile,
    cancelTransfer,
    updateProgress,
    getTransfer,
  };
}
