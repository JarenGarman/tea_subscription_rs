use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use rocket::routes;

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

    let _rocket = rocket::build().manage(db).mount("/", routes![]);

    Ok(())
}
