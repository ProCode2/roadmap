use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

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

#[rocket::post("/login", data = "<login_data>")]
pub async fn login(
    db: Connection<Db>,
    login_data: Json<LoginData>,
) -> Result<Json<String>, Status> {
    let user_data = User::get(db, login_data.email.clone(), login_data.password.clone()).await;
    match user_data {
        Ok(got_user) => match got_user {
            Some(user) => {
                let claim_token = create_jwt(user.id).unwrap();
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
