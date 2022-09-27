use std::str::FromStr;

use anyhow::{Context, Result};
use bson::Document;
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use mongodb::{bson::doc, options::FindOptions};

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

    // pub async fn tag(&self, id: String) -> Result<Option<Tag>> {
    //     let tag = self
    //         .conn
    //         .database("data")
    //         .collection::<Tag>("tags")
    //         .find_one(doc! {"_id": id}, None)
    //         .await?;
    //     Ok(tag)
    // }

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

    // pub async fn author(&self, id: String) -> Result<Option<Author>> {
    //     let author = self
    //         .conn
    //         .database("data")
    //         .collection::<Author>("authors")
    //         .find_one(doc! {"_id": id}, None)
    //         .await?;
    //     Ok(author)
    // }

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

    pub async fn entries(&self, limit: Option<u32>, page: Option<u32>) -> Result<Vec<Entry>> {
        let mut entriess_cursor = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .find(
                doc! {"id": {"$gt": page.unwrap_or(0) * limit.unwrap_or(50)}},
                FindOptions::builder()
                    .limit(Some(limit.unwrap_or(50) as i64))
                    .build(),
            )
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

    pub async fn recent(&self, limit: Option<u32>, page: Option<u32>) -> Result<Vec<Entry>> {
        let mut entriess_cursor = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .aggregate(
                vec![
                    doc! {"$sort": {"id": -1}},
                    doc! {"$skip": limit.unwrap_or(50)*page.unwrap_or(0)},
                    doc! {"$limit": limit},
                ],
                None,
            )
            .await?;
        let mut entries: Vec<Entry> = vec![];
        while let Some(entry) = entriess_cursor.try_next().await? {
            let entry: Entry = bson::from_document(entry)?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub async fn top_tier(&self, limit: Option<u32>, page: Option<u32>) -> Result<Vec<Entry>> {
        let mut entriess_cursor = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .aggregate(
                vec![
                    doc! {"$match": {
                        "tier": "S"
                    }},
                    doc! {"$unionWith": {"coll": "entries", "pipeline": [
                                {"$match":
                                {
                                "tier":
                                {"$ne": "S"}
                            }
                        }, {"$sort": {"tier": 1}}
                        ]
                    }},
                    doc! {"$skip": limit.unwrap_or(50)*page.unwrap_or(0)},
                    doc! {"$limit": limit},
                ],
                None,
            )
            .await?;
        let mut entries: Vec<Entry> = vec![];
        while let Some(entry) = entriess_cursor.try_next().await? {
            let entry: Entry = bson::from_document(entry)?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub async fn popular(&self, limit: Option<u32>, page: Option<u32>) -> Result<Vec<Entry>> {
        let mut entriess_cursor = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .aggregate(
                vec![
                    doc! {"$sort": {"favorites": 1}},
                    doc! {"$skip": limit.unwrap_or(50)*page.unwrap_or(0)},
                    doc! {"$limit": limit},
                ],
                None,
            )
            .await?;
        let mut entries: Vec<Entry> = vec![];
        while let Some(entry) = entriess_cursor.try_next().await? {
            let entry: Entry = bson::from_document(entry)?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub async fn top_rated(&self, limit: Option<u32>, page: Option<u32>) -> Result<Vec<Entry>> {
        let mut entriess_cursor = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .aggregate(
                vec![
                    doc! {"$sort": {"rating": 1}},
                    doc! {"$skip": limit.unwrap_or(50)*page.unwrap_or(0)},
                    doc! {"$limit": limit},
                ],
                None,
            )
            .await?;
        let mut entries: Vec<Entry> = vec![];
        while let Some(entry) = entriess_cursor.try_next().await? {
            let entry: Entry = bson::from_document(entry)?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub async fn search(
        &self,
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
    ) -> Result<Vec<Entry>> {
        let mut pipeline: Vec<Document> = vec![];
        match query {
            Some(ref query) => pipeline.push(doc! {
                "$search": {
                    "index": "entry",
                    "text": {
                        "query": query,
                        "path": {
                            "wildcard": "*",
                        },
                    }
                }
            }),
            None => {}
        }
        match include_tags {
            Some(tags) => {
                for tag in tags {
                    pipeline.push(doc! {
                        "$match": {
                            "tags.name": tag,
                        }
                    });
                }
            }
            None => {}
        }
        match exclude_tags {
            Some(tags) => {
                for tag in tags {
                    pipeline.push(doc! {
                        "$match": {
                            "tags.name": {
                                "$ne": tag,
                            },
                        }
                    });
                }
            }
            None => {}
        }
        match include_authors {
            Some(authors) => pipeline.push(doc! {"$match": {
                "author_name": {
                    "$in": authors,
                }
            }}),
            None => {}
        }
        match exclude_authors {
            Some(authors) => pipeline.push(doc! {"$match": {
                "author_name": {
                    "$nin": authors,
                }
            }}),
            None => {}
        }
        match include_tiers {
            Some(tiers) => pipeline.push(doc! {"$match": {
                "tier": {
                    "$in": tiers,
                }
            }}),
            None => {}
        }
        match exclude_tiers {
            Some(tiers) => pipeline.push(doc! {"$match": {
                "tier": {
                    "$nin": tiers,
                }
            }}),
            None => {}
        }
        match rating_above {
            Some(rating) => pipeline.push(doc! {"$match": {
                "rating": {
                    "$gte": rating,
                }
            }}),
            None => {}
        }
        match rating_below {
            Some(rating) => pipeline.push(doc! {"$match": {
                "rating": {
                    "$lte": rating,
                }
            }}),
            None => {}
        }
        match include_parodies {
            Some(parodies) => pipeline.push(doc! {"$match": {
                "parody": {
                    "$in": parodies,
                }
            }}),
            None => {}
        }
        match exclude_parodies {
            Some(parodies) => pipeline.push(doc! {"$match": {
                "parody": {
                    "$nin": parodies,
                }
            }}),
            None => {}
        }
        match include_pair {
            Some(pair) => pipeline.push(doc! {"$match": {
                "pair": {
                    "$in": pair,
                }
            }}),
            None => {}
        }
        match exclude_pair {
            Some(pair) => pipeline.push(doc! {"$match": {
                "pair": {
                    "$nin": pair,
                }
            }}),
            None => {}
        }
        match benzene {
            Some(benzene) => pipeline.push(doc! {"$match": {
                "benzene": benzene,
            }}),
            None => {}
        }
        match max_page {
            Some(page) => pipeline.push(doc! {"$match": {
                "pages": {
                    "$size": {
                        "$lte": page,
                    }
                }
            }}),
            None => {}
        }
        match min_page {
            Some(page) => pipeline.push(doc! {"$match": {
                "pages": {
                    "$size": {
                        "$gte": page,
                    }
                }
            }}),
            None => {}
        }

        pipeline.push(doc! {"$skip": limit.unwrap_or(50)*page.unwrap_or(0)});
        pipeline.push(doc! {"$limit": limit.unwrap_or(50)});

        let mut entriess_cursor = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .aggregate(pipeline, None)
            .await?;
        let mut entries: Vec<Entry> = vec![];
        while let Some(entry) = entriess_cursor.try_next().await? {
            let entry: Entry = bson::from_document(entry)?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub async fn entry(&self, id: u32) -> Result<Option<Entry>> {
        let entry = self
            .conn
            .database("data")
            .collection::<Entry>("entries")
            .find_one(doc! {"id": id}, None).await?;
        Ok(entry)
    }
}
