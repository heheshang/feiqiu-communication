use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Contact::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Contact::OwnerUid).integer().not_null())
                    .col(ColumnDef::new(Contact::ContactUid).integer().not_null())
                    .col(ColumnDef::new(Contact::Remark).string())
                    .col(ColumnDef::new(Contact::Tag).string())
                    .col(ColumnDef::new(Contact::CreateTime).timestamp().not_null())
                    .col(ColumnDef::new(Contact::UpdateTime).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Contact::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Contact {
    Table,
    Id,
    OwnerUid,
    ContactUid,
    Remark,
    Tag,
    CreateTime,
    UpdateTime,
}
