use chrono::DateTime;
use chrono::Utc;

// Value Objects
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NicoVideo {
    pub title: String,
    pub content_id: String,
    pub url: String,
    pub start_time: DateTime<Utc>,
}

impl NicoVideo {
    pub fn new(title: String, content_id: String, start_time: DateTime<Utc>) -> Self {
        let url = Self::content_id_to_url(&content_id);

        NicoVideo {
            title,
            content_id,
            url,
            start_time,
        }
    }

    fn content_id_to_url(content_id: &str) -> String {
        return format!("https://www.nicovideo.jp/watch/{}", content_id);
    }
}
