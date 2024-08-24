use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use serde_json::Value;
use sqlx::Acquire;
use std::error::Error;

use crate::Db;

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Map {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub content: Value,
    pub sources: Vec<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Map {
    pub async fn get_all(mut con: Connection<Db>) -> Result<Vec<Map>, Box<dyn Error>> {
        let maps: Vec<Map> = sqlx::query_as("SELECT * from Map")
            .fetch_all(&mut **con)
            .await?;
        println!("{:?}", maps);
        Ok(maps)
    }

    pub async fn new(
        mut con: Connection<Db>,
        title: String,
        desc: String,
        keywords: Vec<String>,
        content: Value,
        sources: Vec<String>,
        tags: Vec<String>,
    ) -> Result<Map, Box<dyn Error>> {
        // create a new transaction
        let mut tx = con.begin().await?;
        // perform all related queries inside tx

        // create map
        let map: Map = sqlx::query_as("INSERT INTO map (title, slug, description, keywords, content, sources, user_id) VALUES($1, $2, $3, $4, $5, $6, 1) RETURNING *")
            .bind(&title)
            .bind(&title.split(" ").collect::<Vec<_>>().join("-")) .bind(&desc)
            .bind(&keywords)
            .bind(&content)
            .bind(&sources).fetch_one(&mut *tx).await?;

        // create tags of the map
        for tag in tags {
            // the update on conflict is intentional cause otherwise it won't return
            // the id
            let tag_id: (i32,) = sqlx::query_as(
                "INSERT INTO tag (name) VALUES($1) ON CONFLICT (name) DO UPDATE SET id = tag.id RETURNING id",
            )
            .bind(tag)
            .fetch_one(&mut *tx)
            .await?;

            // fill map to tag mapper table
            let _ = sqlx::query("INSERT INTO map_tag (map_id, tag_id) VALUES($1, $2)")
                .bind(map.id)
                .bind(tag_id.0)
                .execute(&mut *tx)
                .await?;
        }

        // commit the transaction
        tx.commit().await?;
        Ok(map)
    }
}
