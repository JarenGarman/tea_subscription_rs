use ::entity::{subscription, subscription::Entity as Subscription};
use sea_orm::{DbConn, DbErr, EntityTrait};

pub struct Query;

impl Query {
    #[expect(clippy::missing_errors_doc)]
    pub async fn find_subscription_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<subscription::Model>, DbErr> {
        Subscription::find_by_id(id).one(db).await
    }

    #[expect(clippy::missing_errors_doc)]
    pub async fn find_subscriptions(db: &DbConn) -> Result<Vec<subscription::Model>, DbErr> {
        Subscription::find().all(db).await
    }
}
