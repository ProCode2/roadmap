mod db;
mod models;
mod routes;

use dotenvy::dotenv;
use rocket::fs;
use std::{env, error::Error};
#[macro_use]
extern crate rocket;

use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlx_pg")]
struct Db(sqlx::PgPool);

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect("Failed to load environment variable");
    let db_url = env::var("DATABASE_URL").expect("Database url not found");
    println!("{}", db_url);
    let figment = rocket::Config::figment().merge((
        "databases.sqlx_pg",
        rocket_db_pools::Config {
            url: db_url,
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
            extensions: None,
        },
    ));

    let _rocket = rocket::custom(figment)
        .attach(Db::init())
        .mount("/assets", fs::FileServer::from("./assets"))
        .mount(
            "/",
            routes![
                routes::map::index,
                routes::map::explore,
                routes::map::create_roadmap
            ],
        )
        .launch()
        .await?;
    Ok(())
}
