use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FileStorage::Table)
                    .col(pk_auto(FileStorage::Fid))
                    .col(String(FileStorage::FileName))
                    .col(String(FileStorage::FilePath))
                    .col(BigInteger(FileStorage::FileSize))
                    .col(String(FileStorage::FileType))
                    .col(Integer(FileStorage::UploaderUid))
                    .col(TimeTimestamp(FileStorage::UploadTime))
                    .foreign_key(
                        ForeignKey::new()
                            .from(FileStorage::Table, FileStorage::UploaderUid)
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
            .drop_table(Table::drop().table(FileStorage::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Uid,
}

#[derive(DeriveIden)]
enum FileStorage {
    Table,
    Fid,
    FileName,
    FilePath,
    FileSize,
    FileType,
    UploaderUid,
    UploadTime,
}
