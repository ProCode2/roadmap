use crate::{models::map::Map, Db};
use askama_rocket::Template;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use serde_json::{json, Value};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[rocket::get("/")]
pub fn index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "explore.html")]
pub struct ExploreTemplate {
    roadmaps: Vec<Map>,
}

#[rocket::get("/roadmaps")]
pub async fn explore(db: Connection<Db>) -> ExploreTemplate {
    let roadmaps = Map::get_all(db)
        .await
        .expect("Can not get roadmaps at the moment.");

    println!("{:?}", roadmaps);
    ExploreTemplate { roadmaps }
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
