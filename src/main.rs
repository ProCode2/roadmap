mod auth;
mod db;
mod jwt;
mod models;
mod routes;

use dotenvy::dotenv;
use rocket::fs;
use std::{env, error::Error};
#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use rocket_db_pools::{sqlx, Database};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "content-type, authorization",
            ));
        }
        // TODO: THIS SHOULD NOT BE *
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Vary", "Origin"));
    }
}

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
            connect_timeout: 10,
            idle_timeout: None,
            extensions: None,
        },
    ));

    let _rocket = rocket::custom(figment)
        .attach(CORS)
        .attach(Db::init())
        .mount("/assets", fs::FileServer::from("./assets"))
        .mount(
            "/auth",
            routes![
                routes::user::login_page,
                routes::user::login,
                routes::user::register,
                routes::user::register_page,
                routes::user::logout,
            ],
        )
        .mount(
            "/",
            routes![
                routes::map::index,
                routes::map::explore,
                routes::map::create_roadmap,
                routes::map::create_roadmap_page,
                routes::map::edit_roadmap
            ],
        )
        .launch()
        .await?;
    Ok(())
}
