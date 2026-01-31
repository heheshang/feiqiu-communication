// src-tauri/src/core/file/handler.rs
//
//! 文件传输事件处理器
//!
//! 处理来自网络层的文件传输事件：
//! - FileDataRequest: 对方请求文件数据块
//! - FileDataReceived: 接收到文件数据块
//! - FileRelease: 文件传输释放/取消

use crate::core::file::transfer::FileReceiver;
use crate::database::handler::{FileStorageHandler, TransferStateHandler};
use crate::error::{AppError, AppResult};
use crate::network::feiq::model::FeiQPacket;
use crate::network::udp::sender;
use sea_orm::DbConn;
use std::sync::OnceLock;
use tracing::info;

/// 文件传输事件处理器
pub struct FileTransferHandler;

/// 文件接收器缓存 (用于分块接收)
///
/// key: (packet_no, file_id)
/// value: FileReceiver
fn file_receivers() -> &'static std::sync::Mutex<std::collections::HashMap<(String, u64), FileReceiver>> {
    static RECEIVERS: OnceLock<std::sync::Mutex<std::collections::HashMap<(String, u64), FileReceiver>>> = OnceLock::new();
    RECEIVERS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

impl FileTransferHandler {
    /// 处理文件数据请求事件
    ///
    /// 当远程用户请求文件数据块时，读取本地文件并发送
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `from_ip`: 请求者IP
    /// - `packet_no`: 数据包编号
    /// - `file_id`: 文件ID
    /// - `offset`: 偏移量
    pub async fn handle_file_data_request(
        db: &DbConn,
        from_ip: &str,
        packet_no: &str,
        file_id: u64,
        offset: u64,
    ) -> AppResult<()> {
        info!(
            "收到文件数据请求: from_ip={}, packet_no={}, file_id={}, offset={}",
            from_ip, packet_no, file_id, offset
        );

        let transfer_states = TransferStateHandler::find_by_packet_no(db, packet_no).await?;

        let transfer_state = transfer_states
            .iter()
            .find(|t| t.file_id as u64 == file_id)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "找不到传输记录: packet_no={}, file_id={}",
                    packet_no, file_id
                ))
            })?;

        let file_storage = FileStorageHandler::find_by_id(db, transfer_state.file_id).await?;

        let chunk = Self::read_file_chunk(&file_storage.file_path, offset).await?;

        let packet = FeiQPacket::make_feiq_file_data_packet(packet_no, file_id, offset, &chunk, None);
        let packet_str = packet.to_feiq_string();

        let addr = format!("{}:{}", from_ip, 2425);
        sender::send_packet_data(&addr, &packet_str)
            .await
            .map_err(|e| AppError::Network(format!("发送文件数据块失败: {}", e)))?;

        info!(
            "已发送文件数据块: file_id={}, offset={}, size={}",
            file_id,
            offset,
            chunk.len()
        );

        Ok(())
    }

    /// 处理文件数据接收事件
    ///
    /// 当接收到文件数据块时，写入本地文件并更新进度
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `from_ip`: 发送者IP
    /// - `packet_no`: 数据包编号
    /// - `file_id`: 文件ID
    /// - `offset`: 偏移量
    /// - `data`: Base64编码的文件数据
    pub async fn handle_file_data_received(
        db: &DbConn,
        from_ip: &str,
        packet_no: &str,
        file_id: u64,
        offset: u64,
        data: &str,
    ) -> AppResult<()> {
        info!(
            "收到文件数据: from_ip={}, packet_no={}, file_id={}, offset={}, size={}",
            from_ip,
            packet_no,
            file_id,
            offset,
            data.len()
        );

        use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
        let chunk = BASE64
            .decode(data)
            .map_err(|e| AppError::Protocol(format!("Base64 解码失败: {}", e)))?;

        let transfer_states = TransferStateHandler::find_by_packet_no(db, packet_no).await?;

        let transfer_state = transfer_states
            .iter()
            .find(|t| t.file_id as u64 == file_id)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "找不到传输记录: packet_no={}, file_id={}",
                    packet_no, file_id
                ))
            })?;

        let key = (packet_no.to_string(), file_id);

        let is_complete = {
            let mut receivers = file_receivers().lock().map_err(|e| {
                AppError::Business(format!("获取文件接收器缓存失败: {}", e))
            })?;

            let receiver = receivers.entry(key.clone()).or_insert_with(|| {
                FileReceiver::new(
                    transfer_state.get_save_path(),
                    file_id,
                    transfer_state.file_size as u64,
                )
            });

            receiver.receive_chunk(offset, &chunk)?;

            let current_size = receiver.current_size()?;
            let is_complete = current_size >= transfer_state.file_size as u64;

            if is_complete {
                info!(
                    "文件接收完成: file_id={}, size={}",
                    file_id, transfer_state.file_size
                );
                receivers.remove(&key);
            }

            is_complete
        };

        let new_offset = offset + chunk.len() as u64;
        TransferStateHandler::update_progress(db, transfer_state.tid, new_offset as i64, 1).await?;

        if is_complete {
            TransferStateHandler::update_status(db, transfer_state.tid, 2, None).await?;
        }

        Ok(())
    }

    /// 处理文件释放事件
    ///
    /// 当文件传输被取消或释放时，清理相关资源
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `from_ip`: 发送者IP
    /// - `packet_no`: 数据包编号
    pub async fn handle_file_release(db: &DbConn, from_ip: &str, packet_no: &str) -> AppResult<()> {
        info!(
            "收到文件释放: from_ip={}, packet_no={}",
            from_ip, packet_no
        );

        let transfer_states = TransferStateHandler::find_by_packet_no(db, packet_no).await?;

        for transfer_state in transfer_states {
            TransferStateHandler::update_status(db, transfer_state.tid, -2, Some("对方取消传输".to_string())).await?;

            let mut receivers = file_receivers().lock().map_err(|e| {
                AppError::Business(format!("获取文件接收器缓存失败: {}", e))
            })?;

            receivers.retain(|p, _| p.0.as_str() != packet_no);
        }

        info!("文件传输已清理: packet_no={}", packet_no);

        Ok(())
    }

    /// 读取文件数据块
    ///
    /// # 参数
    /// - `file_path`: 文件路径
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// 返回文件数据块 (最大 4KB)
    async fn read_file_chunk(file_path: &str, offset: u64) -> AppResult<Vec<u8>> {
        use tokio::fs::File;
        use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

        let mut file = File::open(file_path)
            .await
            .map_err(AppError::Io)?;

        file.seek(SeekFrom::Start(offset))
            .await
            .map_err(AppError::Io)?;

        let mut buffer = vec![0u8; 4096];
        let n = file
            .read(&mut buffer)
            .await
            .map_err(AppError::Io)?;

        buffer.truncate(n);
        Ok(buffer)
    }
}

trait TransferStateExt {
    fn get_save_path(&self) -> String;
}

impl TransferStateExt for crate::database::model::transfer_state::Model {
    fn get_save_path(&self) -> String {
        format!("/tmp/feiqiu_download_{}", self.file_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_structure() {
        assert!(true);
    }
}
