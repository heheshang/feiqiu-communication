import { useState, useEffect, useCallback } from 'react';
import { useUser } from '../../../hooks/useUser';
import { useContact } from '../../../hooks/useContact';
import { useChat } from '../../../hooks/useChat';
import { useUserStore } from '../../../store';
import type { UserInfo, ChatMessage } from '../../../types';

export interface LayoutState {
  selectedUser: UserInfo | null;
  selectedSessionId: number | null;
  viewMode: 'normal' | 'chat' | 'contact';
  activeTab: 'chats' | 'groups';
  selectedGroupId: number | null;
}

export function useLayoutState() {
  const { currentUser, getCurrentUser } = useUser();
  const { onlineUsers } = useContact();
  const { initialize } = useUserStore();

  const [layoutState, setLayoutState] = useState<LayoutState>({
    selectedUser: null,
    selectedSessionId: null,
    viewMode: 'normal',
    activeTab: 'chats',
    selectedGroupId: null,
  });

  const [createGroupDialogOpen, setCreateGroupDialogOpen] = useState(false);

  const { messages, loadSessionMessages, selectSession, retryMessage, sendFileMessage, sessions } =
    useChat();

  useEffect(() => {
    initialize();
  }, [initialize]);

  useEffect(() => {
    if (!currentUser) {
      getCurrentUser();
    }
  }, [currentUser, getCurrentUser]);

  const handleSessionSelect = useCallback(
    (sessionId: number, userId: number) => {
      const session = sessions.find((s) => s.sid === sessionId);
      if (!session) {
        console.warn('Session not found:', sessionId);
        return;
      }

      const targetUser = onlineUsers.find((u) => u.uid === userId);
      if (!targetUser) {
        console.warn('User not found:', userId);
        return;
      }

      selectSession(session);

      setLayoutState({
        selectedUser: targetUser,
        selectedSessionId: sessionId,
        viewMode: 'chat',
        activeTab: 'chats',
        selectedGroupId: null,
      });
    },
    [sessions, onlineUsers, selectSession]
  );

  const handleUserSelect = useCallback((user: UserInfo) => {
    setLayoutState({
      selectedUser: user,
      selectedSessionId: null,
      viewMode: 'chat',
      activeTab: 'chats',
      selectedGroupId: null,
    });
  }, []);

  const handleBackToList = useCallback(() => {
    setLayoutState((prev) => ({
      ...prev,
      selectedUser: null,
      selectedSessionId: null,
      viewMode: 'normal',
      selectedGroupId: null,
    }));
  }, []);

  const handleLoadMore = useCallback(() => {
    if (layoutState.selectedUser) {
      loadSessionMessages(0, layoutState.selectedUser.uid);
    }
  }, [layoutState.selectedUser, loadSessionMessages]);

  const handleRetryMessage = useCallback(
    (message: ChatMessage) => {
      retryMessage(message);
    },
    [retryMessage]
  );

  const handleSendFile = useCallback(
    async (file: File) => {
      if (!layoutState.selectedUser) return;

      try {
        await sendFileMessage(0, layoutState.selectedUser.uid, file.name, file.name);
      } catch (error) {
        console.error('发送文件失败:', error);
        throw error;
      }
    },
    [layoutState.selectedUser, sendFileMessage]
  );

  const handleTabChange = useCallback((tab: 'chats' | 'groups') => {
    setLayoutState({
      selectedUser: null,
      selectedSessionId: null,
      viewMode: 'normal',
      activeTab: tab,
      selectedGroupId: null,
    });
  }, []);

  const handleGroupSelect = useCallback((groupId: number) => {
    setLayoutState((prev) => ({
      ...prev,
      selectedGroupId: groupId,
      selectedUser: null,
      selectedSessionId: null,
      viewMode: 'chat',
    }));
  }, []);

  const handleCreateGroupOpen = useCallback(() => {
    setCreateGroupDialogOpen(true);
  }, []);

  const handleCreateGroupClose = useCallback(() => {
    setCreateGroupDialogOpen(false);
  }, []);

  const handleGroupDeleted = useCallback(() => {
    setLayoutState((prev) => ({ ...prev, selectedGroupId: null }));
  }, []);

  const showBackButton = layoutState.viewMode !== 'normal';

  return {
    layoutState,
    createGroupDialogOpen,
    showBackButton,
    currentUser,
    onlineUsers,
    messages,
    sessions,
    handleSessionSelect,
    handleUserSelect,
    handleBackToList,
    handleLoadMore,
    handleRetryMessage,
    handleSendFile,
    handleTabChange,
    handleGroupSelect,
    handleCreateGroupOpen,
    handleCreateGroupClose,
    handleGroupDeleted,
  };
}
