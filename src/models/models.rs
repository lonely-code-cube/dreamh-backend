use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: Option<String>,
    pub author_name: String,
    pub author_id: ObjectId,
    pub thumbnail: String,
    pub pair: String,
    pub tier: String,
    pub rating: u32,
    pub favorites: u32,
    pub tags: Vec<Tag>,
    pub pages: Vec<String>,
    pub ehentai: Option<String>,
    pub nhentai: Option<String>,
    pub imgur: Option<String>,
    pub source: Option<String>,
    pub benzene: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Author {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}
