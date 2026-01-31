// src-tauri/src/core/file/request.rs
//
//! 文件请求处理逻辑

use crate::error::{AppError, AppResult};
use crate::network::feiq::{
    constants::*,
    model::{FeiQPacket, FileAttachment},
};

/// 处理接收到的文件附件请求
///
/// 当收到 IPMSG_SENDMSG | IPMSG_FILEATTACHOPT 消息时调用
pub fn handle_file_attach_request(packet: &FeiQPacket) -> AppResult<Vec<FileAttachment>> {
    // TODO: FeiQ file transfer not implemented yet
    let _ = packet;
    Err(AppError::Protocol("FeiQ file transfer not implemented yet".to_string()))
    // 检查是否带文件附件
    // if !packet.has_option(IPMSG_FILEATTACHOPT) {
    //     return Err(AppError::Protocol("Not a file attachment packet".to_string()));
    // }

    // // 解析文件附件头
    // let extension = packet
    //     .extension
    //     .as_ref()
    //     .ok_or_else(|| AppError::Protocol("Missing file attachment extension".to_string()))?;

    // let files = FileAttachment::from_ipmsg_header(extension)
    //     .map_err(|e| AppError::Protocol(format!("Failed to parse file attachment: {}", e)))?;

    // Ok(files)
}

/// 创建文件附件请求包
///
/// 用于发送文件给其他用户
pub fn create_file_attach_request(files: &[FileAttachment], receiver_ip: &str, receiver_port: u16) -> FeiQPacket {
    // TODO: FeiQ file transfer not implemented yet
    // let receiver = format!("{}:{}", receiver_ip, receiver_port);
    // ProtocolPacket::make_file_attach_packet(files, &receiver)
    let _ = (files, receiver_ip, receiver_port);
    FeiQPacket::make_feiq_entry_packet(None)
}

/// 创建文件数据请求包
///
/// 用于接收方请求文件数据
pub fn create_file_data_request(packet_no: &str, file_id: u64, offset: u64) -> FeiQPacket {
    // TODO: FeiQ file transfer not implemented yet
    // ProtocolPacket::make_get_file_data_packet(packet_no, file_id, offset)
    let _ = (packet_no, file_id, offset);
    FeiQPacket::make_feiq_entry_packet(None)
}

/// 创建文件释放包
///
/// 用于通知发送方释放文件资源
pub fn create_file_release(packet_no: &str) -> FeiQPacket {
    // TODO: FeiQ file transfer not implemented yet
    // ProtocolPacket::make_release_files_packet(packet_no)
    let _ = packet_no;
    FeiQPacket::make_feiq_entry_packet(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: FeiQ file transfer not implemented yet - tests disabled
    // #[test]
    // fn test_handle_file_attach_request() {
    //     // 创建一个带文件附件的包
    //     let packet = ProtocolPacket {
    //         command: IPMSG_SENDMSG | IPMSG_FILEATTACHOPT,
    //         extension: Some("test.txt:1024:1234567890:1".to_string()),
    //         ..Default::default()
    //     };

    //     let files = handle_file_attach_request(&packet).unwrap();
    //     assert_eq!(files.len(), 1);
    //     assert_eq!(files[0].file_name, "test.txt");
    //     assert_eq!(files[0].file_size, 1024);
    // }

    // #[test]
    // fn test_create_file_attach_request() {
    //     let files = vec![FileAttachment {
    //         file_name: "test.txt".to_string(),
    //         file_size: 1024,
    //         mtime: 1234567890,
    //         attr: 1,
    //     }];

    //     let packet = create_file_attach_request(&files, "192.168.1.100", 2425);
    //     assert!(packet.has_option(IPMSG_FILEATTACHOPT));
    //     assert_eq!(packet.receiver, "192.168.1.100:2425");
    // }
}
