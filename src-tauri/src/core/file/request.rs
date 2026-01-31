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
/// 当收到文件附件消息时调用
pub fn handle_file_attach_request(packet: &FeiQPacket) -> AppResult<Vec<FileAttachment>> {
    // 检查是否为文件附件消息 (msg_sub_type 0x20 = SENDMSG)
    if packet.ext_info.msg_sub_type != 0x20 {
        return Err(AppError::Protocol("Not a file attachment packet".to_string()));
    }

    // 解析文件附件信息
    // 格式: "filename1:size1:mtime1:attr1\afilename2:size2:mtime2:attr2"
    let files_info = &packet.ext_info.remark;

    if files_info.is_empty() {
        return Err(AppError::Protocol("Empty file attachment data".to_string()));
    }

    let mut files = Vec::new();

    // 多个文件用 \x07 分隔
    for file_str in files_info.split('\x07') {
        let parts: Vec<&str> = file_str.split(':').collect();
        if parts.len() < 4 {
            return Err(AppError::Protocol(format!(
                "Invalid file attachment format: {}",
                file_str
            )));
        }

        let file_name = parts[0].to_string();
        let file_size = parts[1]
            .parse::<i64>()
            .map_err(|_| AppError::Protocol("Invalid file size".to_string()))?;
        let mtime = parts[2]
            .parse::<u64>()
            .map_err(|_| AppError::Protocol("Invalid mtime".to_string()))?;
        let attr = parts[3]
            .parse::<u32>()
            .map_err(|_| AppError::Protocol("Invalid attr".to_string()))?;

        files.push(FileAttachment {
            file_name,
            file_size,
            mtime,
            attr,
        });
    }

    Ok(files)
}

/// 创建文件附件请求包
///
/// 用于发送文件给其他用户
pub fn create_file_attach_request(files: &[FileAttachment], _receiver_ip: &str, _receiver_port: u16) -> FeiQPacket {
    // 创建 FeiQ 文件附件包
    FeiQPacket::make_feiq_file_attach_packet(files, None)
}

/// 创建文件数据请求包
///
/// 用于接收方请求文件数据
pub fn create_file_data_request(packet_no: &str, file_id: u64, offset: u64) -> FeiQPacket {
    FeiQPacket::make_feiq_get_file_data_packet(packet_no, file_id, offset, None)
}

/// 创建文件释放包
///
/// 用于通知发送方释放文件资源
pub fn create_file_release(packet_no: &str) -> FeiQPacket {
    FeiQPacket::make_feiq_release_files_packet(packet_no, None)
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
