use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection};
use std::error::Error;

use crate::Db;

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl Tag {
    pub async fn get_all(db: &mut Connection<Db>) -> Result<Vec<Tag>, Box<dyn Error>> {
        let tags: Vec<Tag> = sqlx::query_as("SELECT * FROM tag;")
            .fetch_all(&mut ***db)
            .await?;
        Ok(tags)
    }
}
