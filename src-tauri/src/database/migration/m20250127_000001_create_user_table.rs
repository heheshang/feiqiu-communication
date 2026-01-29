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
                    .if_not_exists()
                    .col(ColumnDef::new(User::Uid).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::FeiqIp).string().not_null())
                    .col(ColumnDef::new(User::FeiqPort).integer().not_null())
                    .col(ColumnDef::new(User::FeiqMachineId).string().not_null())
                    .col(ColumnDef::new(User::Nickname).string().not_null())
                    .col(ColumnDef::new(User::Avatar).string())
                    .col(ColumnDef::new(User::Status).tiny_integer())
                    .col(ColumnDef::new(User::CreateTime).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdateTime).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
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
