use sea_orm_migration::{
    prelude::*,
    schema::{pk_auto, string},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Customer::Table)
                    .if_not_exists()
                    .col(pk_auto(Customer::Id))
                    .col(string(Customer::FirstName))
                    .col(string(Customer::LastName))
                    .col(string(Customer::Email))
                    .col(string(Customer::Address))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Customer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Customer {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Address,
}
