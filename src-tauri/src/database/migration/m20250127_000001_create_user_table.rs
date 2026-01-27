use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(pk_auto(User::Uid))
                    .col(String(User::FeiqIp))
                    .col(Integer(User::FeiqPort))
                    .col(String(User::FeiqMachineId))
                    .col(String(User::Nickname))
                    .col(String(User::Avatar))
                    .col(TinyInt(User::Status))
                    .col(TimeTimestamp(User::CreateTime))
                    .col(TimeTimestamp(User::UpdateTime))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Uid,
    FeiqIp,
    FeiqPort,
    FeiqMachineId,
    Nickname,
    Avatar,
    Status,
    CreateTime,
    UpdateTime,
}
