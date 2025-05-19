pub use sea_orm_migration::prelude::*;

mod m20250519_170202_create_teas;
mod m20250519_171144_create_customers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250519_170202_create_teas::Migration),
            Box::new(m20250519_171144_create_customers::Migration),
        ]
    }
}
