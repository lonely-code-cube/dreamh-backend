use rocket::serde::json::{Json, Value};
use rocket::State;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::db::DB;
use crate::models::models::Entry;

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

#[derive(Deserialize)]
pub struct Search {
    limit: Option<u32>,
    page: Option<u32>,
    include_tags: Option<Vec<String>>,
    exclude_tags: Option<Vec<String>>,
    include_authors: Option<Vec<String>>,
    exclude_authors: Option<Vec<String>>,
    include_tiers: Option<Vec<String>>,
    exclude_tiers: Option<Vec<String>>,
    rating_above: Option<u32>,
    rating_below: Option<u32>,
    include_parodies: Option<Vec<String>>,
    exclude_parodies: Option<Vec<String>>,
    include_pair: Option<Vec<String>>,
    exclude_pair: Option<Vec<String>>,
    benzene: Option<bool>,
    max_page: Option<u32>,
    min_page: Option<u32>,
    query: Option<String>,
}

impl ApiEntry {
    pub fn from(entry: &Entry) -> Self {
        ApiEntry {
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
        }
    }
}

#[get("/all")]
pub async fn get_all(db: &State<DB>) -> Value {
    let recent = db.recent(Some(10), Some(0)).await.unwrap();
    let top_tier = db.top_tier(Some(10), Some(0)).await.unwrap();
    let popular = db.popular(Some(10), Some(0)).await.unwrap();
    let top_rated = db.top_rated(Some(10), Some(0)).await.unwrap();
    json!({ "recent": recent.iter().map(|entry| ApiEntry::from(entry)).collect::<Vec<_>>(),
        "topTier":  top_tier.iter().map(|entry| ApiEntry::from(entry)).collect::<Vec<_>>(),
        "popular": popular.iter().map(|entry| ApiEntry::from(entry)).collect::<Vec<_>>(),
        "topRated": top_rated.iter().map(|entry| ApiEntry::from(entry)).collect::<Vec<_>>(),
    })
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

#[get("/entries?<page>&<limit>")]
pub async fn get_entries(
    db: &State<DB>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Json<Vec<ApiEntry>> {
    let limit = match limit {
        Some(x) => {
            if x <= 50 {
                x
            } else {
                50
            }
        }
        None => 50,
    };
    let entries = db
        .entries(Some(limit), page)
        .await
        .unwrap()
        .iter()
        .map(|entry| ApiEntry::from(entry))
        .collect::<Vec<_>>();
    Json(entries)
}

#[get("/recent?<page>&<limit>")]
pub async fn get_recent(
    db: &State<DB>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Json<Vec<ApiEntry>> {
    let limit = match limit {
        Some(x) => {
            if x <= 50 {
                x
            } else {
                50
            }
        }
        None => 50,
    };
    let entries = db
        .recent(Some(limit), page)
        .await
        .unwrap()
        .iter()
        .map(|entry| ApiEntry::from(entry))
        .collect::<Vec<_>>();
    Json(entries)
}

#[get("/top-tiers?<page>&<limit>")]
pub async fn get_top_tiers(
    db: &State<DB>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Json<Vec<ApiEntry>> {
    let limit = match limit {
        Some(x) => {
            if x <= 50 {
                x
            } else {
                50
            }
        }
        None => 50,
    };
    let entries = db
        .top_tier(Some(limit), page)
        .await
        .unwrap()
        .iter()
        .map(|entry| ApiEntry::from(entry))
        .collect::<Vec<_>>();
    Json(entries)
}

#[get("/popular?<page>&<limit>")]
pub async fn get_popular(
    db: &State<DB>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Json<Vec<ApiEntry>> {
    let limit = match limit {
        Some(x) => {
            if x <= 50 {
                x
            } else {
                50
            }
        }
        None => 50,
    };
    let entries = db
        .popular(Some(limit), page)
        .await
        .unwrap()
        .iter()
        .map(|entry| ApiEntry::from(entry))
        .collect::<Vec<_>>();
    Json(entries)
}

#[get("/top-rated?<page>&<limit>")]
pub async fn get_top_rated(
    db: &State<DB>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Json<Vec<ApiEntry>> {
    let limit = match limit {
        Some(x) => {
            if x <= 50 {
                x
            } else {
                50
            }
        }
        None => 50,
    };
    let entries = db
        .top_rated(Some(limit), page)
        .await
        .unwrap()
        .iter()
        .map(|entry| ApiEntry::from(entry))
        .collect::<Vec<_>>();
    Json(entries)
}

#[options("/search")]
pub async fn search_options() {}

#[post("/search", format = "application/json", data = "<search_params>")]
pub async fn search(search_params: Json<Search>, db: &State<DB>) -> Json<Vec<ApiEntry>> {
    let limit = match search_params.limit {
        Some(x) => {
            if x <= 50 {
                x
            } else {
                50
            }
        }
        None => 50,
    };
    let entries = db
        .search(
            search_params.limit,
            search_params.page,
            search_params.include_tags.to_owned(),
            search_params.exclude_tags.to_owned(),
            search_params.include_authors.to_owned(),
            search_params.exclude_authors.to_owned(),
            search_params.include_tiers.to_owned(),
            search_params.exclude_tiers.to_owned(),
            search_params.rating_above,
            search_params.rating_below,
            search_params.include_parodies.to_owned(),
            search_params.exclude_parodies.to_owned(),
            search_params.include_pair.to_owned(),
            search_params.exclude_pair.to_owned(),
            search_params.benzene,
            search_params.max_page,
            search_params.min_page,
            search_params.query.to_owned(),
        )
        .await
        .unwrap()
        .iter()
        .map(|entry| ApiEntry::from(entry))
        .collect::<Vec<_>>();
    Json(entries)
}

#[get("/entry/<id>")]
pub async fn entry(id: u32, db: &State<DB>) -> (Status, Option<Json<ApiEntry>>) {
    let e = db.entry(id).await.unwrap();
    match e {
        Some(x) => (Status::Ok, Some(Json(ApiEntry::from(&x)))),
        None => (Status::NotFound, None)
    }
}