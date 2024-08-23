use askama_rocket::Template;
use dotenvy::dotenv;
use rocket::fs;
use sqlx::Row;
use std::env;
use std::error::Error;

#[macro_use]
extern crate rocket;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[rocket::get("/")]
fn index() -> IndexTemplate<'static> {
    IndexTemplate { name: "world" }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect("Failed to load environment variable");
    let db_url: String = env::var("DB_URL").unwrap();

    let con = sqlx::postgres::PgPool::connect(&db_url).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&con).await?;
    let sum: i32 = res.get("sum");

    println!("1 + 1 = {}", sum);
    let _rocket = rocket::build()
        .mount("/assets", fs::FileServer::from("./assets"))
        .mount("/", routes![index])
        .launch()
        .await?;
    Ok(())
}
