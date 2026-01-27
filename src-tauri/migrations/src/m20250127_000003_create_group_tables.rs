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
                    .col(pk_auto(Group::Gid))
                    .col(String(Group::GroupName))
                    .col(String(Group::Avatar))
                    .col(Integer(Group::CreatorUid))
                    .col(String(Group::Description))
                    .col(TimeTimestamp(Group::CreateTime))
                    .col(TimeTimestamp(Group::UpdateTime))
                    .foreign_key(
                        ForeignKey::new()
                            .from(Group::Table, Group::CreatorUid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create group_member table
        manager
            .create_table(
                Table::create()
                    .table(GroupMember::Table)
                    .col(pk_auto(GroupMember::Id))
                    .col(Integer(GroupMember::Gid))
                    .col(Integer(GroupMember::MemberUid))
                    .col(TinyInt(GroupMember::Role))
                    .col(TimeTimestamp(GroupMember::JoinTime))
                    .foreign_key(
                        ForeignKey::new()
                            .from(GroupMember::Table, GroupMember::Gid)
                            .to(Group::Table, Group::Gid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::new()
                            .from(GroupMember::Table, GroupMember::MemberUid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
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
            .await?
            .drop_table(Table::drop().table(GroupMember::Table).to_owned())
            .await?
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
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
