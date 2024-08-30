use askama::Template;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::time::{Duration, OffsetDateTime};
use rocket_db_pools::Connection;

use crate::auth::AuthUser;
use crate::jwt::create_jwt;
use crate::models::user::User;
use crate::Db;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct RegisterData {
    name: String,
    email: String,
    password: String,
}

#[rocket::post("/register", data = "<register_data>")]
pub async fn register(
    cookies: &CookieJar<'_>,
    db: Connection<Db>,
    register_data: Json<RegisterData>,
) -> Result<Json<String>, Status> {
    let user_data = User::new(
        db,
        register_data.name.clone(),
        register_data.email.clone(),
        register_data.password.clone(),
    )
    .await;
    match user_data {
        Ok(user) => {
            let claim_token = create_jwt(user.id).unwrap();
            let mut cookie = Cookie::new("auth_cookie", claim_token.to_string());

            let now = OffsetDateTime::now_utc() + Duration::days(1);
            cookie.set_expires(now);

            cookies.add(cookie);
            Ok(Json(claim_token))
        }
        Err(err) => {
            println!("ERROR: {:?}", err);
            return Err(Status::Unauthorized);
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct LoginData {
    email: String,
    password: String,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    user: AuthUser,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    user: AuthUser,
}

#[rocket::get("/login")]
pub fn login_page() -> LoginTemplate {
    LoginTemplate {
        user: AuthUser { id: None },
    }
}

#[rocket::get("/register")]
pub fn register_page() -> RegisterTemplate {
    RegisterTemplate {
        user: AuthUser { id: None },
    }
}

#[rocket::post("/login", data = "<login_data>")]
pub async fn login(
    cookies: &CookieJar<'_>,
    db: Connection<Db>,
    login_data: Json<LoginData>,
) -> Result<Json<String>, Status> {
    let user_data = User::get(db, login_data.email.clone(), login_data.password.clone()).await;
    match user_data {
        Ok(got_user) => match got_user {
            Some(user) => {
                let claim_token = create_jwt(user.id).unwrap();
                let mut cookie = Cookie::new("auth_cookie", claim_token.to_string());

                let now = OffsetDateTime::now_utc() + Duration::days(1);
                cookie.set_expires(now);

                cookies.add(cookie);
                Ok(Json(claim_token))
            }
            None => {
                return Err(Status::Unauthorized);
            }
        },
        Err(err) => {
            println!("ERROR: {:?}", err);
            return Err(Status::Unauthorized);
        }
    }
}

#[rocket::get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove("auth_cookie");
    Redirect::to(uri!("/auth/login"))
}
