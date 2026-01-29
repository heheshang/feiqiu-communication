use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create group table
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Group::Gid).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Group::GroupName).string().not_null())
                    .col(ColumnDef::new(Group::Avatar).string())
                    .col(ColumnDef::new(Group::CreatorUid).integer().not_null())
                    .col(ColumnDef::new(Group::Description).string())
                    .col(ColumnDef::new(Group::CreateTime).timestamp().not_null())
                    .col(ColumnDef::new(Group::UpdateTime).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create group_member table
        manager
            .create_table(
                Table::create()
                    .table(GroupMember::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GroupMember::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GroupMember::Gid).integer().not_null())
                    .col(ColumnDef::new(GroupMember::MemberUid).integer().not_null())
                    .col(ColumnDef::new(GroupMember::Role).tiny_integer().not_null())
                    .col(ColumnDef::new(GroupMember::JoinTime).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create unique index on (gid, member_uid)
        manager
            .create_index(
                Index::create()
                    .name("idx_group_member_gid_uid")
                    .table(GroupMember::Table)
                    .col(GroupMember::Gid)
                    .col(GroupMember::MemberUid)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_group_member_gid_uid").to_owned())
            .await?;
        manager.drop_table(Table::drop().table(GroupMember::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Group::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Gid,
    GroupName,
    Avatar,
    CreatorUid,
    Description,
    CreateTime,
    UpdateTime,
}

#[derive(DeriveIden)]
enum GroupMember {
    Table,
    Id,
    Gid,
    MemberUid,
    Role,
    JoinTime,
}
