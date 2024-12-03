use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dataset {
    pub _id: String,
    pub id: String,
    pub author: String,
    pub disabled: bool,
    pub gated: serde_json::Value,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    pub likes: i64,
    #[serde(rename = "trendingScore")]
    pub trending_score: f64,
    pub private: bool,
    pub sha: String,
    pub description: Option<String>,
    pub downloads: i64,
    pub tags: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub key: String,
}
