use chrono::DateTime;
use chrono::Utc;

// Value Objects

pub struct NicoMeta {
    pub status: u16,
}

pub struct NicoVideo {
    pub title: String,
    pub content_id: String,
    pub start_time: DateTime<Utc>,
}

pub struct NicoResult {
    pub data: Vec<NicoVideo>,
    pub meta: NicoMeta,
}
