use sea_orm_migration::{
    prelude::*,
    schema::{boolean, integer, pk_auto, time},
};

use crate::{
    m20250519_171144_create_customers::Customer,
    m20250519_172607_create_subscriptions::Subscription,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CustomerSubscription::Table)
                    .if_not_exists()
                    .col(pk_auto(CustomerSubscription::Id))
                    .col(boolean(CustomerSubscription::Active))
                    .col(time(CustomerSubscription::Frequency))
                    .col(integer(CustomerSubscription::CustomerId))
                    .col(integer(CustomerSubscription::SubscriptionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-customer_subscription-customer_id")
                            .from(
                                CustomerSubscription::Table,
                                CustomerSubscription::CustomerId,
                            )
                            .to(Customer::Table, Customer::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-customer_subscription-subscription_id")
                            .from(
                                CustomerSubscription::Table,
                                CustomerSubscription::SubscriptionId,
                            )
                            .to(Subscription::Table, Subscription::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CustomerSubscription::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CustomerSubscription {
    Table,
    Id,
    Active,
    Frequency,
    CustomerId,
    SubscriptionId,
}
