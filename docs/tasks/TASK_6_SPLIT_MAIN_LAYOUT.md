# 任务文档: 拆分 MainLayout 组件

## 任务信息

- **任务ID**: 6
- **优先级**: 高
- **预计时间**: 2小时
- **影响文件**: 5个新文件 + MainLayout.tsx 重构

## 当前问题

`MainLayout.tsx` 包含 271 行代码，组件过于复杂：

- 6 个 useState 调用
- 2 个 useEffect 调用
- 11 个事件处理函数
- 复杂的条件渲染逻辑
- 混合了布局、状态管理和事件处理

## 重构方案

### 新文件结构

```
src/components/MainLayout/
├── MainLayout.tsx           (80行 - 仅布局和状态协调)
├── hooks/
│   └── useLayoutState.ts    (120行 - 状态管理逻辑)
├── components/
│   ├── Sidebar.tsx          (60行 - 左侧栏)
│   ├── ContactPanel.tsx     (30行 - 中间通讯录)
│   └── ChatPanel.tsx        (50行 - 右侧聊天窗口)
└── MainLayout.less          (已有)
```

### 代码迁移计划

#### 1. hooks/useLayoutState.ts (新建)

```typescript
// 从 MainLayout.tsx 提取：
// - LayoutState 接口
// - layoutState state
// - createGroupDialogOpen state
// - 所有处理函数：
//   handleSessionSelect, handleUserSelect, handleBackToList
//   handleLoadMore, handleRetryMessage, handleSendFile
//   handleTabChange, handleGroupSelect
//   handleCreateGroupOpen, handleCreateGroupClose
// - showBackButton 计算属性
```

#### 2. components/Sidebar.tsx (新建)

负责：

- 返回按钮渲染
- 标签页切换按钮
- 条件渲染 SessionList 或 GroupList

#### 3. components/ContactPanel.tsx (新建)

负责：

- 返回按钮渲染
- ContactList 渲染

#### 4. components/ChatPanel.tsx (新建)

负责：

- 条件渲染 ChatWindow 或 GroupChatWindow

#### 5. MainLayout.tsx (重构)

负责：

- 使用 useLayoutState hook
- 组合 Sidebar、ContactPanel、ChatPanel
- 保持原有接口和 props

## 实施步骤

### 步骤 1: 创建 hooks/useLayoutState.ts

提取所有状态管理逻辑

### 步骤 2: 创建 components/Sidebar.tsx

提取左侧栏逻辑

### 步骤 3: 创建 components/ContactPanel.tsx

提取中间通讯录

### 步骤 4: 创建 components/ChatPanel.tsx

提取右侧聊天窗口

### 步骤 5: 重构 MainLayout.tsx

简化为组合组件

### 步骤 6: 验证

- TypeScript 类型检查
- ESLint 检查
- 功能测试

## 详细设计

### useLayoutState.ts 结构

```typescript
export interface LayoutState {
  selectedUser: UserInfo | null;
  selectedSessionId: number | null;
  viewMode: 'normal' | 'chat' | 'contact';
  activeTab: 'chats' | 'groups';
  selectedGroupId: number | null;
}

export function useLayoutState() {
  const [layoutState, setLayoutState] = useState<LayoutState>(...);
  const [createGroupDialogOpen, setCreateGroupDialogOpen] = useState(false);

  // 从 useChat, useUser, useContact 获取数据
  const { ... } = useChat();
  const { ... } = useUser();
  const { ... } = useContact();

  // 处理函数
  const handleSessionSelect = useCallback((sessionId, userId) => { ... }, [...]);
  const handleUserSelect = useCallback((user) => { ... }, [...]);
  // ... 其他处理函数

  const showBackButton = layoutState.viewMode !== 'normal';

  return {
    layoutState,
    createGroupDialogOpen,
    showBackButton,
    // 处理函数
    handleSessionSelect,
    handleUserSelect,
    // ...
  };
}
```

## 风险与缓解

| 风险         | 缓解措施                      |
| ------------ | ----------------------------- |
| 破坏现有功能 | 保持原有接口和 props 不变     |
| 类型错误     | 完整 TypeScript 类型定义      |
| 性能下降     | 使用 useCallback 缓存处理函数 |
| 状态不同步   | 确保 state 依赖关系正确       |

## 验收标准

- [ ] MainLayout.tsx 行数 < 100行
- [ ] TypeScript 类型检查通过
- [ ] ESLint 无警告
- [ ] 应用功能正常
- [ ] 移动端返回按钮工作正常

## 兼容性

- 向后兼容：所有 props 和回调保持不变
- 无破坏性变更
- 组件接口不变

## 总结

本次重构将 271 行的 MainLayout 拆分为 5 个更小的文件，每个职责单一，提高可维护性和可测试性。

---

**文档版本**: 1.0  
**生成时间**: 2026-02-01
