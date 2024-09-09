use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{self, Acquire};
use rocket_db_pools::Connection;
use serde_json::Value;
use std::error::Error;

use crate::Db;

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct MapItem {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct MapPageData {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub tags: Vec<String>,
    pub content: Value,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

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
    pub async fn get_by_slug(
        mut con: Connection<Db>,
        slug: &str,
    ) -> Result<MapPageData, Box<dyn Error>> {
        let map: MapPageData = sqlx::query_as("SELECT m.id, m.title, m.slug, m.description, m.content, m.created_at, ARRAY_AGG(t.name) AS tags, u.id AS user_id, u.name AS user_name FROM Map m JOIN map_tag mt ON m.id = mt.map_id JOIN tag t ON t.id = mt.tag_id JOIN users u ON m.user_id = u.id WHERE m.slug = $1 GROUP BY 
        m.id, u.id")
            .bind(slug)
            .fetch_one(&mut **con)
            .await?;
        Ok(map)
    }
    pub async fn get_all(
        con: &mut Connection<Db>,
        title: Option<&str>,
        tags: &Vec<&str>,
    ) -> Result<Vec<MapItem>, Box<dyn Error>> {
        if title.is_some() && title.map(|v| v != "").unwrap() && tags.len() > 0 {
            println!("Here both");
            let title = format!("%{}%", title.unwrap());
            let maps: Vec<MapItem> = sqlx::query_as("SELECT m.id, m.title, m.slug, m.description, m.created_at, ARRAY_AGG(t.name) AS tags, u.id AS user_id, u.name AS user_name FROM Map m JOIN map_tag mt ON m.id = mt.map_id JOIN tag t ON t.id = mt.tag_id JOIN users u ON m.user_id = u.id WHERE t.name = ANY($1) AND m.title ILIKE $2 GROUP BY 
        m.id, u.id")
                .bind(&tags)
                .bind(title)
                .fetch_all(&mut ***con)
            .await?;

            println!("{:?}", maps);
            Ok(maps)
        } else if title.is_some() && title.map(|v| v != "").unwrap() {
            println!("Here title");
            let title = format!("%{}%", title.unwrap());
            let maps: Vec<MapItem> = sqlx::query_as("SELECT m.id, m.title, m.slug, m.description, m.created_at, ARRAY_AGG(t.name) AS tags, u.id AS user_id , u.name AS user_name FROM Map m JOIN map_tag mt ON m.id = mt.map_id JOIN tag t ON t.id = mt.tag_id JOIN users u ON m.user_id = u.id WHERE m.title ILIKE $1 GROUP BY 
        m.id, u.id")
                .bind(title)
                .fetch_all(&mut ***con)
                .await?;
            println!("{:?}", maps);
            Ok(maps)
        } else if tags.len() > 0 {
            println!("Here tags");
            let maps: Vec<MapItem> = sqlx::query_as("SELECT m.id, m.title, m.slug, m.description, m.created_at, ARRAY_AGG(t.name) AS tags, u.id AS user_id, u.name AS user_name FROM Map m JOIN map_tag mt ON m.id = mt.map_id JOIN tag t ON t.id = mt.tag_id JOIN users u ON m.user_id = u.id WHERE t.name = ANY($1) GROUP BY 
        m.id, u.id").bind(&tags).fetch_all(&mut ***con).await?;

            println!("{:?}", maps);
            Ok(maps)
        } else {
            let maps: Vec<MapItem> = sqlx::query_as("SELECT m.id, m.title, m.slug, m.description, m.created_at, ARRAY_AGG(t.name) AS tags, u.id AS user_id, u.name AS user_name FROM Map m JOIN map_tag mt ON m.id = mt.map_id JOIN tag t ON t.id = mt.tag_id JOIN users u ON m.user_id = u.id GROUP BY 
        m.id, u.id")
                .fetch_all(&mut ***con)
                .await?;
            println!("{:?}", maps);
            Ok(maps)
        }
    }

    pub async fn edit(
        mut con: Connection<Db>,
        map_id: i32,
        user_id: i32,
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
        let map: Map = sqlx::query_as("UPDATE map SET title = $1, slug = $2, description = $3, keywords = $4, content = $5, sources = $6 WHERE id = $7 AND user_id = $8 RETURNING *")
            .bind(&title)
            .bind(&title.split(" ").collect::<Vec<_>>().join("-")) 
            .bind(&desc)
            .bind(&keywords)
            .bind(&content)
            .bind(&sources)
            .bind(&map_id)
            .bind(&user_id)
            .fetch_one(&mut *tx).await?;

        println!("Here {:?}", map);
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
