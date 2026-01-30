# 群聊功能实现计划

**创建日期**: 2026-01-30
**状态**: Planning
**预计工期**: 2-3 days

---

## 一、当前状态分析

### ✅ 已完成

**后端基础设施:**

1. ✅ 数据库模型 (Group, GroupMember)
2. ✅ 数据库处理器 (GroupHandler, GroupMemberHandler)
3. ✅ 群组消息广播器 (GroupBroadcaster)
4. ✅ IPC 命令 (7个命令)
5. ✅ 前端 API 封装 (groupAPI)
6. ✅ ChatService 支持群聊 (session_type = 1)

### ❌ 待完成

**后端架构问题:**

1. ❌ GroupService 所有方法都是 `todo!()` (Phase 1 骨架未实现)
2. ❌ IPC 层直接调用 Handler (违反重构架构，应调用 GroupService)

**前端缺失:**

1. ❌ 群组列表组件
2. ❌ 群聊界面组件
3. ❌ 创建群组对话框
4. ❌ 成员管理界面

---

## 二、实施计划

### Phase 1: 后端 GroupService 实现 (Day 1)

**任务 1.1: 实现 GroupService 所有方法**

文件: `src-tauri/src/core/group/service.rs`

需要实现的方法:

- `create_group()` - 创建群组 (包含添加成员逻辑)
- `get_groups()` - 获取用户群组列表
- `add_member()` - 添加成员
- `remove_member()` - 移除成员
- `get_members()` - 获取成员列表
- `update_group()` - 更新群组信息
- `delete_group()` - 删除群组

**实现模式** (参考 ChatService):

```rust
pub async fn create_group(
    db: &DbConn,
    group_name: String,
    creator_uid: i64,
    desc: Option<String>,
    avatar: Option<String>,
) -> AppResult<i64> {
    // 1. 调用 GroupHandler::create()
    // 2. 返回群组ID
}
```

**任务 1.2: 重构 IPC 层使用 GroupService**

文件: `src-tauri/src/ipc/group.rs`

重构前:

```rust
pub async fn create_group_handler(...) -> Result<i64, String> {
    let group = GroupHandler::create(...).await.map_err_to_frontend()?;
    // ...
}
```

重构后:

```rust
pub async fn create_group_handler(...) -> Result<i64, String> {
    GroupService::create_group(db.inner(), group_name, creator_uid, desc, avatar)
        .await
        .map_err_to_frontend()
}
```

**验证:**

- [ ] `cargo check` 通过
- [ ] `cargo test` 通过
- [ ] 所有 7 个 IPC 命令已重构

---

### Phase 2: 前端群组列表组件 (Day 1-2)

**任务 2.1: 创建群组服务层**

文件: `src/services/groupService.ts`

```typescript
export const groupService = {
  async getGroups(userUid: number) {
    return await groupAPI.getUserGroups(userUid);
  },
  async createGroup(name: string, creatorUid: number, memberUids: number[]) {
    return await groupAPI.createGroup(name, creatorUid, memberUids);
  },
  // ... 其他方法
};
```

**任务 2.2: 创建群组 Store**

文件: `src/store/groupStore.ts`

使用 Zustand 管理群组状态:

- 群组列表
- 当前群组
- 成员列表
- UI 状态 (创建对话框、成员管理等)

**任务 2.3: 创建群组列表组件**

文件: `src/components/GroupList.tsx`

功能:

- 显示用户加入的所有群组
- 显示群组头像、名称
- 显示未读消息数
- 点击切换到群聊

**任务 2.4: 创建群组卡片组件**

文件: `src/components/GroupCard.tsx`

显示群组信息:

- 群组名称
- 成员数量
- 最后消息预览

---

### Phase 3: 创建群组功能 (Day 2)

**任务 3.1: 创建创建群组对话框**

文件: `src/components/CreateGroupDialog.tsx`

功能:

- 输入群组名称
- 选择联系人作为成员
- 确认创建
- 联系人多选组件

**任务 3.2: 集成到主界面**

在侧边栏添加"创建群组"按钮，点击打开对话框。

---

### Phase 4: 群聊界面 (Day 2-3)

**任务 4.1: 创建群聊窗口组件**

文件: `src/components/GroupChatWindow.tsx`

功能:

- 显示群组名称、成员数量
- 消息列表 (复用现有 ChatMessage 组件)
- 输入框和发送按钮
- 成员列表抽屉

**任务 4.2: 创建成员列表组件**

文件: `src/components/GroupMemberList.tsx`

功能:

- 显示所有成员
- 显示成员角色 (所有者/管理员/普通成员)
- 管理员可以移除成员
- 显示成员在线状态

**任务 4.3: 集成到聊天界面**

修改 `src/pages/Chat.tsx`:

- 根据 `sessionType` 切换单聊/群聊界面
- 群聊使用 `GroupChatWindow`
- 单聊使用现有 `ChatWindow`

---

### Phase 5: 成员管理功能 (Day 3)

**任务 5.1: 创建添加成员对话框**

文件: `src/components/AddMemberDialog.tsx`

功能:

- 显示可添加的联系人 (不在群组中的用户)
- 多选成员
- 确认添加

**任务 5.2: 创建成员管理菜单**

右键点击群组成员显示菜单:

- 查看资料
- 设置为管理员/移除管理员 (群主)
- 移除出群 (群主/管理员)

---

## 三、测试计划

### 后端测试

**单元测试** (`src-tauri/src/core/group/service.rs`):

- [ ] 测试创建群组
- [ ] 测试添加成员
- [ ] 测试移除成员
- [ ] 测试获取群组列表
- [ ] 测试群组更新和删除

### 前端测试

**组件测试**:

- [ ] GroupList 渲染测试
- [ ] CreateGroupDialog 交互测试
- [ ] GroupChatWindow 消息发送测试

### 集成测试

**端到端测试**:

- [ ] 创建群组流程
- [ ] 添加成员流程
- [ ] 发送群消息流程
- [ ] 管理成员流程

---

## 四、实施优先级

### P0 (核心功能 - Day 1-2)

1. ✅ GroupService 实现
2. ✅ IPC 层重构
3. ✅ 群组列表组件
4. ✅ 创建群组对话框
5. ✅ 群聊界面

### P1 (增强功能 - Day 3)

1. 成员管理界面
2. 添加成员对话框
3. 成员角色管理

### P2 (优化 - 可延后)

1. 群组设置界面
2. 群组公告
3. @成员功能
4. 群组头像上传

---

## 五、技术考虑

### 后端架构

**职责分离:**

- **IPC 层**: 参数转换 + 错误映射 (薄层)
- **Service 层**: 业务逻辑封装
- **Handler 层**: 数据库 CRUD

**群组消息广播:**

- 使用现有的 `GroupBroadcaster`
- 遍历所有在线成员并发送 UDP 消息
- 跳过发送者自己

### 前端架构

**状态管理:**

- 使用 Zustand (与现有架构一致)
- 分离群组状态和聊天状态

**组件复用:**

- 复用 `ChatMessage` 组件显示消息
- 复用 `MessageInput` 组件输入框
- 新增群组特定组件

---

## 六、风险评估

### 技术风险

1. **群组消息可靠性**
   - 风险: UDP 广播可能丢失
   - 缓解: 实现消息重传机制

2. **成员状态同步**
   - 风险: 成员上下线时状态可能不一致
   - 缓解: 使用事件总线实时更新

### 实施风险

1. **时间估算**
   - 风险: 可能超出预期工期
   - 缓解: 按 P0/P1 优先级分阶段实施

2. **测试覆盖**
   - 风险: 多设备测试困难
   - 缓解: 侧重单元测试和模拟测试

---

## 七、成功标准

### 功能完整性

- [ ] 可以创建群组
- [ ] 可以添加/移除成员
- [ ] 可以发送群消息
- [ ] 可以查看群组列表
- [ ] 可以查看成员列表

### 代码质量

- [ ] 所有 IPC 命令使用 GroupService
- [ ] TypeScript 编译无错误
- [ ] 后端单元测试通过
- [ ] 前端组件测试通过

### 用户体验

- [ ] 界面响应流畅
- [ ] 错误提示友好
- [ ] 群组消息实时送达

---

## 八、后续优化

1. **性能优化**
   - 群组消息分页加载
   - 成员列表虚拟化

2. **功能增强**
   - 群组公告
   - @成员提醒
   - 群组文件共享
   - 群组投票

3. **用户体验**
   - 群组搜索
   - 群组分类
   - 消息免打扰
