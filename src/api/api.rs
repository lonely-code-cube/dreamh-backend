use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

use crate::db::DB;

#[derive(Serialize)]
pub struct ApiTag {
    id: String,
    name: String,
}
#[derive(Serialize)]
pub struct ApiAuthor {
    id: String,
    name: String,
    source: Option<String>,
}

#[derive(Serialize)]
pub struct ApiEntry {
    pub oid: String,
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub parody: Option<String>,
    pub author_name: String,
    pub author_id: String,
    pub thumbnail: String,
    pub pair: String,
    pub tier: String,
    pub rating: f32,
    pub rated_by: u32,
    pub favorites: u32,
    pub tags: Vec<ApiTag>,
    pub pages: Vec<String>,
    pub ehentai: Option<String>,
    pub nhentai: Option<String>,
    pub imgur: Option<String>,
    pub source: Option<String>,
    pub benzene: bool,
}

#[get("/tags")]
pub async fn get_tags(db: &State<DB>) -> Json<Vec<ApiTag>> {
    let tags = db
        .all_tags()
        .await
        .unwrap()
        .iter()
        .map(|tag| ApiTag {
            id: tag.id.unwrap().to_string(),
            name: tag.name.to_owned(),
        })
        .collect::<Vec<_>>();
    Json(tags)
}

#[get("/authors")]
pub async fn get_authors(db: &State<DB>) -> Json<Vec<ApiAuthor>> {
    let authors = db
        .all_authors()
        .await
        .unwrap()
        .iter()
        .map(|author| ApiAuthor {
            id: author.id.unwrap().to_string(),
            name: author.name.to_owned(),
            source: author.source.to_owned(),
        })
        .collect::<Vec<_>>();
    Json(authors)
}

#[get("/entries")]
pub async fn get_entries(db: &State<DB>) -> Json<Vec<ApiEntry>> {
    let entries = db
        .entries()
        .await
        .unwrap()
        .iter()
        .map(|entry| ApiEntry {
            oid: entry.oid.unwrap().to_string(),
            id: entry.id.unwrap(),
            title: entry.title.to_owned(),
            description: entry.description.to_owned(),
            parody: entry.parody.to_owned(),
            author_name: entry.author_name.to_owned(),
            author_id: entry.author_id.to_string(),
            thumbnail: entry.thumbnail.to_owned(),
            pair: entry.pair.to_owned(),
            tier: entry.tier.to_owned(),
            rating: entry.rating,
            rated_by: entry.rated_by,
            favorites: entry.favorites,
            tags: entry
                .tags
                .to_owned()
                .iter()
                .map(|tag| ApiTag {
                    name: tag.name.to_owned(),
                    id: tag.id.unwrap().to_string(),
                })
                .collect(),
            pages: entry.pages.to_owned(),
            ehentai: entry.ehentai.to_owned(),
            nhentai: entry.nhentai.to_owned(),
            imgur: entry.imgur.to_owned(),
            source: entry.source.to_owned(),
            benzene: entry.benzene,
        })
        .collect::<Vec<_>>();
    Json(entries)
}
