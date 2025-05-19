use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use rocket::{
    State, patch, routes,
    serde::{Deserialize, json::Json},
};
use sea_orm::DatabaseConnection;
use service::Mutation;

#[derive(Deserialize)]
pub struct SubJson {
    active: bool,
}

#[patch("/<id>", format = "json", data = "<sub_json>")]
async fn update(
    conn: &State<DatabaseConnection>,
    id: i32,
    sub_json: Json<SubJson>,
) -> Json<<entity::prelude::CustomerSubscription as sea_orm::EntityTrait>::Model> {
    let sub = Mutation::update_customer_subscription_by_id(conn, id, sub_json.active)
        .await
        .expect("Failed to update customer susbscription");

    Json(sub)
}

#[rocket::main]
async fn main() -> Result<(), Box<rocket::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sea_orm::Database::connect(database_url)
        .await
        .expect("Failed to connect to the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    let _rocket = rocket::build()
        .manage(db)
        .mount("/api/v1/customer_subscriptions", routes![update])
        .launch()
        .await?;

    Ok(())
}
