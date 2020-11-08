use log::error;

use reqwest::Client;
use reqwest::Error;

use chrono::DateTime;
use chrono::SecondsFormat;
use chrono::Utc;

use crate::vo::*;

pub async fn search(
    client: &Client,
    query: &str,
    start_time_gte: &DateTime<Utc>,
) -> Option<NicoResult> {
    let r = request(
        client,
        format!(
            "https://api.search.nicovideo.jp/api/v2/video/contents/search?q={}&targets=tags&fields=contentId,title,startTime&_sort=-startTime&_limit=100&filters[startTime][gte]={}",
            query,
            start_time_gte.to_rfc3339_opts(SecondsFormat::Millis, true),
        )
    ).await;
    match r {
        Ok(v) => {
            let status_code = v.meta.status;
            if status_code == 200 {
                Some(v)
            } else {
                error!(target: "nicow", "HTTP: Status {} != 200", status_code);
                None
            }
        }
        Err(e) => {
            error!(target: "nicow", "HTTP: {}", e);
            None
        }
    }
}

async fn request(client: &Client, url: String) -> Result<NicoResult, Error> {
    client.get(&url).send().await?.json::<NicoResult>().await
}
