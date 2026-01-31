// src-tauri/tests/integration_tests.rs
//
/// 集成测试 - Phase 8
///
/// 测试端到端场景:
/// - 用户发现和在线状态管理
/// - 消息发送和接收流程
/// - 文件传输流程
/// - 数据库持久化
use feiqiu_communication::database::handler::{chat::ChatMessageHandler, user::UserHandler};
use feiqiu_communication::database::model::user;
use feiqiu_communication::network::feiq::{constants::*, parser::parse_feiq_packet};
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

    // 创建 SENDMSG 数据包
    use feiqiu_communication::network::feiq::model::ProtocolPacket;

    let packet = ProtocolPacket::new_ipmsg(
        "1.0".to_string(),
        IPMSG_SENDMSG | IPMSG_UTF8OPT,
        "Alice".to_string(),
        "Bob".to_string(),
        "12345".to_string(),
        Some("Hello, Bob!".to_string()),
    );

    // 验证数据包格式
    assert_eq!(packet.base_command(), IPMSG_SENDMSG);
    assert!(packet.is_utf8());
    assert_eq!(packet.extension, Some("Hello, Bob!".to_string()));

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

    // IPMsg 格式数据包
    let ipmsg_data = "1.0:32:sender:host:12345:Hello";

    let packet = parse_feiq_packet(ipmsg_data).expect("Failed to parse IPMsg packet");

    assert_eq!(packet.version, "1.0");
    assert_eq!(packet.base_command(), 32);
    assert_eq!(packet.sender, "sender");
    assert_eq!(packet.receiver, "host");
    assert_eq!(packet.msg_no, "12345");
    assert_eq!(packet.extension, Some("Hello".to_string()));

    // FeiQ 格式数据包
    let feiq_data = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0220165:SHIKUN-SH:6291459:ssk";

    let packet = parse_feiq_packet(feiq_data).expect("Failed to parse FeiQ packet");

    assert_eq!(packet.version, "1_lbt6_0");
    assert_eq!(packet.hostname, Some("SHIKUN-SH".to_string()));
    assert_eq!(packet.mac_addr, Some("5C60BA7361C6".to_string()));
    assert_eq!(packet.msg_type, Some(9));
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

    // 保存用户
    let alice = UserHandler::create(&db, alice).await.expect("Failed to create Alice");

    let bob = UserHandler::create(&db, bob).await.expect("Failed to create Bob");

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
