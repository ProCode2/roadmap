use std::error::Error;

use crate::Db;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection};

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password_digest: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn get(
        mut con: Connection<Db>,
        email: String,
        password: String,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let user: User = sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&mut **con)
            .await?;
        if !verify(password, &user.password_digest).unwrap() {
            return Ok(None);
        }

        Ok(Some(user))
    }

    pub async fn new(
        mut con: Connection<Db>,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, Box<dyn Error>> {
        let password_hash = hash(password, DEFAULT_COST).unwrap();
        let user: User = sqlx::query_as(
            "INSERT INTO users (name, email, password_digest) VALUES($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&mut **con)
        .await?;
        Ok(user)
    }
}
