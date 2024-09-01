use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rocket::form::validate::Len;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{self, Acquire, PgConnection};
use rocket_db_pools::Connection;
use serde_json::Value;
use sqlx::Postgres;
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
    pub async fn get_by_id(
        mut con: Connection<Db>,
        map_id: i32,
        user_id: i32,
    ) -> Result<Map, Box<dyn Error>> {
        let map: Map = sqlx::query_as("SELECT * FROM Map WHERE id = $1 AND user_id = $2")
            .bind(map_id)
            .bind(user_id)
            .fetch_one(&mut **con)
            .await?;
        Ok(map)
    }
    pub async fn get_all(
        con: &mut Connection<Db>,
        title: Option<&str>,
        tags: &Vec<&str>,
    ) -> Result<Vec<Map>, Box<dyn Error>> {
        if title.is_some() && title.map(|v| v != "").unwrap() && tags.len() > 0 {
            println!("Here both");
            let title = format!("%{}%", title.unwrap());
            let maps: Vec<Map> = sqlx::query_as("SELECT m.* FROM Map m JOIN map_tag mt ON m.id = mt.map_id JOIN tag t ON t.id = mt.tag_id WHERE t.name = ANY($1) AND m.title ILIKE $2")
                .bind(&tags)
                .bind(title)
                .fetch_all(&mut ***con)
            .await?;

            println!("{:?}", maps);
            Ok(maps)
        } else if title.is_some() && title.map(|v| v != "").unwrap() {
            println!("Here title");
            let title = format!("%{}%", title.unwrap());
            let maps: Vec<Map> = sqlx::query_as("SELECT * FROM Map WHERE title ILIKE $1")
                .bind(title)
                .fetch_all(&mut ***con)
                .await?;
            println!("{:?}", maps);
            Ok(maps)
        } else if tags.len() > 0 {
            println!("Here tags");
            let maps: Vec<Map> = sqlx::query_as("SELECT m.* FROM Map m JOIN map_tag mt ON m.id = mt.map_id JOIN tag t ON t.id = mt.tag_id WHERE t.name = ANY($1)").bind(&tags).fetch_all(&mut ***con).await?;

            println!("{:?}", maps);
            Ok(maps)
        } else {
            let maps: Vec<Map> = sqlx::query_as("SELECT * from Map")
                .fetch_all(&mut ***con)
                .await?;
            println!("{:?}", maps);
            Ok(maps)
        }
    }

    pub async fn edit(
        mut con: Connection<Db>,
        map_id: i32,
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

        // update map
        let map: Map = sqlx::query_as("UPDATE map SET title = $1, slug = $2, description = $3, keywords = $4, content = $5, sources = $6 WHERE id = $7 AND user_id = 1 RETURNING *")
            .bind(&title)
            .bind(&title.split(" ").collect::<Vec<_>>().join("-")) 
            .bind(&desc)
            .bind(&keywords)
            .bind(&content)
            .bind(&sources)
            .bind(&map_id)
            .fetch_one(&mut *tx).await?;

        // update tags of the map
        for tag in tags.clone() {
            // the update on conflict is intentional cause otherwise it won't return
            // the id
            let tag_id: (i32,) = sqlx::query_as(
                "INSERT INTO tag (name) VALUES($1) ON CONFLICT (name) DO UPDATE SET id = tag.id RETURNING id",
            )
            .bind(tag)
            .fetch_one(&mut *tx)
            .await?;

            // fill map to tag mapper table
            let _ = sqlx::query(
                "INSERT INTO map_tag (map_id, tag_id) VALUES($1, $2) ON CONFLICT (map_id, tag_id) DO NOTHING",
            )
            .bind(map.id)
            .bind(tag_id.0)
            .execute(&mut *tx)
            .await?;

            // delete tags that don't exist for this map anymore
            let _ = sqlx::query(
                "DELETE FROM map_tag WHERE map_id = $1 AND tag_id NOT IN (SELECT id FROM tag WHERE name = ANY($2))",
            )
            .bind(map.id)
            .bind(&tags)
            .execute(&mut *tx)
            .await?;
        }

        // commit the transaction
        tx.commit().await?;
        Ok(map)
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
