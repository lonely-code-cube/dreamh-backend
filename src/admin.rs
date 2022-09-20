use std::env;
use std::str::FromStr;

use bson::oid::ObjectId;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

use crate::db::DB;
use crate::guards::admin::Admin;
use crate::models::models::{Entry, Tag};

lazy_static! {
    static ref USERNAME: String =
        env::var("USERNAME").expect("USERNAME environment variable not set");
    static ref PASSWORD: String =
        env::var("PASSWORD").expect("PASSWORD environment variable not set");
}

#[derive(Deserialize)]
pub struct LoginCreds {
    name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct TagCreate {
    name: String,
}

#[derive(Deserialize)]
pub struct TagDelete {
    id: String,
}

#[derive(Deserialize)]
pub struct AuthorCreate {
    name: String,
    source: Option<String>,
}

#[derive(Deserialize)]
pub struct EntryTag {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize)]
pub struct EntryCreate {
    pub title: String,
    pub description: Option<String>,
    pub parody: Option<String>,
    pub author_name: String,
    pub author_id: String,
    pub thumbnail: String,
    pub pair: String,
    pub tier: String,
    pub tags: Vec<EntryTag>,
    pub pages: Vec<String>,
    pub ehentai: Option<String>,
    pub nhentai: Option<String>,
    pub imgur: Option<String>,
    pub source: Option<String>,
    pub benzene: bool,
}

#[post("/login", data = "<creds>")]
pub async fn login_post(creds: Json<LoginCreds>, cookies: &CookieJar<'_>) -> Status {
    if constant_time_eq::constant_time_eq(&creds.name.as_bytes(), USERNAME.as_bytes())
        && constant_time_eq::constant_time_eq(&creds.password.as_bytes(), PASSWORD.as_bytes())
    {
        cookies.add_private(Cookie::build("auth", creds.password.to_owned()).finish());
        return Status::Ok;
    }
    Status::Unauthorized
}

#[get("/login")]
pub async fn login() -> Template {
    Template::render("login", context! {})
}

#[get("/")]
pub async fn index(_admin: Admin) -> Template {
    Template::render("admin", context! {})
}

#[get("/add/tag")]
pub async fn add_tag(_admin: Admin, db: &State<DB>) -> Template {
    #[derive(Serialize)]
    struct RenderTag {
        id: String,
        name: String,
    }
    let tags = db
        .all_tags()
        .await
        .unwrap()
        .iter()
        .map(|tag| RenderTag {
            id: tag.id.unwrap().to_string(),
            name: tag.name.to_owned(),
        })
        .collect::<Vec<_>>();
    Template::render(
        "add_tag",
        context! {
            tags: tags,
        },
    )
}

#[post("/add/tag", data = "<tag>")]
pub async fn add_tag_post(_admin: Admin, db: &State<DB>, tag: Json<TagCreate>) -> Status {
    db.add_tag(tag.name.to_owned()).await.unwrap();
    Status::Created
}

#[post("/delete/tag", data = "<tag>")]
pub async fn delete_tag_post(_admin: Admin, db: &State<DB>, tag: Json<TagDelete>) -> Status {
    db.delete_tag(tag.id.to_owned()).await.unwrap();
    Status::Ok
}

#[get("/add/author")]
pub async fn add_author(_admin: Admin, db: &State<DB>) -> Template {
    #[derive(Serialize)]
    struct RenderAuthor {
        id: String,
        name: String,
        source: Option<String>,
    }
    let authors = db
        .all_authors()
        .await
        .unwrap()
        .iter()
        .map(|author| RenderAuthor {
            id: author.id.unwrap().to_string(),
            name: author.name.to_owned(),
            source: author.source.to_owned(),
        })
        .collect::<Vec<_>>();
    Template::render(
        "add_author",
        context! {
            authors: authors,
        },
    )
}

#[post("/add/author", data = "<author>")]
pub async fn add_author_post(_admin: Admin, db: &State<DB>, author: Json<AuthorCreate>) -> Status {
    db.add_author(author.name.to_owned(), author.source.to_owned())
        .await
        .unwrap();
    Status::Created
}

#[get("/add/entry")]
pub async fn add_entry(_admin: Admin, db: &State<DB>) -> Template {
    #[derive(Serialize)]
    struct RenderEntry {
        pub id: u32,
        pub oid: String,
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
        pub tags: Vec<Tag>,
        pub pages: Vec<String>,
        pub ehentai: Option<String>,
        pub nhentai: Option<String>,
        pub imgur: Option<String>,
        pub source: Option<String>,
        pub benzene: bool,
    }
    let entries = db
        .entries()
        .await
        .unwrap()
        .iter()
        .map(|entry| RenderEntry {
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
            tags: entry.tags.to_owned(),
            pages: entry.pages.to_owned(),
            ehentai: entry.ehentai.to_owned(),
            nhentai: entry.nhentai.to_owned(),
            imgur: entry.imgur.to_owned(),
            source: entry.source.to_owned(),
            benzene: entry.benzene,
        })
        .collect::<Vec<_>>();
    #[derive(Serialize)]
    struct RenderTag {
        id: String,
        name: String,
    }
    let tags = db
        .all_tags()
        .await
        .unwrap()
        .iter()
        .map(|tag| RenderTag {
            id: tag.id.unwrap().to_string(),
            name: tag.name.to_owned(),
        })
        .collect::<Vec<_>>();
    #[derive(Serialize)]
    struct RenderAuthor {
        id: String,
        name: String,
        source: Option<String>,
    }
    let authors = db
        .all_authors()
        .await
        .unwrap()
        .iter()
        .map(|author| RenderAuthor {
            id: author.id.unwrap().to_string(),
            name: author.name.to_owned(),
            source: author.source.to_owned(),
        })
        .collect::<Vec<_>>();
    Template::render(
        "add_entry",
        context! {
            entries: entries,
            tags: tags,
            authors: authors,
        },
    )
}

#[post("/add/entry", data = "<entry>")]
pub async fn add_entry_post(_admin: Admin, db: &State<DB>, entry: Json<EntryCreate>) -> Status {
    db.add_entry(Entry {
        oid: None,
        id: Some(0),
        title: entry.title.to_owned(),
        description: entry.description.to_owned(),
        parody: entry.parody.to_owned(),
        author_name: entry.author_name.to_owned(),
        author_id: ObjectId::from_str(&entry.author_id).unwrap(),
        thumbnail: entry.thumbnail.to_owned(),
        pair: entry.pair.to_owned(),
        tier: entry.tier.to_owned(),
        rating: 0_f32,
        rated_by: 0,
        favorites: 0,
        tags: entry
            .tags
            .iter()
            .map(|tag| Tag {
                id: Some(ObjectId::from_str(&tag.id).unwrap()),
                name: tag.name.to_owned(),
            })
            .collect(),
        pages: entry.pages.to_owned(),
        ehentai: entry.ehentai.to_owned(),
        nhentai: entry.nhentai.to_owned(),
        imgur: entry.imgur.to_owned(),
        source: entry.source.to_owned(),
        benzene: entry.benzene,
    })
    .await
    .unwrap();
    Status::Created
}
