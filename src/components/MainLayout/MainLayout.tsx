import React from 'react';
import { CreateGroupDialog } from '../CreateGroupDialog';
import { useLayoutState } from './hooks/useLayoutState';
import { Sidebar } from './components/Sidebar';
import { ContactPanel } from './components/ContactPanel';
import { ChatPanel } from './components/ChatPanel';
import './MainLayout.less';

const MainLayout: React.FC = () => {
  const {
    layoutState,
    createGroupDialogOpen,
    showBackButton,
    currentUser,
    onlineUsers,
    messages,
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
  } = useLayoutState();

  return (
    <div className={`main-layout ${layoutState.viewMode}`}>
      <Sidebar
        activeTab={layoutState.activeTab}
        selectedUserId={layoutState.selectedUser?.uid}
        showBackButton={showBackButton}
        onBackToList={handleBackToList}
        onTabChange={handleTabChange}
        onSessionSelect={handleSessionSelect}
        onCreateGroupOpen={handleCreateGroupOpen}
        onSelectGroup={handleGroupSelect}
      />

      <ContactPanel
        users={onlineUsers}
        showBackButton={showBackButton}
        onBackToList={handleBackToList}
        onUserClick={handleUserSelect}
      />

      <ChatPanel
        activeTab={layoutState.activeTab}
        selectedUser={layoutState.selectedUser || undefined}
        selectedGroupId={layoutState.selectedGroupId}
        currentUserId={currentUser?.uid}
        messages={Object.values(messages).flat()}
        onLoadMore={handleLoadMore}
        onRetryMessage={handleRetryMessage}
        onSendFile={handleSendFile}
        onGroupDeleted={handleGroupDeleted}
      />

      <CreateGroupDialog
        isOpen={createGroupDialogOpen}
        onClose={handleCreateGroupClose}
        availableMembers={onlineUsers.map((u) => ({
          id: u.uid.toString(),
          name: u.nickname,
        }))}
      />
    </div>
  );
};

export default MainLayout;
