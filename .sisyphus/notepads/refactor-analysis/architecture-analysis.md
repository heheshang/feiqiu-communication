# 飞秋通讯项目 - 架构分析报告

生成时间: 2026-01-29
分析范围: 后端 IPC/事件/业务逻辑层、前端状态管理/IPC/组件、数据库层

---

## 一、后端架构分析报告

### 1.1 IPC 层业务逻辑分析

#### 🔴 严重问题:业务逻辑大量泄露到 IPC 层

| 文件                        | 业务逻辑泄露点                                                                                                                                 | 应该移至                                           | 严重程度 |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------- | -------- |
| `src-tauri/src/ipc/chat.rs` | **L66-90**: 消息发送的完整流程<br>• 创建数据库记录<br>• 获取/创建会话<br>• 更新会话最后消息<br>• 构造网络包<br>• 发送 UDP 包<br>• 更新消息状态 | `core/chat/sender.rs` (MessageSender)              | **高**   |
| `src-tauri/src/ipc/chat.rs` | **L99-118**: 群聊/单聊分支逻辑<br>• 群聊广播逻辑<br>• 单聊目标用户查询<br>• 网络包构造与发送                                                   | `core/chat/sender.rs` (send_to_group/send_to_user) | **高**   |
| `src-tauri/src/ipc/chat.rs` | **L228-266**: 消息重试逻辑<br>• 查询消息详情<br>• 重置状态<br>• 查询目标用户<br>• 重新构造包<br>• 发送并更新状态                               | `core/chat/sender.rs` (retry_message)              | **中**   |
| `src-tauri/src/ipc/file.rs` | **L23-53**: 文件元数据读取<br>• 读取文件路径<br>• 获取文件大小<br>• 获取修改时间<br>• 判断文件/目录                                            | `core/file/request.rs` (prepare_file_metadata)     | **高**   |
| `src-tauri/src/ipc/file.rs` | **L66-108**: 文件传输状态管理<br>• 创建文件存储记录<br>• 创建传输状态记录<br>• 设置传输参数                                                    | `core/file/transfer.rs` (create_transfer_state)    | **高**   |
| `src-tauri/src/ipc/file.rs` | **L243-271**: 传输恢复逻辑<br>• 根据方向创建处理器<br>• 启动后台传输任务                                                                       | `core/file/resume.rs` (resume_transfer)            | **中**   |
| `src-tauri/src/ipc/user.rs` | **L18-86**: 用户初始化逻辑<br>• 获取本地网络信息<br>• 生成机器 ID<br>• 查询/创建用户<br>• 处理错误场景                                         | `core/user/manager.rs` (initialize_user)           | **中**   |

#### 具体代码示例

**问题 1: 消息发送逻辑泄露 (严重度: 高)**

```rust
// src-tauri/src/ipc/chat.rs:60-126
// ❌ 业务逻辑应该在 Service 层,而非 IPC 层
pub async fn send_text_message_handler(
    session_type: i8,
    target_id: i64,
    content: String,
    owner_uid: i64,
    state: State<'_, DbConn>,
) -> Result<i64, String> {
    let db = state.inner();

    // 业务逻辑 1: 创建消息记录
    let message = ChatMessageHandler::create(...).await?;

    // 业务逻辑 2: 获取/创建会话
    let session = ChatSessionHandler::get_or_create(...).await?;

    // 业务逻辑 3: 更新会话最后消息
    ChatSessionHandler::update_last_message(...).await?;

    // 业务逻辑 4: 构造网络包
    let packet = ProtocolPacket::make_message_packet(&content, true);

    // 业务逻辑 5: 发送逻辑 (群聊/单聊)
    if session_type == 1 {
        let sent_count = GroupBroadcaster::broadcast_message(...).await?;
    } else {
        let target_user = UserHandler::find_by_id(db, target_id).await?;
        let addr = format!("{}:{}", target_user.feiq_ip, target_user.feiq_port);
        sender::send_packet(&addr, &packet).await?;
    }

    // 业务逻辑 6: 更新消息状态
    ChatMessageHandler::update_status(db, message.mid, 1).await?;

    Ok(message.mid)
}
```

**应该重构为:**

```rust
// ✅ IPC 层只负责参数转换和错误映射
pub async fn send_text_message_handler(
    params: SendMessageParams,
    db: State<'_, DbConn>,
) -> Result<i64, String> {
    use crate::core::chat::sender::MessageSender;

    let sender = MessageSender::new(db.inner());
    sender.send_text_message(params)
        .await
        .map_err(|e| e.to_string())
}

// ✅ 业务逻辑在 Service 层
// src-tauri/src/core/chat/sender.rs
impl MessageSender {
    pub async fn send_text_message(
        &self,
        params: SendMessageParams,
    ) -> AppResult<i64> {
        //1. 创建消息
        //2. 管理/创建会话
        //3. 更新最后消息
        //4. 构造网络包
        //5. 发送 (群聊/单聊)
        //6. 更新状态
        //7. 发布事件
    }
}
```

**问题 2: 类型转换逻辑重复 (严重度: 中)**

```rust
// src-tauri/src/ipc/chat.rs:26-53 (重复模式)
let result: Vec<ChatMessage> = messages
    .into_iter()
    .map(|m| ChatMessage {
        mid: m.mid,
        session_type: if m.session_type == 0 { SessionType::Single } else { SessionType::Group },
        target_id: m.target_id,
        sender_uid: m.sender_uid,
        msg_type: match m.msg_type {
            0 => MessageType::Text,
            1 => MessageType::File,
            2 => MessageType::Emoji,
            _ => MessageType::Text,
        },
        content: m.content,
        send_time: m.send_time,
        status: match m.status {
            -1 => MessageStatus::Failed,
            0 => MessageStatus::Sending,
            1 => MessageStatus::Sent,
            2 => MessageStatus::Read,
            _ => MessageStatus::Sending,
        },
    })
    .collect();
```

**建议:** 创建 DTO 转换器

```rust
// src-tauri/src/core/dto/mod.rs
impl From<chat_message::Model> for ChatMessage {
    fn from(db_model: chat_message::Model) -> Self {
        Self {
            mid: db_model.mid,
            session_type: db_model.session_type.into(),
            target_id: db_model.target_id,
            sender_uid: db_model.sender_uid,
            msg_type: db_model.msg_type.into(),
            content: db_model.content,
            send_time: db_model.send_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            status: db_model.status.into(),
        }
    }
}
```

### 1.2 事件系统分析

#### ⚠️ 中等问题: 事件粒度与订阅机制不完善

| 问题              | 位置                                  | 严重程度 | 说明                                                         |
| ----------------- | ------------------------------------- | -------- | ------------------------------------------------------------ |
| 事件粒度过粗      | `src-tauri/src/event/model.rs:L33-64` | **中**   | `NetworkEvent::PacketReceived` 包含所有包,订阅者需要手动过滤 |
| 缺少事件过滤机制  | `src-tauri/src/main.rs:L273-291`      | **高**   | 主事件循环只处理少数事件,多数事件未消费                      |
| UI 事件未使用     | `src-tauri/src/main.rs:L294-298`      | **低**   | `UiEvent` 定义了但处理器为空                                 |
| 事件序列化开销    | `src-tauri/src/event/model.rs:L13-25` | **中**   | 所有事件通过 `AppEvent` 枚举包装,增加了序列化层级            |
| 缺少事件撤销/补偿 | 全局                                  | **中**   | 没有事件失败时的补偿机制                                     |

**具体代码示例:**

**问题 1: 事件粒度过粗 (严重度: 中)**

```rust
// src-tauri/src/event/model.rs:L33-38
// ❌ PacketReceived 包含所有包,订阅者需要手动解析命令字
pub enum NetworkEvent {
    PacketReceived {
        packet: String,  // FeiqPacket JSON 字符串
        addr: String,
    },
    // ...
}
```

**建议: 按协议命令细化事件**

```rust
// ✅ 细化事件类型
pub enum NetworkEvent {
    PacketReceived {
        packet: String,
        addr: String,
    },
    // 新增: 具体命令事件
    EntryReceived { user: UserInfo, addr: String },   // BR_ENTRY
    AnsEntryReceived { user: UserInfo, addr: String }, // ANSENTRY
    MessageReceived { msg: ChatMessage, addr: String }, // SENDMSG
    ExitReceived { ip: String },                      // BR_EXIT
}
```

**问题 2: 事件处理不完整 (严重度: 高)**

```rust
// src-tauri/src/main.rs:L273-291
async fn handle_network_event(event: crate::event::model::NetworkEvent, _app_handle: &tauri::AppHandle) {
    match event {
        crate::event::model::NetworkEvent::PacketReceived { packet, addr } => {
            info!("收到数据包: {} from {}", packet, addr);
            // ⚠️ 只记录日志,没有实际处理!
            // 数据包解析和处理由 discovery 模块的事件循环处理
        }
        crate::event::model::NetworkEvent::UserOnline { user } => {
            info!("用户上线事件: {}", user);
            // ⚠️ 没有通知前端!
        }
        crate::event::model::NetworkEvent::UserOffline { ip } => {
            info!("用户离线事件: {}", ip);
            // ⚠️ 没有通知前端!
        }
        _ => {}
    }
}
```

**建议: 完善事件到 UI 的通知**

```rust
async fn handle_network_event(event: NetworkEvent, app_handle: &tauri::AppHandle) {
    match event {
        NetworkEvent::UserOnline { user } => {
            info!("用户上线: {}", user.nickname);
            // 通知前端
            app_handle.emit_all("user-online", user).ok();
        }
        NetworkEvent::UserOffline { ip } => {
            info!("用户离线: {}", ip);
            app_handle.emit_all("user-offline", ip).ok();
        }
        NetworkEvent::PacketReceived { packet, addr } => {
            // 解析并分发到具体处理器
            let parsed = parse_packet(&packet)?;
            if let Some(msg) = parsed.as_message() {
                app_handle.emit_all("message-received", msg).ok();
            }
        }
        _ => {}
    }
}
```

### 1.3 业务逻辑层现状

#### ✅ 优点: 模块结构清晰

```
src-tauri/src/core/
├── chat/           # 聊天业务逻辑
│   ├── mod.rs      # 模块导出
│   ├── sender.rs   # 消息发送
│   ├── receiver.rs # 消息接收
│   ├── receipt.rs  # 已读回执
│   └── manager.rs  # 会话管理
├── contact/        # 联系人业务逻辑
│   ├── mod.rs
│   └── discovery.rs # 用户发现 ✅ 实现完整
├── file/           # 文件传输业务逻辑
│   ├── mod.rs
│   ├── request.rs  # 文件请求
│   ├── transfer.rs # 文件传输
│   └── resume.rs   # 传输恢复
└── group/          # 群组业务逻辑
    └── broadcast.rs # 群组广播
```

#### ⚠️ 问题: 业务逻辑分散

| 模块            | 状态         | 缺失部分                                                                     | 重复模式 |
| --------------- | ------------ | ---------------------------------------------------------------------------- | -------- |
| `core/chat/`    | **部分实现** | ✅ 消息接收/回执处理<br>❌ 消息发送逻辑在 IPC 层<br>❌ 会话创建逻辑在 IPC 层 | -        |
| `core/contact/` | **实现完整** | ✅ 用户发现完整实现                                                          | -        |
| `core/file/`    | **部分实现** | ✅ 传输协议处理<br>❌ 文件元数据处理在 IPC 层<br>❌ 传输                     |
