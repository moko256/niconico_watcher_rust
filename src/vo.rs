use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

// Value Objects

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NicoMeta {
    pub total_count: i64,
    pub status: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NicoVideo {
    pub title: String,
    pub content_id: String,
    pub start_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct NicoResult {
    pub data: Vec<NicoVideo>,
    pub meta: NicoMeta,
}