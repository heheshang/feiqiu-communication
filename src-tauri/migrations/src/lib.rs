pub use sea_orm_migration::prelude::*;

mod m20250127_000001_create_user_table;
mod m20250127_000002_create_contact_table;
mod m20250127_000003_create_group_tables;
mod m20250127_000004_create_chat_tables;
mod m20250127_000005_create_file_storage_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250127_000001_create_user_table::Migration),
            Box::new(m20250127_000002_create_contact_table::Migration),
            Box::new(m20250127_000003_create_group_tables::Migration),
            Box::new(m20250127_000004_create_chat_tables::Migration),
            Box::new(m20250127_000005_create_file_storage_table::Migration),
        ]
    }
}
