# 事件系统重构设计文档

## 1. 当前架构分析

### 事件发布流程

- UDP接收器收到数据包
- 解析为ProtocolPacket
- 序列化为JSON字符串
- 发布NetworkEvent::PacketReceived { packet: String, addr: String }

### 订阅者处理流程

- 订阅PacketReceived事件
- 反序列化JSON字符串为ProtocolPacket
- 检查命令类型（base_command()）
- 根据命令类型执行不同逻辑

### 痛点和问题

1. 订阅者需要反序列化JSON字符串
2. 订阅者需要判断命令类型
3. 每个订阅者都要重复解析逻辑
4. 增加订阅者复杂度

## 2. 新架构设计

### 细粒度事件类型定义

根据IPMsg/FeiQ协议命令，设计以下细粒度事件：

- UserOnline - 用户上线（IPMSG_BR_ENTRY）
- UserOffline - 用户下线（IPMSG_BR_EXIT）
- UserPresenceResponse - 在线应答（IPMSG_ANSENTRY）
- MessageReceived - 收到消息（IPMSG_SENDMSG）
- MessageReceiptReceived - 收到确认（IPMSG_RECVMSG）
- MessageRead - 消息已读（IPMSG_READMSG）
- MessageDeleted - 消息删除（IPMSG_DELMSG）
- FileRequestReceived - 文件请求（IPMSG_FILEATTACHOPT）

### 事件发布流程

- UDP接收器收到数据包
- 解析为ProtocolPacket
- 提取关键字段（用户信息、消息内容等）
- 根据命令类型发布对应的细粒度事件

### 订阅者简化流程

- 订阅感兴趣的具体事件类型
- 直接使用事件中的字段
- 无需解析和判断

## 3. 事件类型清单

### NetworkEvent（新增）

```rust
pub enum NetworkEvent {
    // 用户上线
    UserOnline {
        ip: String,
        port: u16,
        nickname: String,
        hostname: Option<String>,
        mac_addr: Option<String>,
    },

    // 用户下线
    UserOffline {
        ip: String,
    },

    // 在线应答
    UserPresenceResponse {
        ip: String,
        port: u16,
        nickname: String,
        hostname: Option<String>,
    },

    // 收到消息
    MessageReceived {
        sender_ip: String,
        sender_port: u16,
        sender_nickname: String,
        content: String,
        msg_no: String,
        needs_receipt: bool,
    },

    // 收到确认
    MessageReceiptReceived {
        msg_no: String,
    },

    // 消息已读
    MessageRead {
        msg_no: String,
    },

    // 消息删除
    MessageDeleted {
        msg_no: String,
    },

    // 文件请求
    FileRequestReceived {
        from_ip: String,
        files: Vec<FileInfo>,
    },

    // 保留：原始数据包（用于向后兼容或调试）
    #[allow(dead_code)]
    PacketReceived {
        packet: String,
        addr: String,
    },
}
```

### 与协议命令的映射

- IPMSG_BR_ENTRY (0x01) -> UserOnline
- IPMSG_BR_EXIT (0x02) -> UserOffline
- IPMSG_ANSENTRY (0x03) -> UserPresenceResponse
- IPMSG_SENDMSG (0x20) -> MessageReceived
- IPMSG_RECVMSG (0x21) -> MessageReceiptReceived
- IPMSG_READMSG (0x30) -> MessageRead
- IPMSG_DELMSG (0x31) -> MessageDeleted
- IPMSG_FILEATTACHOPT -> FileRequestReceived

## 4. 向后兼容策略

### 保留PacketReceived事件

- 保留PacketReceived事件用于调试
- 标记为#[allow(dead_code)]表示可选
- 新代码优先使用细粒度事件

### 订阅者迁移步骤

1. 添加新的订阅分支处理细粒度事件
2. 验证新逻辑正确
3. 移除旧的PacketReceived处理逻辑
4. 更新测试

### 迁移时间表

- 第一阶段：添加细粒度事件，PacketReceived继续发布
- 第二阶段：订阅者逐步迁移到细粒度事件
- 第三阶段：移除PacketReceived处理逻辑

## 5. 实施计划

### 步骤1：添加新事件类型定义

修改 src-tauri/src/event/model.rs：

- 在NetworkEvent中添加新的细粒度事件变体
- 保留PacketReceived用于向后兼容

### 步骤2：修改UDP接收器发布逻辑

修改 src-tauri/src/network/udp/receiver.rs：

- 解析ProtocolPacket的命令类型
- 提取关键字段（用户信息、消息内容等）
- 根据命令类型发布对应的细粒度事件

### 步骤3：更新现有订阅者

修改订阅者文件：

- src-tauri/src/core/chat/receiver.rs（订阅MessageReceived）
- src-tauri/src/core/chat/receipt.rs（订阅MessageRead/MessageDeleted）
- 移除PacketReceived的处理逻辑

### 步骤4：添加单元测试

在 src-tauri/src/event/ 下创建测试：

- 测试每个细粒度事件的发布和订阅
- 测试事件字段的正确性

### 步骤5：清理旧代码

- 移除PacketReceived的订阅者
- 更新文档

## 6. 风险评估

### 潜在问题

1. **事件字段遗漏**：某些协议命令可能包含额外信息
   - 缓解：保留PacketReceived事件作为fallback
2. **订阅者遗漏**：某些订阅者可能被遗漏
   - 缓解：使用grep搜索所有EVENT_SENDER和EVENT_RECEIVER使用

3. **测试覆盖不足**：新事件类型可能缺少测试
   - 缓解：逐步迁移，每个迁移都验证测试通过

4. **性能影响**：解析packet增加CPU开销
   - 缓解：解析开销很小，且减少了订阅者的重复解析

### 缓解措施

- 保留PacketReceived事件作为安全网
- 每个步骤都运行cargo test验证
- 使用grep搜索所有事件相关代码

### 回滚计划

如果新架构出现问题：

1. 恢复UDP接收器只发布PacketReceived
2. 订阅者恢复使用PacketReceived
3. 移除细粒度事件类型

## 7. 总结

细粒度事件重构将：

- ✅ 减少订阅者复杂度
- ✅ 提高代码可维护性
- ✅ 保持向后兼容性
- ✅ 支持渐进式迁移

实施时间估计：2-3小时
