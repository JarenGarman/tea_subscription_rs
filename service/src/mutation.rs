use ::entity::{customer_subscription, customer_subscription::Entity as CustomerSubscription};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, DbErr, EntityTrait};

pub struct Mutation;

impl Mutation {
    #[expect(clippy::missing_errors_doc)]
    pub async fn update_customer_subscription_by_id(
        db: &DbConn,
        id: i32,
        active: bool,
    ) -> Result<customer_subscription::Model, DbErr> {
        let mut customer_subscription: customer_subscription::ActiveModel =
            CustomerSubscription::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom(
                    "Cannot find customer_subscription.".to_owned(),
                ))
                .map(Into::into)?;

        customer_subscription.active = Set(active);

        customer_subscription::ActiveModel::update(customer_subscription, db).await
    }
}
