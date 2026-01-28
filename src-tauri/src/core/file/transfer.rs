// src-tauri/src/core/file/transfer.rs
//
//! 文件分块传输逻辑

use crate::error::{AppError, AppResult};
use crate::network::feiq::model::{FeiqPacket, FileAttachment};
use crate::network::udp::sender;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use tokio::time::{timeout, Duration};

/// 文件传输配置
const CHUNK_SIZE: usize = 4 * 1024; // 4KB
const TRANSFER_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RETRIES: u32 = 3;

/// 文件传输进度
#[derive(Debug, Clone)]
pub struct FileTransferProgress {
    pub file_id: u64,
    pub offset: u64,
    pub total: u64,
    pub progress: u8, // 0-100
}

impl FileTransferProgress {
    pub fn new(file_id: u64, total: u64) -> Self {
        Self {
            file_id,
            offset: 0,
            total,
            progress: 0,
        }
    }

    pub fn update(&mut self, chunk_size: usize) {
        self.offset += chunk_size as u64;
        self.progress = ((self.offset as f64 / self.total as f64) * 100.0) as u8;
        self.progress = self.progress.min(100);
    }

    pub fn is_complete(&self) -> bool {
        self.offset >= self.total
    }
}

/// 文件发送器
pub struct FileSender {
    file_path: String,
    file_id: u64,
    target_addr: String,
    packet_no: String,
}

impl FileSender {
    pub fn new(file_path: String, file_id: u64, target_addr: String, packet_no: String) -> Self {
        Self {
            file_path,
            file_id,
            target_addr,
            packet_no,
        }
    }

    /// 发送文件（分块传输）
    pub async fn send(&self) -> AppResult<FileTransferProgress> {
        let path = Path::new(&self.file_path);
        let file_size = path
            .metadata()
            .map_err(|e| AppError::IoError(e.to_string()))?
            .len();

        let mut progress = FileTransferProgress::new(self.file_id, file_size);
        let mut file = File::open(path).map_err(|e| AppError::IoError(e.to_string()))?;

        loop {
            if progress.is_complete() {
                break;
            }

            let mut chunk = vec![0u8; CHUNK_SIZE];
            let n = file
                .read(&mut chunk)
                .map_err(|e| AppError::IoError(e.to_string()))?;

            if n == 0 {
                break;
            }

            chunk.truncate(n);

            // 发送数据块，带重试
            let mut retries = 0;
            loop {
                match self.send_chunk(&chunk, progress.offset).await {
                    Ok(_) => {
                        progress.update(n);
                        break;
                    }
                    Err(e) if retries < MAX_RETRIES => {
                        retries += 1;
                        tracing::warn!(
                            "Chunk send failed (attempt {}/{}): {}",
                            retries,
                            MAX_RETRIES,
                            e
                        );
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                    Err(e) => {
                        return Err(AppError::TransferError(format!(
                            "Failed to send chunk after {} retries: {}",
                            MAX_RETRIES, e
                        )));
                    }
                }
            }
        }

        Ok(progress)
    }

    /// 发送单个数据块
    async fn send_chunk(&self, chunk: &[u8], offset: u64) -> AppResult<()> {
        // 构建数据包: "packet_no:file_id:offset:data"
        let data_base64 = base64::encode(chunk);
        let extension = format!("{}:{}:{}:{}", self.packet_no, self.file_id, offset, data_base64);

        let packet = FeiqPacket {
            command: crate::network::feiq::constants::IPMSG_SENDMSG
                | crate::network::feiq::constants::IPMSG_UTF8OPT,
            extension: Some(extension),
            ..Default::default()
        };

        timeout(TRANSFER_TIMEOUT, sender::send_packet(&self.target_addr, &packet))
            .await
            .map_err(|_| AppError::TransferError("Transfer timeout".to_string()))?
            .map_err(|e| AppError::TransferError(e.to_string()))?;

        Ok(())
    }

    /// 计算文件 SHA256 校验和
    pub fn checksum(&self) -> AppResult<String> {
        let path = Path::new(&self.file_path);
        let mut file = File::open(path).map_err(|e| AppError::IoError(e.to_string()))?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];

        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|e| AppError::IoError(e.to_string()))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }
}

/// 文件接收器
pub struct FileReceiver {
    save_path: String,
    file_id: u64,
    expected_size: u64,
}

impl FileReceiver {
    pub fn new(save_path: String, file_id: u64, expected_size: u64) -> Self {
        Self {
            save_path,
            file_id,
            expected_size,
        }
    }

    /// 接收文件数据块
    pub fn receive_chunk(&mut self, offset: u64, data: &[u8]) -> AppResult<usize> {
        // 以读写模式打开文件，追加数据
        use std::io::Write;

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.save_path)
            .map_err(|e| AppError::IoError(e.to_string()))?;

        file
            .seek(io::SeekFrom::Start(offset))
            .map_err(|e| AppError::IoError(e.to_string()))?;

        file
            .write_all(data)
            .map_err(|e| AppError::IoError(e.to_string()))?;

        Ok(data.len())
    }

    /// 验证文件完整性
    pub fn verify(&self, expected_checksum: &str) -> AppResult<bool> {
        let mut file = File::open(&self.save_path).map_err(|e| AppError::IoError(e.to_string()))?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];

        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|e| AppError::IoError(e.to_string()))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let checksum = format!("{:x}", hasher.finalize());
        Ok(checksum == expected_checksum)
    }

    /// 获取当前文件大小
    pub fn current_size(&self) -> AppResult<u64> {
        let path = Path::new(&self.save_path);
        if path.exists() {
            Ok(path
                .metadata()
                .map_err(|e| AppError::IoError(e.to_string()))?
                .len())
        } else {
            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_progress() {
        let mut progress = FileTransferProgress::new(1, 1000);
        assert_eq!(progress.progress, 0);

        progress.update(500);
        assert_eq!(progress.progress, 50);
        assert_eq!(progress.offset, 500);

        progress.update(500);
        assert_eq!(progress.progress, 100);
        assert!(progress.is_complete());
    }
}
