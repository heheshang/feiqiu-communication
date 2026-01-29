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
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatMessage::Mid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ChatMessage::SessionType).integer().not_null())
                    .col(ColumnDef::new(ChatMessage::TargetId).integer().not_null())
                    .col(ColumnDef::new(ChatMessage::SenderUid).integer().not_null())
                    .col(ColumnDef::new(ChatMessage::MsgType).integer().not_null())
                    .col(ColumnDef::new(ChatMessage::Content).string().not_null())
                    .col(ColumnDef::new(ChatMessage::SendTime).string().not_null())
                    .col(ColumnDef::new(ChatMessage::Status).integer().not_null())
                    .col(ColumnDef::new(ChatMessage::MsgNo).string())
                    .col(ColumnDef::new(ChatMessage::CreateTime).timestamp().not_null())
                    .col(ColumnDef::new(ChatMessage::UpdateTime).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create chat_session table
        manager
            .create_table(
                Table::create()
                    .table(ChatSession::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatSession::Sid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ChatSession::OwnerUid).integer().not_null())
                    .col(ColumnDef::new(ChatSession::SessionType).integer().not_null())
                    .col(ColumnDef::new(ChatSession::TargetId).integer().not_null())
                    .col(ColumnDef::new(ChatSession::LastMsgId).integer())
                    .col(ColumnDef::new(ChatSession::UnreadCount).integer().not_null())
                    .col(ColumnDef::new(ChatSession::UpdateTime).timestamp().not_null())
                    .col(ColumnDef::new(ChatSession::CreateTime).timestamp().not_null())
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
            .await?;
        manager
            .drop_index(Index::drop().name("idx_chat_message_sender").to_owned())
            .await?;
        manager.drop_table(Table::drop().table(ChatSession::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ChatMessage::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum ChatMessage {
    Table,
    Mid,
    SessionType,
    TargetId,
    SenderUid,
    MsgType,
    Content,
    SendTime,
    Status,
    MsgNo,
    CreateTime,
    UpdateTime,
}

#[derive(DeriveIden)]
enum ChatSession {
    Table,
    Sid,
    OwnerUid,
    SessionType,
    TargetId,
    LastMsgId,
    UnreadCount,
    UpdateTime,
    CreateTime,
}
