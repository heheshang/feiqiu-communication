use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create chat_message table
        manager
            .create_table(
                Table::create()
                    .table(ChatMessage::Table)
                    .col(pk_auto(ChatMessage::Mid))
                    .col(Integer(ChatMessage::SenderUid))
                    .col(Integer(ChatMessage::ReceiverUid))
                    .col(String(ChatMessage::Content))
                    .col(TinyInt(ChatMessage::MessageType))
                    .col(TinyInt(ChatMessage::Status))
                    .col(TimeTimestamp(ChatMessage::SendTime))
                    .col(TimeTimestamp(ChatMessage::CreateTime))
                    .foreign_key(
                        ForeignKey::new()
                            .from(ChatMessage::Table, ChatMessage::SenderUid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create chat_session table
        manager
            .create_table(
                Table::create()
                    .table(ChatSession::Table)
                    .col(pk_auto(ChatSession::SessionId))
                    .col(Integer(ChatSession::OwnerUid))
                    .col(TinyInt(ChatSession::SessionType))
                    .col(Integer(ChatSession::TargetId))
                    .col(Integer(ChatSession::LastMessageId))
                    .col(Integer(ChatSession::UnreadCount))
                    .col(TimeTimestamp(ChatSession::UpdateTime))
                    .foreign_key(
                        ForeignKey::new()
                            .from(ChatSession::Table, ChatSession::OwnerUid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::new()
                            .from(ChatSession::Table, ChatSession::LastMessageId)
                            .to(ChatMessage::Table, ChatMessage::Mid)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_chat_message_sender")
                    .table(ChatMessage::Table)
                    .col(ChatMessage::SenderUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chat_session_owner_target")
                    .table(ChatSession::Table)
                    .col(ChatSession::OwnerUid)
                    .col(ChatSession::TargetId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_chat_session_owner_target").to_owned())
            .await?
            .drop_index(Index::drop().name("idx_chat_message_sender").to_owned())
            .await?
            .drop_table(Table::drop().table(ChatSession::Table).to_owned())
            .await?
            .drop_table(Table::drop().table(ChatMessage::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Uid,
}

#[derive(DeriveIden)]
enum ChatMessage {
    Table,
    Mid,
    SenderUid,
    ReceiverUid,
    Content,
    MessageType,
    Status,
    SendTime,
    CreateTime,
}

#[derive(DeriveIden)]
enum ChatSession {
    Table,
    SessionId,
    OwnerUid,
    SessionType,
    TargetId,
    LastMessageId,
    UnreadCount,
    UpdateTime,
}
