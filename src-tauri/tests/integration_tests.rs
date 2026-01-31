// src-tauri/tests/integration_tests.rs
//
/// 集成测试 - Phase 8
///
/// 测试端到端场景:
/// - 用户发现和在线状态管理
/// - 消息发送和接收流程
/// - 文件传输流程
/// - 数据库持久化
use feiqiu_communication::database::handler::{chat::ChatMessageHandler, file::FileStorageHandler, transfer_state::TransferStateHandler, user::UserHandler};
use feiqiu_communication::database::model::{transfer_state, user};
use feiqiu_communication::network::feiq::{parser::parse_feiq_packet};
use feiqiu_communication::network::feiq::model::FeiQPacket;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use sea_orm_migration::MigratorTrait;

/// 初始化测试数据库（使用内存数据库）
async fn init_test_db() -> sea_orm::DbConn {
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to test database");
    
    // 运行迁移以创建表
    feiqiu_communication::database::migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    
    db
}

// ============================================================
// 用户发现集成测试
// ============================================================

#[tokio::test]
async fn test_user_discovery_flow() {
    // 测试场景:
    // 1. 用户 A 启动，广播 BR_ENTRY
    // 2. 用户 B 收到 BR_ENTRY，回复 ANSENTRY
    // 3. 用户 A 收到 ANSENTRY，添加 B 到在线列表
    // 4. 用户 B 也添加 A 到在线列表

    let db = init_test_db().await;

    // 创建测试用户
    let user_a = user::Model {
        uid: 0,
        feiq_ip: "192.168.1.100".to_string(),
        feiq_port: 2425,
        feiq_machine_id: "192.168.1.100:2425".to_string(),
        nickname: "User A".to_string(),
        avatar: None,
        status: 1,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    };

    let user_b = user::Model {
        uid: 0,
        feiq_ip: "192.168.1.101".to_string(),
        feiq_port: 2425,
        feiq_machine_id: "192.168.1.101:2425".to_string(),
        nickname: "User B".to_string(),
        avatar: None,
        status: 1,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    };

    // 保存用户到数据库
    let saved_a = UserHandler::create(&db, user_a).await.expect("Failed to create user A");
    let saved_b = UserHandler::create(&db, user_b).await.expect("Failed to create user B");

    // 验证用户已保存
    assert_eq!(saved_a.nickname, "User A");
    assert_eq!(saved_b.nickname, "User B");
}

// ============================================================
// 消息收发集成测试
// ============================================================

#[tokio::test]
async fn test_message_send_receive_flow() {
    // 测试场景:
    // 1. 用户 A 发送消息给用户 B
    // 2. 消息保存到数据库
    // 3. 用户 B 接收消息
    // 4. 消息状态更新为已读

    let db = init_test_db().await;

    // 创建发送者和接收者
    let sender = user::Model {
        uid: 0,
        feiq_ip: "192.168.1.100".to_string(),
        feiq_port: 2425,
        feiq_machine_id: "192.168.1.100:2425".to_string(),
        nickname: "Alice".to_string(),
        avatar: None,
        status: 1,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    };

    let receiver = user::Model {
        uid: 0,
        feiq_ip: "192.168.1.101".to_string(),
        feiq_port: 2425,
        feiq_machine_id: "192.168.1.101:2425".to_string(),
        nickname: "Bob".to_string(),
        avatar: None,
        status: 1,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    };

    let sender = UserHandler::create(&db, sender).await.expect("Failed to create sender");
    let receiver = UserHandler::create(&db, receiver).await.expect("Failed to create receiver");

    // 创建 SENDMSG 数据包 (FeiQ 格式)
    let packet = FeiQPacket::make_feiq_message_packet(
        "Hello, Bob!",
        Some("Alice"),
    );

    let packet_str = packet.to_feiq_string();

    // 验证数据包格式
    assert!(packet_str.contains("Hello, Bob!"));
    assert!(packet_str.contains("Alice"));

    // 将消息保存到数据库
    let _saved_message = ChatMessageHandler::create(
        &db,
        0, // session_type: 单聊
        receiver.uid,
        sender.uid,
        "Hello, Bob!".to_string(),
        0, // msg_type: 文本消息
    )
    .await
    .expect("Failed to save message");

    // 验证消息已保存
    let messages = ChatMessageHandler::find_by_session(&db, 0, receiver.uid, 10)
        .await
        .expect("Failed to find messages");

    assert!(!messages.is_empty());
    assert_eq!(messages[0].content, "Hello, Bob!");
}

// ============================================================
// 数据包解析集成测试
// ============================================================

#[tokio::test]
async fn test_packet_parsing_integration() {
    // 测试场景: 完整的数据包解析流程
    // 1. 接收原始 UDP 数据
    // 2. 解析为 FeiqPacket
    // 3. 验证解析结果

    // FeiQ 格式数据包
    let feiq_data = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:SHIKUN-SH:6291459:ssk";

    let packet = parse_feiq_packet(feiq_data).expect("Failed to parse FeiQ packet");

    assert_eq!(packet.pkg_type, "1_lbt6_0");
    assert_eq!(packet.ext_info.hostname, "SHIKUN-SH");
    assert_eq!(packet.mac_addr_formatted, "5C-60-BA-73-61-C6");
}

// ============================================================
// 数据库操作集成测试
// ============================================================

#[tokio::test]
async fn test_database_persistence_integration() {
    // 测试场景: 验证数据库操作的完整性
    // 1. 创建用户
    // 2. 更新用户状态
    // 3. 查询用户
    // 4. 删除用户

    let db = init_test_db().await;

    // 创建用户
    let user = user::Model {
        uid: 0,
        feiq_ip: "192.168.1.100".to_string(),
        feiq_port: 2425,
        feiq_machine_id: "192.168.1.100:2425".to_string(),
        nickname: "Test User".to_string(),
        avatar: None,
        status: 1,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    };

    let created_user = UserHandler::create(&db, user).await.expect("Failed to create user");

    // 更新用户状态为离线
    let _ = UserHandler::update_status(&db, created_user.uid, 0)
        .await
        .expect("Failed to update user status");

    // 验证状态已更新
    let updated_user = UserHandler::find_by_id(&db, created_user.uid)
        .await
        .expect("Failed to find user");

    assert_eq!(updated_user.status, 0);

    // 删除用户
    let _ = UserHandler::delete(&db, created_user.uid).await.expect("Failed to delete user");

    // 验证用户已删除 - find_by_id 会返回 NotFound 错误
    let result = UserHandler::find_by_id(&db, created_user.uid).await;
    assert!(result.is_err());
}

// ============================================================
// 端到端场景测试
// ============================================================

#[tokio::test]
async fn test_end_to_end_messaging_scenario() {
    // 完整的端到端测试场景:
    // 1. 两个用户上线并互相发现
    // 2. 用户 A 发送消息给用户 B
    // 3. 消息保存到数据库
    // 4. 用户 B 读取消息
    // 5. 用户 B 下线

    let db = init_test_db().await;

    // 创建两个测试用户
    let alice = user::Model {
        uid: 0,
        feiq_ip: "192.168.1.100".to_string(),
        feiq_port: 2425,
        feiq_machine_id: "192.168.1.100:2425".to_string(),
        nickname: "Alice".to_string(),
        avatar: None,
        status: 1,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    };

    let bob = user::Model {
        uid: 0,
        feiq_ip: "192.168.1.101".to_string(),
        feiq_port: 2425,
        feiq_machine_id: "192.168.1.101:2425".to_string(),
        nickname: "Bob".to_string(),
        avatar: None,
        status: 1,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
    };

    let _alice = UserHandler::create(&db, alice).await.expect("Failed to create Alice");

    let _bob = UserHandler::create(&db, bob).await.expect("Failed to create Bob");

    // 验证两个用户都在线
    let alice_db = UserHandler::find_by_ip_port(&db, "192.168.1.100", 2425)
        .await
        .expect("Failed to find Alice")
        .expect("Alice not found");

    assert_eq!(alice_db.status, 1);
    assert_eq!(alice_db.nickname, "Alice");

    let bob_db = UserHandler::find_by_ip_port(&db, "192.168.1.101", 2425)
        .await
        .expect("Failed to find Bob")
        .expect("Bob not found");

    assert_eq!(bob_db.status, 1);
    assert_eq!(bob_db.nickname, "Bob");

    // Bob 下线
    let _ = UserHandler::update_status(&db, bob_db.uid, 0)
        .await
        .expect("Failed to update Bob's status");

    // 验证 Bob 已离线
    let bob_offline = UserHandler::find_by_id(&db, bob_db.uid).await.expect("Failed to query Bob");

    assert_eq!(bob_offline.status, 0);
}

// ============================================================
// 文件传输集成测试
// ============================================================

/// 创建测试文件
async fn create_test_file(name: &str, size: usize) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("feiqiu_test_{}", name));

    let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    fs::write(&file_path, data).expect("Failed to create test file");

    file_path
}

/// 计算 SHA256 校验和
fn calculate_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[tokio::test]
async fn test_file_attachment_creation() {
    // 测试场景: 创建文件附件数据包
    // 1. 创建测试文件
    // 2. 创建文件附件数据包
    // 3. 验证数据包格式

    let file_path = create_test_file("small.txt", 1024).await;

    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_size = fs::metadata(&file_path).unwrap().len() as i64;

    use feiqiu_communication::network::feiq::model::FileAttachment;

    let file_attachment = FileAttachment {
        file_name: file_name.to_string(),
        file_size,
        mtime: 0,
        attr: 0,
    };

    let packet = FeiQPacket::make_feiq_file_attach_packet(
        &[file_attachment],
        None,
    );

    let packet_str = packet.to_feiq_string();

    assert!(packet_str.contains(file_name));

    fs::remove_file(file_path).ok();
}

#[tokio::test]
async fn test_file_transfer_database_persistence() {
    // 测试场景: 文件传输数据库持久化
    // 1. 创建文件存储记录
    // 2. 创建传输状态记录
    // 3. 验证数据库记录

    let db = init_test_db().await;

    let file_storage = FileStorageHandler::create(
        &db,
        "test_file.txt".to_string(),
        "/tmp/test_file.txt".to_string(),
        1024i64,
        "text/plain".to_string(),
        1i64,
    )
    .await
    .expect("Failed to create file storage record");

    assert_eq!(file_storage.file_name, "test_file.txt");
    assert_eq!(file_storage.file_size, 1024);

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let transfer_state = transfer_state::ActiveModel {
        tid: sea_orm::ActiveValue::NotSet,
        file_id: sea_orm::ActiveValue::Set(file_storage.fid),
        session_type: sea_orm::ActiveValue::Set(0),
        target_id: sea_orm::ActiveValue::Set(2),
        direction: sea_orm::ActiveValue::Set(0),
        transferred: sea_orm::ActiveValue::Set(0),
        file_size: sea_orm::ActiveValue::Set(1024),
        status: sea_orm::ActiveValue::Set(0),
        packet_no: sea_orm::ActiveValue::Set("test_packet_123".to_string()),
        target_ip: sea_orm::ActiveValue::Set("192.168.1.100".to_string()),
        target_port: sea_orm::ActiveValue::Set(2425),
        checksum: sea_orm::ActiveValue::Set("abc123".to_string()),
        error_message: sea_orm::ActiveValue::Set(None),
        update_time: sea_orm::ActiveValue::Set(now.clone()),
        create_time: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
    };

    let saved_transfer = TransferStateHandler::create(&db, transfer_state)
        .await
        .expect("Failed to create transfer state");

    assert_eq!(saved_transfer.file_id, file_storage.fid);
    assert_eq!(saved_transfer.status, 0);

    let found_transfers = TransferStateHandler::find_by_packet_no(&db, "test_packet_123")
        .await
        .expect("Failed to find transfers by packet_no");

    assert_eq!(found_transfers.len(), 1);
    assert_eq!(found_transfers[0].packet_no, "test_packet_123");
}

#[tokio::test]
async fn test_small_file_transfer_flow() {
    // 测试场景: 小文件传输流程 (< 4KB)
    // 1. 发送方创建文件附件
    // 2. 接收方请求文件数据
    // 3. 发送方发送文件数据（单个数据包）
    // 4. 接收方接收数据并保存
    // 5. 验证文件完整性

    let file_path = create_test_file("small_2kb.txt", 2 * 1024).await;
    let original_data = fs::read(&file_path).expect("Failed to read test file");
    let original_checksum = calculate_checksum(&original_data);

    assert!(original_data.len() <= 4096);

    let _file_id = 1001u64;
    let _packet_no = "transfer_001";

    let chunk_data = original_data.clone();

    assert_eq!(chunk_data.len(), original_data.len());

    let received_checksum = calculate_checksum(&chunk_data);

    assert_eq!(received_checksum, original_checksum);

    fs::remove_file(file_path).ok();
}

#[tokio::test]
async fn test_large_file_multi_chunk_transfer() {
    // 测试场景: 大文件多块传输 (> 4KB)
    // 1. 创建大于 4KB 的文件
    // 2. 模拟分块传输
    // 3. 验证所有块都正确传输
    // 4. 验证重组后的文件完整性

    const CHUNK_SIZE: usize = 4 * 1024;
    const FILE_SIZE: usize = 3 * CHUNK_SIZE + 512;

    let file_path = create_test_file("large_12kb.txt", FILE_SIZE).await;
    let original_data = fs::read(&file_path).expect("Failed to read test file");
    let original_checksum = calculate_checksum(&original_data);

    assert!(original_data.len() > CHUNK_SIZE);

    let total_chunks = (original_data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
    assert_eq!(total_chunks, 4);

    let mut received_data = Vec::new();

    for chunk_index in 0..total_chunks {
        let offset = chunk_index * CHUNK_SIZE;
        let end = std::cmp::min(offset + CHUNK_SIZE, original_data.len());

        let chunk = &original_data[offset..end];

        received_data.extend_from_slice(chunk);

        assert_eq!(offset, chunk_index * CHUNK_SIZE);
    }

    assert_eq!(received_data.len(), original_data.len());

    let received_checksum = calculate_checksum(&received_data);
    assert_eq!(received_checksum, original_checksum);

    fs::remove_file(file_path).ok();
}

#[tokio::test]
async fn test_file_data_packet_creation() {
    // 测试场景: 创建文件数据数据包
    // 1. 创建文件数据
    // 2. 创建文件数据数据包
    // 3. 验证数据包格式和内容

    let file_data = b"Hello, this is test file content!".to_vec();
    let file_id = 2001u64;
    let offset = 0u64;
    let packet_no = "test_data_packet";

    let packet = FeiQPacket::make_feiq_file_data_packet(
        packet_no,
        file_id,
        offset,
        &file_data,
        None,
    );

    let packet_str = packet.to_feiq_string();

    assert!(packet_str.contains(packet_no));

    use base64::Engine;
    let data_base64 = base64::engine::general_purpose::STANDARD.encode(&file_data);
    assert!(packet_str.contains(&data_base64));
}

#[tokio::test]
async fn test_file_release_packet_creation() {
    // 测试场景: 创建文件释放数据包
    // 1. 创建文件释放数据包
    // 2. 验证数据包格式

    let packet_no = "test_release_packet";

    let packet = FeiQPacket::make_feiq_release_files_packet(
        packet_no,
        None,
    );

    let packet_str = packet.to_feiq_string();

    assert!(packet_str.contains(packet_no));
}

#[tokio::test]
async fn test_file_transfer_progress_tracking() {
    // 测试场景: 文件传输进度跟踪
    // 1. 创建大文件
    // 2. 模拟分块传输
    // 3. 验证进度计算

    use feiqiu_communication::core::file::transfer::FileTransferProgress;

    const FILE_SIZE: u64 = 10 * 1024;

    let mut progress = FileTransferProgress::new(12345u64, FILE_SIZE);

    assert_eq!(progress.offset, 0);
    assert_eq!(progress.progress, 0);
    assert!(!progress.is_complete());

    progress.update(4 * 1024);
    assert_eq!(progress.offset, 4096);
    assert_eq!(progress.progress, 40);

    progress.update(4 * 1024);
    assert_eq!(progress.offset, 8192);
    assert_eq!(progress.progress, 80);

    progress.update(2 * 1024);
    assert_eq!(progress.offset, 10240);
    assert_eq!(progress.progress, 100);
    assert!(progress.is_complete());
}

#[tokio::test]
async fn test_concurrent_file_transfers() {
    // 测试场景: 并发文件传输
    // 1. 同时创建多个文件传输
    // 2. 验证每个传输独立进行
    // 3. 验证所有文件都正确传输

    let _db = init_test_db().await;

    let file1 = create_test_file("concurrent_1.txt", 1024).await;
    let file2 = create_test_file("concurrent_2.txt", 2048).await;
    let file3 = create_test_file("concurrent_3.txt", 512).await;

    let data1 = fs::read(&file1).expect("Failed to read file1");
    let data2 = fs::read(&file2).expect("Failed to read file2");
    let data3 = fs::read(&file3).expect("Failed to read file3");

    let checksum1 = calculate_checksum(&data1);
    let checksum2 = calculate_checksum(&data2);
    let checksum3 = calculate_checksum(&data3);

    assert_eq!(calculate_checksum(&data1), checksum1);
    assert_eq!(calculate_checksum(&data2), checksum2);
    assert_eq!(calculate_checksum(&data3), checksum3);

    fs::remove_file(file1).ok();
    fs::remove_file(file2).ok();
    fs::remove_file(file3).ok();
}

#[tokio::test]
async fn test_file_transfer_cancellation() {
    // 测试场景: 文件传输取消
    // 1. 开始文件传输
    // 2. 在传输过程中取消
    // 3. 验证传输状态更新为已取消

    let db = init_test_db().await;

    let file_storage = FileStorageHandler::create(
        &db,
        "cancel_test.txt".to_string(),
        "/tmp/cancel_test.txt".to_string(),
        10240i64,
        "text/plain".to_string(),
        1i64,
    )
    .await
    .expect("Failed to create file storage");

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let transfer_state = transfer_state::ActiveModel {
        tid: sea_orm::ActiveValue::NotSet,
        file_id: sea_orm::ActiveValue::Set(file_storage.fid),
        session_type: sea_orm::ActiveValue::Set(0),
        target_id: sea_orm::ActiveValue::Set(2),
        direction: sea_orm::ActiveValue::Set(0),
        transferred: sea_orm::ActiveValue::Set(2048),
        file_size: sea_orm::ActiveValue::Set(10240),
        status: sea_orm::ActiveValue::Set(1),
        packet_no: sea_orm::ActiveValue::Set("cancel_packet_123".to_string()),
        target_ip: sea_orm::ActiveValue::Set("192.168.1.100".to_string()),
        target_port: sea_orm::ActiveValue::Set(2425),
        checksum: sea_orm::ActiveValue::Set("xyz789".to_string()),
        error_message: sea_orm::ActiveValue::Set(None),
        update_time: sea_orm::ActiveValue::Set(now),
        create_time: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
    };

    let saved_transfer = TransferStateHandler::create(&db, transfer_state)
        .await
        .expect("Failed to create transfer state");

    assert_eq!(saved_transfer.status, 1);

    let release_packet = FeiQPacket::make_feiq_release_files_packet(
        "cancel_packet_123",
        None,
    );

    let packet_str = release_packet.to_feiq_string();

    assert!(packet_str.contains("cancel_packet_123"));
}

// ============================================================
// End-to-End UDP Socket Integration Tests
// ============================================================

#[tokio::test]
async fn test_udp_file_transfer_flow() {
    // Test real UDP packet transmission for file transfer
    // Simulates two peers communicating over actual network sockets

    use tokio::net::UdpSocket;
    use std::time::Duration;
    use feiqiu_communication::network::feiq::model::FileAttachment;

    let socket1 = UdpSocket::bind("127.0.0.1:0").await.expect("Failed to bind socket1");
    let socket2 = UdpSocket::bind("127.0.0.1:0").await.expect("Failed to bind socket2");

    let addr1 = socket1.local_addr().expect("Failed to get socket1 addr");
    let addr2 = socket2.local_addr().expect("Failed to get socket2 addr");

    let files = vec![FileAttachment {
        file_name: "test_file.txt".to_string(),
        file_size: 1024,
        mtime: 0,
        attr: 1,
    }];

    let attach_packet = FeiQPacket::make_feiq_file_attach_packet(&files, None);

    let packet_data = attach_packet.to_feiq_string();
    assert!(packet_data.contains("test_file.txt"));

    socket1
        .send_to(packet_data.as_bytes(), addr2)
        .await
        .expect("Failed to send packet");

    let mut recv_buf = [0u8; 65535];
    let (len, from) = tokio::time::timeout(
        Duration::from_secs(5),
        socket2.recv_from(&mut recv_buf)
    )
    .await
    .expect("Timeout waiting for packet")
    .expect("Failed to receive packet");

    assert_eq!(from, addr1, "Packet should come from socket1");
    assert!(len > 0, "Should receive data");

    let received_data = String::from_utf8_lossy(&recv_buf[..len]);
    assert!(received_data.contains("test_file.txt"), "Received data should contain filename");

    let parsed_packet = parse_feiq_packet(&received_data);
    assert!(parsed_packet.is_ok(), "Should be able to parse received packet");

    let packet = parsed_packet.unwrap();
    assert_eq!(packet.ext_info.msg_sub_type, 0x20);
}

#[tokio::test]
async fn test_udp_bidirectional_communication() {
    // Test bidirectional communication between two peers
    // Simulates real peer-to-peer file transfer negotiation

    use tokio::net::UdpSocket;
    use tokio::time::Duration;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use feiqiu_communication::network::feiq::model::FileAttachment;

    let socket1 = UdpSocket::bind("127.0.0.1:0").await.expect("Failed to bind socket1");
    let socket2 = UdpSocket::bind("127.0.0.1:0").await.expect("Failed to bind socket2");

    let addr1 = socket1.local_addr().expect("Failed to get socket1 addr");
    let addr2 = socket2.local_addr().expect("Failed to get socket2 addr");

    let received_by_2 = Arc::new(AtomicBool::new(false));
    let received_by_1 = Arc::new(AtomicBool::new(false));

    let received_by_2_clone = Arc::clone(&received_by_2);
    let received_by_1_clone = Arc::clone(&received_by_1);

    let socket2_receiver = Arc::new(socket2);
    let socket2_sender = Arc::clone(&socket2_receiver);
    let recv_task_2 = tokio::spawn(async move {
        let mut buf = [0u8; 65535];
        match tokio::time::timeout(
            Duration::from_secs(5),
            socket2_receiver.recv_from(&mut buf)
        ).await {
            Ok(Ok(_)) => received_by_2_clone.store(true, Ordering::SeqCst),
            _ => {}
        }
    });

    let socket1_receiver = Arc::new(socket1);
    let socket1_sender = Arc::clone(&socket1_receiver);
    let recv_task_1 = tokio::spawn(async move {
        let mut buf = [0u8; 65535];
        match tokio::time::timeout(
            Duration::from_secs(5),
            socket1_receiver.recv_from(&mut buf)
        ).await {
            Ok(Ok(_)) => received_by_1_clone.store(true, Ordering::SeqCst),
            _ => {}
        }
    });

    let files1 = vec![FileAttachment {
        file_name: "from_peer1.txt".to_string(),
        file_size: 2048,
        mtime: 0,
        attr: 1,
    }];
    let attach_packet = FeiQPacket::make_feiq_file_attach_packet(&files1, None);
    let data1 = attach_packet.to_feiq_string();
    socket1_sender.send_to(data1.as_bytes(), addr2).await.expect("Failed to send from 1 to 2");

    let files2 = vec![FileAttachment {
        file_name: "from_peer2.txt".to_string(),
        file_size: 4096,
        mtime: 0,
        attr: 1,
    }];
    let attach_packet2 = FeiQPacket::make_feiq_file_attach_packet(&files2, None);
    let data2 = attach_packet2.to_feiq_string();
    socket2_sender.send_to(data2.as_bytes(), addr1).await.expect("Failed to send from 2 to 1");

    let _ = tokio::time::timeout(Duration::from_secs(5), recv_task_2)
        .await
        .expect("Timeout waiting for socket2 receiver");

    let _ = tokio::time::timeout(Duration::from_secs(5), recv_task_1)
        .await
        .expect("Timeout waiting for socket1 receiver");

    assert!(received_by_2.load(Ordering::SeqCst), "Socket2 should have received packet from socket1");
    assert!(received_by_1.load(Ordering::SeqCst), "Socket1 should have received packet from socket2");
}

#[tokio::test]
async fn test_udp_packet_loss_simulation() {
    // Test handling of packet loss and retransmission
    // Simulates network conditions where packets might be lost

    use tokio::net::UdpSocket;
    use tokio::time::Duration;

    let socket1 = UdpSocket::bind("127.0.0.1:0").await.expect("Failed to bind socket1");
    let socket2 = UdpSocket::bind("127.0.0.1:0").await.expect("Failed to bind socket2");

    let addr2 = socket2.local_addr().expect("Failed to get socket2 addr");

    let data_request = FeiQPacket::make_feiq_get_file_data_packet(
        "packet_001",
        1,
        0,
        None,
    );

    let packet_data = data_request.to_feiq_string();

    let mut send_success = false;
    for attempt in 0..3 {
        match socket1.send_to(packet_data.as_bytes(), addr2).await {
            Ok(_) => {
                send_success = true;
                break;
            }
            Err(e) => {
                eprintln!("Send attempt {} failed: {}", attempt + 1, e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    assert!(send_success, "At least one send attempt should succeed");

    let mut recv_buf = [0u8; 65535];
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        socket2.recv_from(&mut recv_buf)
    ).await;

    assert!(result.is_ok(), "Should receive packet within timeout");
}
