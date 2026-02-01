# 任务文档: 拆分 main.rs 模块

## 任务信息

- **任务ID**: 1
- **优先级**: 高
- **预计时间**: 2-3小时
- **影响文件**: 6个新文件 + main.rs 重构

## 当前问题

`main.rs` 包含 431 行代码，违反单一职责原则：

- 应用初始化逻辑 (init_app_internal: 50行)
- 后台服务启动 (start_background_services: 46行)
- 网络事件处理 (handle_network_event: 77行)
- Tauri 命令注册 (main 函数: 80行)
- 辅助函数 (日志、网络信息、计算机名)

## 重构方案

### 新模块结构

```
src-tauri/src/
├── main.rs                    (60行 - 仅入口)
├── app/
│   ├── mod.rs                 (模块导出)
│   ├── init.rs                (应用初始化: 120行)
│   ├── commands.rs            (Tauri命令: 20行)
│   └── setup.rs               (应用设置: 60行)
├── event/
│   ├── mod.rs                 (已有)
│   ├── bus.rs                 (已有)
│   ├── model.rs               (已有)
│   └── handlers.rs    NEW     (事件处理器: 100行)
└── lib.rs                     (修改 - 添加 app 模块)
```

### 代码迁移计划

#### 1. app/init.rs (从 main.rs 迁移)

```rust
// 包含:
// - init_app_internal() → app_init()
// - init_logging()
// - ensure_current_user_exists()
// - start_background_services()
// - get_local_network_info()
// - get_computer_name()
```

#### 2. event/handlers.rs (新建)

```rust
// 包含:
// - handle_network_event() → 拆分为子函数
// - handle_ui_event()
// - 每个 NetworkEvent 变体一个处理函数
```

#### 3. app/setup.rs (新建)

```rust
// 包含:
// - setup() 回调函数中的初始化逻辑
// - 通道创建和等待逻辑
```

#### 4. app/commands.rs (新建)

```rust
// 包含:
// - get_version() 命令
// - 命令导出宏
```

## 实施步骤

### 步骤 1: 创建 event/handlers.rs

- 迁移 handle_network_event 和 handle_ui_event
- 将网络事件处理拆分为独立函数

### 步骤 2: 创建 app/init.rs

- 迁移所有初始化相关函数
- 保持原有逻辑不变

### 步骤 3: 创建 app/setup.rs

- 提取 main() 中的 setup 逻辑

### 步骤 4: 创建 app/commands.rs

- 迁移 get_version 命令

### 步骤 5: 创建 app/mod.rs

- 导出所有子模块

### 步骤 6: 重构 main.rs

- 简化为入口文件
- 使用新模块

### 步骤 7: 更新 lib.rs

- 添加 app 模块声明

### 步骤 8: 验证

- cargo check
- cargo clippy
- cargo test

## 详细设计

### event/handlers.rs 结构

```rust
pub async fn handle_network_event(event: NetworkEvent, db: &Arc<DbConn>) {
    match event {
        NetworkEvent::UserOnline { ... } => handle_user_online(...).await,
        NetworkEvent::UserOffline { ... } => handle_user_offline(...).await,
        // ... 每个变体一个处理函数
    }
}

async fn handle_user_online(ip: String, port: u16, ...) {
    info!("用户上线: {} ({}:{})", nickname, ip, port);
    // 处理逻辑
}
```

### app/init.rs 结构

```rust
pub async fn init_app(app_handle: &AppHandle) -> Result<Arc<DbConn>, Box<dyn Error>> {
    // 初始化逻辑
}

pub async fn start_services(app_handle: AppHandle, db: Arc<DbConn>) {
    // 启动后台服务
}

pub fn init_logging() { ... }
pub async fn ensure_user_exists(db: &DbConn) -> Result<(), String> { ... }
```

## 风险与缓解

| 风险     | 缓解措施                       |
| -------- | ------------------------------ |
| 编译错误 | 小步重构，每次验证 cargo check |
| 逻辑丢失 | 保持原有逻辑，仅移动代码位置   |
| 循环依赖 | 仔细检查模块间的依赖关系       |

## 验收标准

- [ ] main.rs 行数 < 100行
- [ ] cargo check 通过
- [ ] cargo clippy 无警告
- [ ] cargo test 通过
- [ ] 应用正常启动

## 变更记录

| 日期       | 变更         | 作者       |
| ---------- | ------------ | ---------- |
| 2026-02-01 | 创建任务文档 | Hephaestus |
