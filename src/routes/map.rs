use std::ops::Deref;

use crate::auth::AuthUser;
use crate::models::tag::Tag;
use crate::{models::map::Map, Db};
use askama_rocket::Template;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use serde_json::Value;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    user: AuthUser,
}

#[rocket::get("/")]
pub fn index(user_data: AuthUser) -> IndexTemplate {
    IndexTemplate { user: user_data }
}

#[derive(Template)]
#[template(path = "explore.html")]
pub struct ExploreTemplate {
    roadmaps: Vec<Map>,
    user: AuthUser,
    tags: Vec<Tag>,
}

#[rocket::get("/roadmaps?<title>&<query_tags>")]
pub async fn explore(
    user_data: AuthUser,
    mut db: Connection<Db>,
    title: Option<&str>,
    query_tags: Vec<&str>,
) -> ExploreTemplate {
    let roadmaps = Map::get_all(&mut db, title, &query_tags)
        .await
        .expect("Cannot get roadmaps at the moment.");

    let tags = Tag::get_all(&mut db)
        .await
        .expect("Cannot get tags at the moment.");
    println!("{:?}, {:?}", title, query_tags);
    ExploreTemplate {
        roadmaps,
        tags,
        user: user_data,
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct CreateMapData {
    title: String,
    description: String,
    keywords: Vec<String>,
    content: Value,
    sources: Vec<String>,
    tags: Vec<String>,
}

#[rocket::post("/roadmaps", format = "json", data = "<map_data>")]
pub async fn create_roadmap(db: Connection<Db>, map_data: Json<CreateMapData>) -> Json<Map> {
    let map = Map::new(
        db,
        map_data.title.clone(),
        map_data.description.clone(),
        map_data.keywords.clone(),
        map_data.content.clone(),
        map_data.sources.clone(),
        map_data.tags.clone(),
    )
    .await
    .expect("Can not create roadmap at the moment.");
    Json(map)
}

#[rocket::put("/roadmaps/<map_id>", format = "json", data = "<new_map>")]
pub async fn edit_roadmap(
    db: Connection<Db>,
    map_id: i32,
    new_map: Json<CreateMapData>,
) -> Json<Map> {
    let map = Map::edit(
        db,
        map_id,
        new_map.title.clone(),
        new_map.description.clone(),
        new_map.keywords.clone(),
        new_map.content.clone(),
        new_map.sources.clone(),
        new_map.tags.clone(),
    )
    .await
    .expect("Can not edit roadmap at the moment.");
    Json(map)
}

#[derive(Template)]
#[template(path = "create.html")]
pub struct CreateTemplate {
    user: AuthUser,
}

#[rocket::get("/create")]
pub async fn create_roadmap_page(user_data: AuthUser) -> CreateTemplate {
    CreateTemplate {
        user: user_data
    }
}
