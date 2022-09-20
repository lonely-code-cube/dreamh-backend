use std::str::FromStr;

use anyhow::{Context, Result};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

use crate::models::models::{Author, Entry, Tag};

pub struct DB {
    conn: Client,
}

impl DB {
    pub async fn connect(uri: &str) -> Result<Self> {
        let db = Client::with_uri_str(uri)
            .await
            .context("Could not connect to database")?;

        Ok(DB { conn: db })
    }

    pub async fn all_tags(&self) -> Result<Vec<Tag>> {
        let mut tags_cursor = self
            .conn
            .database("data")
            .collection::<Tag>("tags")
            .find(None, None)
            .await?;
        let mut tags: Vec<Tag> = vec![];
        while let Some(tag) = tags_cursor.try_next().await? {
            tags.push(tag);
        }
        Ok(tags)
    }

    pub async fn tag(&self, id: String) -> Result<Option<Tag>> {
        let tag = self
            .conn
            .database("data")
            .collection::<Tag>("tags")
            .find_one(doc! {"_id": id}, None)
            .await?;
        Ok(tag)
    }

    pub async fn add_tag(&self, name: String) -> Result<Tag> {
        let x = self
            .conn
            .database("data")
            .collection::<Tag>("tags")
            .insert_one(
                Tag {
                    id: None,
                    name: name.clone(),
                },
                None,
            )
            .await?;
        Ok(Tag {
            id: match x.inserted_id {
                bson::Bson::ObjectId(oid) => Some(oid),
                _ => None,
            },
            name: name,
        })
    }

    pub async fn update_tag(&self, id: String, name: String) -> Result<()> {
        self.conn
            .database("data")
            .collection::<Tag>("tags")
            .update_one(doc! {"_id": id}, doc! {"$set": {"name": name}}, None)
            .await?;
        Ok(())
    }

    pub async fn delete_tag(&self, id: String) -> Result<()> {
        self.conn
            .database("data")
            .collection::<Tag>("tags")
            .delete_one(doc! {"_id": ObjectId::from_str(&id)?}, None)
            .await?;
        Ok(())
    }

    pub async fn all_authors(&self) -> Result<Vec<Author>> {
        let mut authors_cursor = self
            .conn
            .database("data")
            .collection::<Author>("authors")
            .find(None, None)
            .await?;
        let mut authors: Vec<Author> = vec![];
        while let Some(author) = authors_cursor.try_next().await? {
            authors.push(author);
        }
        Ok(authors)
    }

    pub async fn author(&self, id: String) -> Result<Option<Author>> {
        let author = self
            .conn
            .database("data")
            .collection::<Author>("authors")
            .find_one(doc! {"_id": id}, None)
            .await?;
        Ok(author)
    }

    pub async fn add_author(&self, name: String, source: Option<String>) -> Result<Author> {
        let x = self
            .conn
            .database("data")
            .collection::<Author>("authors")
            .insert_one(
                Author {
                    id: None,
                    name: name.clone(),
                    source: source.clone(),
                },
                None,
            )
            .await?;
        Ok(Author {
            id: match x.inserted_id {
                bson::Bson::ObjectId(oid) => Some(oid),
                _ => None,
            },
            name: name,
            source: source,
        })
    }

    pub async fn entries(&self) -> Result<Vec<Entry>> {
        let mut entriess_cursor = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .find(None, None)
            .await?;
        let mut entries: Vec<Entry> = vec![];
        while let Some(entry) = entriess_cursor.try_next().await? {
            entries.push(entry);
        }
        Ok(entries)
    }

    pub async fn add_entry(&self, entry: Entry) -> Result<()> {
        self.conn
            .database("data")
            .collection::<Entry>("entries")
            .insert_one(entry, None)
            .await?;
        Ok(())
    }
}
