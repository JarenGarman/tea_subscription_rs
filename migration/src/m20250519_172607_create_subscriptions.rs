use sea_orm_migration::{
    prelude::*,
    schema::{money, pk_auto, string},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(pk_auto(Subscription::Id))
                    .col(string(Subscription::Title))
                    .col(money(Subscription::Price))
                    .col(string(Subscription::ImageURL))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subscription {
    Table,
    Id,
    Title,
    Price,
    ImageURL,
}
