use chrono::DateTime;
use chrono::Utc;

// Value Objects
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NicoVideo {
    pub title: String,
    pub content_id: String,
    pub start_time: DateTime<Utc>,
}
