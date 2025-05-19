use sea_orm_migration::{
    prelude::*,
    schema::{pk_auto, string, time},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tea::Table)
                    .if_not_exists()
                    .col(pk_auto(Tea::Id))
                    .col(string(Tea::Title))
                    .col(string(Tea::Description))
                    .col(time(Tea::BrewTime))
                    .col(string(Tea::ImageURL))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tea::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tea {
    Table,
    Id,
    Title,
    Description,
    BrewTime,
    ImageURL,
}
