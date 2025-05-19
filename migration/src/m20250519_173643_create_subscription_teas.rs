use sea_orm_migration::{prelude::*, schema::pk_auto};

use crate::{
    m20250519_170202_create_teas::Tea, m20250519_172607_create_subscriptions::Subscription,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SubscriptionTea::Table)
                    .if_not_exists()
                    .col(pk_auto(SubscriptionTea::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Subscription::Table, Subscription::Id)
                            .to(SubscriptionTea::Table, SubscriptionTea::SubscriptionId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tea::Table, Tea::Id)
                            .to(SubscriptionTea::Table, SubscriptionTea::TeaId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubscriptionTea::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SubscriptionTea {
    Table,
    Id,
    SubscriptionId,
    TeaId,
}
