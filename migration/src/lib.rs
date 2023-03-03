pub use sea_orm_migration::prelude::*;

mod m20230303_111206_create_table_orders;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230303_111206_create_table_orders::Migration),
        ]
    }
}
