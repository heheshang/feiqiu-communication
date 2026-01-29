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
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FileStorage::Fid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FileStorage::FileName).string().not_null())
                    .col(ColumnDef::new(FileStorage::FilePath).string().not_null())
                    .col(ColumnDef::new(FileStorage::FileSize).big_integer().not_null())
                    .col(ColumnDef::new(FileStorage::FileType).string().not_null())
                    .col(ColumnDef::new(FileStorage::UploaderUid).integer().not_null())
                    .col(ColumnDef::new(FileStorage::UploadTime).string().not_null())
                    .col(ColumnDef::new(FileStorage::CreateTime).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(FileStorage::Table).to_owned()).await
    }
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
    CreateTime,
}
