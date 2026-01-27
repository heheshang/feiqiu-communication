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
                    .col(pk_auto(Contact::Id))
                    .col(Integer(Contact::OwnerUid))
                    .col(Integer(Contact::ContactUid))
                    .col(String(Contact::Remark))
                    .col(String(Contact::Tag))
                    .col(TimeTimestamp(Contact::CreateTime))
                    .col(TimeTimestamp(Contact::UpdateTime))
                    .foreign_key(
                        ForeignKey::new()
                            .from(Contact::Table, Contact::OwnerUid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::new()
                            .from(Contact::Table, Contact::ContactUid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await
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
