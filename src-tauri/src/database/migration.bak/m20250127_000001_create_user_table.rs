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
                    .col(string(User::FeiqIp))
                    .col(integer(User::FeiqPort))
                    .col(string(User::FeiqMachineId))
                    .col(string(User::Nickname))
                    .col(string(User::Avatar))
                    .col(tiny_int(User::Status))
                    .col(timestamp(User::CreateTime))
                    .col(timestamp(User::UpdateTime))
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
