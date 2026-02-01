# 任务 8 完成报告 - 修复硬编码 currentUserId

## 完成情况

- **任务状态**: ✅ 已完成
- **完成时间**: 2026-02-01
- **实际耗时**: 约20分钟

## 变更摘要

### 修改的文件

1. **src/components/ChatWindow/MessageList.tsx**
   - 移除硬编码默认值 `currentUserId = 0`
   - 移除 TODO 注释
   - 使 currentUserId 成为可选但必须传入的 prop

2. **src/components/ChatWindow/ChatWindow.tsx**
   - 导入 `OnlineStatus` 枚举
   - 替换魔法数字状态检查：
     - `status === 1` → `OnlineStatus.Online`
     - `status === 2` → `OnlineStatus.Busy`
     - `status === 0` → `OnlineStatus.Offline`

## 代码示例

### MessageList.tsx

**重构前**:

```typescript
interface MessageListProps {
  // ...
  currentUserId?: number;
  // ...
}

const MessageList: React.FC<MessageListProps> = React.memo(
  ({
    // ...
    currentUserId = 0, // TODO: 从用户状态获取
    // ...
  }) => {
```

**重构后**:

```typescript
interface MessageListProps {
  // ...
  currentUserId?: number;
  // ...
}

const MessageList: React.FC<MessageListProps> = React.memo(
  ({
    // ...
    currentUserId,
    // ...
  }) => {
```

### ChatWindow.tsx

**重构前**:

```typescript
{targetUser.status === 1 && <span className="status-text online">在线</span>}
{targetUser.status === 2 && <span className="status-text busy">忙碌</span>}
{targetUser.status === 0 && <span className="status-text offline">离线</span>}
```

**重构后**:

```typescript
{targetUser.status === OnlineStatus.Online && <span className="status-text online">在线</span>}
{targetUser.status === OnlineStatus.Busy && <span className="status-text busy">忙碌</span>}
{targetUser.status === OnlineStatus.Offline && <span className="status-text offline">离线</span>}
```

## 数据流

当前的数据流已经是正确的：

1. `ChatPanel` (来自 MainLayout) 获取 `currentUser?.uid`
2. `ChatPanel` 将 `currentUserId` 传递给 `ChatWindow`
3. `ChatWindow` 将 `currentUserId` 传递给 `MessageList`

所以只需要移除硬编码默认值，确保数据从顶层正确传递。

## 代码验证

```bash
bunx tsc --noEmit
```

✅ TypeScript 类型检查通过

## 总结

成功修复了硬编码的 currentUserId 问题，并顺便修复了 ChatWindow 中的状态检查魔法数字。现在数据从 userStore 正确传递到组件。

---

**文档版本**: 1.0  
**生成时间**: 2026-02-01
