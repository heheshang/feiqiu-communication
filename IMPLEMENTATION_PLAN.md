# 飞秋通讯 - 实施计划

> **项目**: 基于 Tauri 2.0 的内网通讯软件
> **创建日期**: 2025-01-27
> **计划状态**: 初始版本
> **预计工期**: 12 周

---

## 目录

1. [项目概述](#一项目概述)
2. [实施阶段划分](#二实施阶段划分)
3. [详细任务分解](#三详细任务分解)
4. [技术实施规范](#四技术实施规范)
5. [验收标准](#五验收标准)
6. [风险控制](#六风险控制)

---

## 一、项目概述

### 1.1 目标

构建一个基于飞秋（FeiQ/IPMsg）协议的内网即时通讯软件，采用 Tauri 2.0 + Rust + React 技术栈，实现跨平台桌面应用。

### 1.2 核心功能

| 功能模块 | 优先级 | 说明                           |
| -------- | ------ | ------------------------------ |
| 用户发现 | P0     | 内网用户自动发现、在线状态同步 |
| 单聊     | P0     | 点对点文字消息、已读回执       |
| 群聊     | P0     | 群组创建、成员管理、消息广播   |
| 文件传输 | P0     | 单聊/群聊文件传输、断点续传    |
| 通讯录   | P1     | 联系人管理、分组、搜索         |
| UI 界面  | P0     | 仿微信风格、响应式设计         |

### 1.3 当前状态

- ✅ 架构文档已完成
- ✅ 参考实现分析已完成
- ✅ Cargo.toml 依赖配置已就绪
- ✅ 部分参考代码已准备
- ⏳ 项目结构初始化（待进行）
- ⏳ 核心功能开发（待进行）

---

## 二、实施阶段划分

```
Phase 1: 项目基础搭建 ━━━━━━━━━━━━━━━━━━━━ Week 1-2
   ├─ Tauri + React 项目初始化
   ├─ 开发环境配置
   └─ 基础目录结构创建

Phase 2: 飞秋协议基础 ━━━━━━━━━━━━━━━━━━━━ Week 3-4
   ├─ 协议解析器（parser.rs）
   ├─ 协议封装器（packer.rs）
   ├─ UDP 通信模块
   └─ 用户在线发现

Phase 3: 数据库与持久化 ━━━━━━━━━━━━━━━━━━━ Week 5
   ├─ SeaORM 模型定义
   ├─ 数据库迁移脚本
   ├─ CRUD 封装
   └─ 数据库初始化

Phase 4: 基础 UI 界面 ━━━━━━━━━━━━━━━━━━━━ Week 6-7
   ├─ 三栏布局框架
   ├─ 通讯录组件
   ├─ 聊天窗口组件
   └─ IPC 接口对接

Phase 5: 消息功能完善 ━━━━━━━━━━━━━━━━━━━ Week 8
   ├─ 消息历史分页
   ├─ 已读回执
   ├─ Emoji 支持
   └─ 消息状态管理

Phase 6: 文件传输功能 ━━━━━━━━━━━━━━━━━━━ Week 9-10
   ├─ 文件请求/确认
   ├─ 分块传输
   ├─ 传输进度展示
   └─ 断点续传

Phase 7: 群聊功能 ━━━━━━━━━━━━━━━━━━━━━━ Week 11
   ├─ 群组创建
   ├─ 成员管理
   └─ 群消息广播

Phase 8: 优化与测试 ━━━━━━━━━━━━━━━━━━━━ Week 12
   ├─ 性能优化
   ├─ 单元测试完善
   ├─ 集成测试
   └─ 跨平台测试
```

---

## 三、详细任务分解

### Phase 1: 项目基础搭建 (Week 1-2)

#### 任务 1.1: Tauri 项目初始化

**目标**: 创建完整的 Tauri + React 项目结构

**步骤**:

1. 使用 `bun create tauri-app@latest` 初始化项目
2. 配置 TypeScript、Vite、Less
3. 设置代码规范（ESLint + Prettier）
4. 配置 Git 仓库

**交付物**:

- 可运行的 Tauri 应用骨架
- 完整的目录结构
- 开发环境配置文档

**验收标准**:

- [ ] `bun run tauri dev` 成功启动
- [ ] 能看到默认的 Tauri 窗口
- [ ] 热更新正常工作

---

#### 任务 1.2: Rust 后端目录结构

**目标**: 创建符合架构规范的 Rust 目录结构

**步骤**:

1. 创建 `src/` 下的所有模块目录
2. 添加 `mod.rs` 文件
3. 配置 Cargo.toml 依赖
4. 设置编译选项

**交付物**:

```
src/
├── main.rs                 # Tauri 入口
├── lib.rs                  # 库入口
├── error.rs                # 错误定义
├── types.rs                # 共享类型
├── core/                   # 核心业务层
│   ├── mod.rs
│   ├── chat/
│   ├── contact/
│   ├── file/
│   └── group/
├── database/               # 数据访问层
│   ├── mod.rs
│   ├── model/
│   ├── migration/
│   └── handler/
├── network/                # 网络通信层
│   ├── mod.rs
│   ├── feiq/
│   └── udp/
├── ipc/                    # IPC 接口层
│   ├── mod.rs
│   ├── chat.rs
│   ├── contact.rs
│   ├── file.rs
│   └── group.rs
├── event/                  # 事件系统
│   ├── mod.rs
│   ├── bus.rs
│   └── model.rs
└── utils/                  # 工具模块
    ├── mod.rs
    ├── snowflake/
    └── serde/
```

**验收标准**:

- [ ] `cargo check` 无错误
- [ ] 所有模块正确导出
- [ ] 目录结构符合文档规范

---

#### 任务 1.3: 前端目录结构

**目标**: 创建符合架构规范的 React 目录结构

**步骤**:

1. 创建组件目录
2. 创建 hooks 目录
3. 创建 IPC 封装目录
4. 创建状态管理目录
5. 配置 Less 主题变量

**交付物**:

```
src/
├── App.tsx                 # 应用根组件
├── main.tsx                # 应用入口
├── components/             # UI 组件
│   ├── Contact/
│   ├── ChatWindow/
│   ├── EmojiPicker/
│   ├── FileUpload/
│   └── SessionList/
├── hooks/                  # 自定义钩子
│   ├── useIPC.ts
│   ├── useChat.ts
│   └── useContact.ts
├── ipc/                    # IPC 封装
│   ├── chat.ts
│   ├── contact.ts
│   ├── file.ts
│   └── index.ts
├── store/                  # 状态管理
│   ├── chatStore.ts
│   ├── userStore.ts
│   └── index.ts
├── styles/                 # 样式
│   ├── variables.less
│   ├── mixins.less
│   └── global.less
├── utils/                  # 工具函数
│   ├── emoji.ts
│   ├── time.ts
│   └── path.ts
└── types/                  # TypeScript 类型
    ├── chat.ts
    ├── user.ts
    └── index.ts
```

**验收标准**:

- [ ] `bun run dev` 无错误
- [ ] TypeScript 类型检查通过
- [ ] Less 样式正常编译

---

### Phase 2: 飞秋协议基础 (Week 3-4)

#### 任务 2.1: 协议常量定义

**目标**: 定义所有飞秋协议常量

**文件**: `src/network/feiq/constants.rs`

**内容**:

```rust
/// 默认端口
pub const FEIQ_DEFAULT_PORT: u16 = 2425;

/// 命令字 (低 8 位)
pub const IPMSG_NOOPERATION: u32 = 0x00000000;
pub const IPMSG_BR_ENTRY: u32 = 0x00000001;
pub const IPMSG_BR_EXIT: u32 = 0x00000002;
pub const IPMSG_ANSENTRY: u32 = 0x00000003;
pub const IPMSG_SENDMSG: u32 = 0x00000020;
pub const IPMSG_RECVMSG: u32 = 0x00000021;
pub const IPMSG_READMSG: u32 = 0x00000030;
pub const IPMSG_ANSREADMSG: u32 = 0x00000032;
pub const IPMSG_GETFILEDATA: u32 = 0x00000060;
pub const IPMSG_RELEASEFILES: u32 = 0x00000061;

/// 选项标志
pub const IPMSG_SENDCHECKOPT: u32 = 0x00000100;
pub const IPMSG_FILEATTACHOPT: u32 = 0x00200000;
pub const IPMSG_UTF8OPT: u32 = 0x00800000;

// ... 更多常量
```

**验收标准**:

- [ ] 所有常量定义完整
- [ ] 单元测试覆盖

---

#### 任务 2.2: 协议数据模型

**目标**: 定义飞秋数据包结构

**文件**: `src/network/feiq/model.rs`

**内容**:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeiqPacket {
    pub version: String,          // 协议版本 "1.0"
    pub command: u32,             // 命令字（含选项标志）
    pub sender: String,           // 发送者信息
    pub receiver: String,         // 接收者信息
    pub msg_no: String,           // 消息编号
    pub extension: Option<String>, // 附加信息
    pub ip: String,               // 发送者 IP（由外部填充）
}

impl FeiqPacket {
    /// 获取基础命令字（去除选项标志）
    pub fn base_command(&self) -> u32 {
        self.command & 0xFF
    }

    /// 检查是否包含某个选项标志
    pub fn has_option(&self, flag: u32) -> bool {
        (self.command & flag) != 0
    }
}
```

**验收标准**:

- [ ] 结构体定义完整
- [ ] 实现必要的辅助方法
- [ ] 序列化/反序列化正常

---

#### 任务 2.3: 协议解析器

**目标**: 实现飞秋协议解析器（使用 combine）

**文件**: `src/network/feiq/parser.rs`

**参考**: `reference/network/feiq/parser.rs`

**关键功能**:

1. 主解析器 `feiq_packet_parser()`
2. 发送者信息解析器
3. 文件头解析器（文件传输用）
4. 错误处理

**单元测试**:

```rust
#[test]
fn test_parse_entry_packet() {
    let input = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
    let packet = feiq_packet_parser().parse(input);
    assert!(packet.is_ok());
}
```

**验收标准**:

- [ ] 能解析 BR_ENTRY 包
- [ ] 能解析 SENDMSG 包
- [ ] 能解析带附件的包
- [ ] 单元测试覆盖率 > 80%

---

#### 任务 2.4: 协议封装器

**目标**: 实现飞秋协议封装器

**文件**: `src/network/feiq/packer.rs`

**关键功能**:

```rust
impl FeiqPacket {
    /// 创建在线广播包
    pub fn make_entry_packet() -> Self {
        // 构造 BR_ENTRY 包
    }

    /// 创建在线响应包
    pub fn make_ansentry_packet() -> Self {
        // 构造 ANSENTRY 包
    }

    /// 创建消息包
    pub fn make_message_packet(content: &str, to_ip: &str) -> Self {
        // 构造 SENDMSG 包
    }

    /// 序列化为字符串
    pub fn to_string(&self) -> String {
        // 格式: "1.0:command:sender:receiver:msg_no:extension"
    }
}
```

**验收标准**:

- [ ] 能创建各种类型的数据包
- [ ] 序列化格式符合飞秋协议
- [ ] 单元测试覆盖

---

#### 任务 2.5: UDP 接收器

**目标**: 实现异步 UDP 接收器

**文件**: `src/network/udp/receiver.rs`

**关键功能**:

```rust
use tokio::net::UdpSocket;

/// 启动 UDP 接收器
pub async fn start_udp_receiver() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2425").await?;
    let mut buf = [0; 2048];

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let data = String::from_utf8_lossy(&buf[..len]);
                // 解析并发送到事件总线
            }
            Err(e) => {
                error!("UDP receive error: {}", e);
            }
        }
    }
}
```

**验收标准**:

- [ ] 能绑定 2425 端口
- [ ] 能接收 UDP 数据包
- [ ] 能将数据包发送到事件总线

---

#### 任务 2.6: UDP 发送器

**目标**: 实现异步 UDP 发送器

**文件**: `src/network/udp/sender.rs`

**关键功能**:

```rust
/// 发送 UDP 数据包
pub async fn send_packet(addr: &str, packet: &FeiqPacket) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let data = packet.to_string();
    socket.send_to(data.as_bytes(), addr).await?;
    Ok(())
}

/// 广播 UDP 数据包
pub async fn broadcast_packet(packet: &FeiqPacket) -> anyhow::Result<()> {
    send_packet("255.255.255.255:2425", packet).await
}
```

**验收标准**:

- [ ] 能发送单播消息
- [ ] 能发送广播消息
- [ ] 错误处理完善

---

#### 任务 2.7: 用户在线发现

**目标**: 实现用户自动发现功能

**文件**: `src/core/contact/discovery.rs`

**流程**:

```
启动时:
1. 广播 BR_ENTRY 包
2. 监听其他用户的 BR_ENTRY
3. 收到 BR_ENTRY 时回复 ANSENTRY
4. 维护在线用户列表
```

**关键代码**:

```rust
/// 启动用户发现
pub async fn start_discovery() {
    // 1. 广播上线
    broadcast_entry().await;

    // 2. 监听事件总线
    loop {
        match EVENT_RECEIVER.recv() {
            AppEvent::Network(NetworkEvent::PacketReceived { packet, addr }) => {
                match packet.base_command() {
                    IPMSG_BR_ENTRY => {
                        // 回复 ANSENTRY
                        send_ansentry(&addr).await;
                        // 更新用户列表
                        add_online_user(&packet, &addr).await;
                    }
                    IPMSG_ANSENTRY => {
                        add_online_user(&packet, &addr).await;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
```

**验收标准**:

- [ ] 启动时能广播上线
- [ ] 能发现其他上线用户
- [ ] 能回复其他用户的上线请求
- [ ] 两台机器能互相发现

---

### Phase 3: 数据库与持久化 (Week 5)

#### 任务 3.1: SeaORM 实体定义

**目标**: 定义所有数据库实体模型

**文件**:

- `src/database/model/user.rs`
- `src/database/model/contact.rs`
- `src/database/model/group.rs`
- `src/database/model/chat_message.rs`
- `src/database/model/chat_session.rs`
- `src/database/model/file_storage.rs`

**示例**:

```rust
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: i64,
    pub feiq_ip: String,
    pub feiq_port: u16,
    pub feiq_machine_id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub status: i8,
    pub create_time: DateTime,
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

**验收标准**:

- [ ] 所有实体定义完成
- [ ] 符合架构文档的表结构
- [ ] 导出正确

---

#### 任务 3.2: 数据库迁移脚本

**目标**: 创建数据库迁移脚本

**工具**: sea-orm-cli

**步骤**:

```bash
# 安装 sea-orm-cli
cargo install sea-orm-cli

# 初始化迁移
sea-orm-cli migrate init

# 生成迁移
sea-orm-cli migrate generate create_user_table
```

**迁移文件**:

- `migrations/m20250127_000001_create_user_table.rs`
- `migrations/m20250127_000002_create_contact_table.rs`
- `migrations/m20250127_000003_create_group_tables.rs`
- `migrations/m20250127_000004_create_chat_tables.rs`
- `migrations/m20250127_000005_create_file_storage_table.rs`

**验收标准**:

- [ ] 所有表创建脚本完成
- [ ] 索引定义正确
- [ ] 迁移能成功执行

---

#### 任务 3.3: CRUD 封装

**目标**: 实现统一的数据库访问接口

**文件**: `src/database/handler/*.rs`

**示例**:

```rust
use sea_orm::{EntityTrait, ActiveModelTrait, DatabaseConnection};

/// 用户处理器
pub struct UserHandler {
    db: DatabaseConnection,
}

impl UserHandler {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建或更新用户
    pub async fn upsert(&self, user: UserModel) -> Result<UserModel, DbErr> {
        // 实现逻辑
    }

    /// 根据 machine_id 查找用户
    pub async fn find_by_machine_id(&self, machine_id: &str) -> Result<Option<UserModel>, DbErr> {
        // 实现逻辑
    }

    /// 获取所有在线用户
    pub async fn get_online_users(&self) -> Result<Vec<UserModel>, DbErr> {
        // 实现逻辑
    }
}
```

**验收标准**:

- [ ] 所有 CRUD 方法实现
- [ ] 错误处理完善
- [ ] 单元测试覆盖

---

#### 任务 3.4: 数据库初始化

**目标**: 实现应用启动时的数据库初始化

**文件**: `src/database/mod.rs`

**功能**:

```rust
use sea_orm::{Database, DbConn};

/// 初始化数据库连接
pub async fn init_database() -> Result<DbConn, DbErr> {
    let db = Database::connect("sqlite://./feiqiu.db").await?;

    // 执行迁移
    run_migrations(&db).await?;

    Ok(db)
}

/// 运行所有迁移
async fn run_migrations(db: &DbConn) -> Result<(), DbErr> {
    // 使用 sea-orm-migration 运行迁移
}
```

**验收标准**:

- [ ] 数据库文件自动创建
- [ ] 迁移自动执行
- [ ] 错误处理完善

---

### Phase 4: 基础 UI 界面 (Week 6-7)

#### 任务 4.1: Less 主题配置

**目标**: 实现仿微信配色方案

**文件**: `src/styles/variables.less`

**变量定义**:

```less
// 仿微信配色
@primary-color: #07c160; // 微信绿
@bg-color: #f5f5f5; // 背景灰
@sidebar-bg: #ededed; // 侧边栏灰
@chat-bg: #f5f5f5; // 聊天背景
@border-color: #dcdcdc; // 边框色

// 字体
@font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
@font-size-base: 14px;
@font-size-small: 12px;

// 间距
@spacing-xs: 4px;
@spacing-sm: 8px;
@spacing-md: 12px;
@spacing-lg: 16px;
@spacing-xl: 24px;
```

**验收标准**:

- [ ] 变量定义完整
- [ ] Less 编译正常

---

#### 任务 4.2: 三栏布局框架

**目标**: 实现主窗口三栏布局

**文件**: `src/components/MainLayout.tsx`

**布局结构**:

```tsx
<div className="main-layout">
  <div className="sidebar">          <!-- 左侧：最近会话 -->
    <SessionList />
  </div>
  <div className="contact-panel">    <!-- 中间：通讯录 -->
    <ContactList />
  </div>
  <div className="chat-panel">       <!-- 右侧：聊天窗口 -->
    <ChatWindow />
  </div>
</div>
```

**样式**: `src/styles/layout.less`

**验收标准**:

- [ ] 三栏布局正确显示
- [ ] 可调整列宽
- [ ] 响应式适配

---

#### 任务 4.3: 通讯录组件

**目标**: 实现通讯录列表和搜索

**文件**: `src/components/Contact/*`

**组件结构**:

- `ContactList.tsx` - 联系人列表
- `ContactItem.tsx` - 单个联系人项
- `ContactSearch.tsx` - 搜索框
- `ContactGroup.tsx` - 分组展示

**功能**:

- 显示在线用户列表
- 实时搜索过滤
- 显示在线状态
- 点击打开聊天

**验收标准**:

- [ ] 能显示在线用户
- [ ] 搜索功能正常
- [ ] 点击能切换聊天

---

#### 任务 4.4: 聊天窗口组件

**目标**: 实现聊天窗口界面

**文件**: `src/components/ChatWindow/*`

**组件结构**:

- `ChatWindow.tsx` - 聊天窗口容器
- `MessageList.tsx` - 消息列表
- `MessageItem.tsx` - 单条消息
- `MessageInput.tsx` - 输入框

**功能**:

- 显示消息历史
- 发送消息
- 消息气泡样式
- 时间戳显示

**验收标准**:

- [ ] 消息正确显示
- [ ] 能发送消息
- [ ] 样式符合微信

---

#### 任务 4.5: IPC 接口封装

**目标**: 前端调用后端接口

**文件**: `src/ipc/*.ts`

**示例**:

```typescript
// src/ipc/chat.ts
import { invoke } from '@tauri-apps/api/tauri';

export const chatAPI = {
  // 获取历史消息
  getHistory: async (sessionType: number, targetId: number, page: number) => {
    return await invoke<ChatMessage[]>('get_chat_history_handler', {
      sessionType,
      targetId,
      page,
      pageSize: 50,
    });
  },

  // 发送消息
  sendMessage: async (sessionType: number, targetId: number, content: string) => {
    return await invoke<number>('send_text_message_handler', {
      sessionType,
      targetId,
      content,
      senderUid: getCurrentUserUid(),
    });
  },
};
```

**验收标准**:

- [ ] 所有 IPC 接口封装
- [ ] TypeScript 类型正确
- [ ] 错误处理完善

---

#### 任务 4.6: 后端 IPC 接口

**目标**: 实现后端 IPC 接口

**文件**: `src/ipc/*.rs`

**示例**:

```rust
#[tauri::command]
pub async fn get_chat_history_handler(
    session_type: i8,
    target_id: i64,
    page: i32,
    page_size: i32,
    db: State<'_, DbConn>,
) -> Result<Vec<ChatMessageModel>, String> {
    let handler = ChatHandler::new(db.0.clone());
    handler
        .get_history(session_type, target_id, page, page_size)
        .await
        .map_err(|e| e.to_string())
}
```

**验收标准**:

- [ ] 所有接口实现
- [ ] 参数校验完善
- [ ] 错误转换正确

---

### Phase 5: 消息功能完善 (Week 8)

#### 任务 5.1: 消息历史分页

**目标**: 实现滚动加载历史消息

**功能**:

- 每页 50 条消息
- 滚动到顶部自动加载
- 反向追加到列表

**验收标准**:

- [ ] 分页加载正常
- [ ] 滚动流畅
- [ ] 无重复消息

---

#### 任务 5.2: 已读回执

**目标**: 实现消息已读功能

**协议**:

- 发送方: IPMSG_SENDMSG | IPMSG_READMSG
- 接收方: IPMSG_READMSG → IPMSG_ANSREADMSG

**验收标准**:

- [ ] 能发送已读回执
- [ ] 能接收并更新状态
- [ ] UI 显示已读状态

---

#### 任务 5.3: Emoji 支持

**目标**: 实现 Emoji 选择和发送

**文件**: `src/components/EmojiPicker/*`

**功能**:

- Emoji 选择器面板
- 分类显示
- 点击插入到输入框

**验收标准**:

- [ ] Emoji 面板显示
- [ ] 选择插入正常
- [ ] 发送接收正确

---

#### 任务 5.4: 消息状态管理

**目标**: 实现消息发送状态

**状态**:

- 发送中 (0)
- 已发送 (1)
- 已读 (2)
- 发送失败 (-1)

**验收标准**:

- [ ] 状态正确更新
- [ ] UI 显示状态图标
- [ ] 失败可重发

---

### Phase 6: 文件传输功能 (Week 9-10)

#### 任务 6.1: 文件请求协议

**目标**: 实现文件传输请求流程

**协议**:

1. 发送方: IPMSG_SENDMSG | IPMSG_FILEATTACHOPT
2. 附加信息包含文件头
3. 接收方: IPMSG_GETFILEDATA
4. 发送方: 分块传输

**验收标准**:

- [ ] 能发送文件请求
- [ ] 能接收并提示
- [ ] 能确认传输

---

#### 任务 6.2: 文件分块传输

**目标**: 实现文件分块传输

**参数**:

- 块大小: 4KB
- 超时: 30 秒
- 重传: 最多 3 次

**验收标准**:

- [ ] 文件正确传输
- [ ] 进度实时更新
- [ ] 校验完整性

---

#### 任务 6.3: 传输进度展示

**目标**: 实现文件传输进度 UI

**组件**: `FileProgress.tsx`

**功能**:

- 进度条
- 速度显示
- 剩余时间
- 取消按钮

**验收标准**:

- [ ] 进度条准确
- [ ] 实时更新
- [ ] 可取消传输

---

#### 任务 6.4: 断点续传

**目标**: 实现断点续传功能

**实现**:

- 记录已传输位置
- 重启后从断点继续
- 传输状态持久化

**验收标准**:

- [ ] 断点后可继续
- [ ] 状态正确恢复
- [ ] 不重新传输已完成部分

---

### Phase 7: 群聊功能 (Week 11)

#### 任务 7.1: 群组创建

**目标**: 实现群组创建功能

**流程**:

1. 选择成员
2. 创建群组（本地）
3. 通知所有成员

**验收标准**:

- [ ] 能创建群组
- [ ] 成员正确添加
- [ ] 群信息显示

---

#### 任务 7.2: 成员管理

**目标**: 实现群成员管理

**功能**:

- 添加成员
- 移除成员
- 角色管理（管理员/群主）
- 成员列表展示

**验收标准**:

- [ ] 成员添加正常
- [ ] 成员移除正常
- [ ] 权限控制正确

---

#### 任务 7.3: 群消息广播

**目标**: 实现群消息发送

**实现**:

- 遍历群成员
- 逐个发送 UDP 包
- 本地存储（session_type=1）

**验收标准**:

- [ ] 消息广播到所有成员
- [ ] 不重复发送
- [ ] 显示在群聊窗口

---

### Phase 8: 优化与测试 (Week 12)

#### 任务 8.1: 性能优化

**目标**: 优化应用性能

**项目**:

- 虚拟滚动（消息列表）
- 消息分页加载
- 图片懒加载
- 数据库查询优化

**验收标准**:

- [ ] 1000+ 消息流畅滚动
- [ ] 启动时间 < 2 秒
- [ ] 内存占用合理

---

#### 任务 8.2: 单元测试

**目标**: 完善单元测试覆盖

**覆盖率要求**: > 80%

**重点模块**:

- 协议解析器
- 数据库 handler
- 业务逻辑

**验收标准**:

- [ ] 覆盖率达标
- [ ] 所有测试通过
- [ ] 无 clippy 警告

---

#### 任务 8.3: 集成测试

**目标**: 端到端测试

**场景**:

- 两机通信
- 消息收发
- 文件传输
- 群聊

**验收标准**:

- [ ] 所有场景通过
- [ ] 无崩溃
- [ ] 稳定运行

---

#### 任务 8.4: 跨平台测试

**目标**: 多平台兼容性测试

**平台**:

- Windows 10/11
- macOS 10.15+
- Ubuntu 20.04+

**验收标准**:

- [ ] 各平台功能一致
- [ ] UI 正常显示
- [ ] 无平台相关 bug

---

## 四、技术实施规范

### 4.1 代码规范

**Rust 代码**:

- 遵循 Rust API Guidelines
- 使用 `cargo fmt` 格式化
- 通过 `cargo clippy` 检查
- 单文件不超过 500 行

**TypeScript 代码**:

- 遵循 Airbnb Style Guide
- 使用 ESLint + Prettier
- 严格类型检查
- 组件单一职责

### 4.2 Git 工作流

**分支策略**:

```
main          - 稳定版本
develop       - 开发分支
feature/*     - 功能分支
fix/*         - 修复分支
```

**提交规范**:

```
feat: 添加功能
fix: 修复 bug
docs: 文档更新
style: 代码格式
refactor: 重构
test: 测试相关
chore: 构建/工具
```

### 4.3 测试要求

**单元测试**: 每个模块必须有测试

**集成测试**: 关键流程必须有集成测试

**覆盖率**: 核心代码 > 80%

### 4.4 文档要求

**代码注释**: 公共 API 必须有文档注释

**README**: 项目说明、快速开始

**CHANGELOG**: 版本更新记录

---

## 五、验收标准

### 5.1 功能验收

| 功能     | 验收标准                         |
| -------- | -------------------------------- |
| 用户发现 | 两台内网机器能互相发现           |
| 单聊     | 能发送接收文字消息，消息正确存储 |
| 群聊     | 能创建群组，群消息正确广播       |
| 文件传输 | 能传输 10MB 文件，支持断点续传   |
| UI       | 界面与微信高度相似，交互流畅     |

### 5.2 性能验收

| 指标     | 要求               |
| -------- | ------------------ |
| 消息延迟 | < 200ms            |
| 并发用户 | 支持 100+ 在线用户 |
| 启动时间 | < 2 秒             |
| 内存占用 | < 200MB            |
| 应用体积 | < 20MB             |

### 5.3 稳定性验收

| 项目     | 标准          |
| -------- | ------------- |
| 崩溃率   | < 0.1%        |
| 内存泄漏 | 无            |
| 长期运行 | 7×24 小时稳定 |

### 5.4 代码质量

| 指标           | 要求  |
| -------------- | ----- |
| 单元测试覆盖率 | > 80% |
| Clippy 警告    | 0     |
| ESLint 错误    | 0     |
| 代码重复率     | < 5%  |

---

## 六、风险控制

### 6.1 技术风险

| 风险       | 影响       | 应对措施            |
| ---------- | ---------- | ------------------- |
| UDP 丢包   | 消息丢失   | ACK + 超时重传      |
| 跨平台兼容 | 功能不一致 | 早期跨平台测试      |
| 性能瓶颈   | 卡顿       | 异步处理 + 虚拟滚动 |
| 协议兼容性 | 无法通信   | 严格遵循 IPMsg 规范 |

### 6.2 进度风险

| 风险     | 应对措施                   |
| -------- | -------------------------- |
| 需求变更 | 锁定核心需求，扩展功能延后 |
| 技术难题 | 提前技术验证，准备备选方案 |
| 人员变动 | 代码文档化，知识共享       |
| 时间不足 | 优先 P0 功能，P1 功能迭代  |

### 6.3 质量风险

| 风险     | 应对措施               |
| -------- | ---------------------- |
| Bug 漏测 | 完善测试用例，代码审查 |
| 性能问题 | 早期性能测试，持续监控 |
| 安全漏洞 | 安全审计，输入校验     |

---

## 附录

### A. 依赖版本

```toml
[dependencies]
tauri = "2.0"
tokio = { version = "1.35", features = ["full"] }
sea-orm = { version = "0.12", features = ["sqlx-sqlite"] }
combine = "4.6"
crossbeam-channel = "0.5"
once_cell = "1.19"
serde = { version = "1.0", features = ["derive"] }
tracing = "0.1"
anyhow = "1.0"
thiserror = "1.0"
```

### B. 参考资料

- [Tauri 官方文档](https://tauri.app/)
- [IPMsg 协议规范](https://ipmsg.org/)
- [SeaORM 文档](https://www.sea-ql.org/SeaORM/)
- [combine 文档](https://docs.rs/combine/)

### C. 联系方式

- 项目仓库: [GitHub]
- 问题反馈: [Issues]
- 文档: [Wiki]

---

**计划版本**: v1.0.0
**最后更新**: 2025-01-27
**负责人**: 开发团队
