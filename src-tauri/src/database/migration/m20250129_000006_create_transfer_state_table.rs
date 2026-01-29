use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TransferState::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TransferState::Tid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TransferState::FileId).integer().not_null())
                    .col(ColumnDef::new(TransferState::SessionType).tiny_integer().not_null())
                    .col(ColumnDef::new(TransferState::TargetId).integer().not_null())
                    .col(ColumnDef::new(TransferState::Direction).tiny_integer().not_null())
                    .col(ColumnDef::new(TransferState::Transferred).big_integer().not_null())
                    .col(ColumnDef::new(TransferState::FileSize).big_integer().not_null())
                    .col(ColumnDef::new(TransferState::Status).tiny_integer().not_null())
                    .col(ColumnDef::new(TransferState::PacketNo).string().not_null())
                    .col(ColumnDef::new(TransferState::TargetIp).string().not_null())
                    .col(ColumnDef::new(TransferState::TargetPort).integer().not_null())
                    .col(ColumnDef::new(TransferState::Checksum).string().not_null())
                    .col(ColumnDef::new(TransferState::ErrorMessage).string())
                    .col(ColumnDef::new(TransferState::UpdateTime).string().not_null())
                    .col(ColumnDef::new(TransferState::CreateTime).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TransferState::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum TransferState {
    Table,
    Tid,
    FileId,
    SessionType,
    TargetId,
    Direction,
    Transferred,
    FileSize,
    Status,
    PacketNo,
    TargetIp,
    TargetPort,
    Checksum,
    ErrorMessage,
    UpdateTime,
    CreateTime,
}
