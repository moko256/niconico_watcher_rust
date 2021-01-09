use log::error;

use reqwest::Client;
use reqwest::Error;

use chrono::DateTime;
use chrono::SecondsFormat;
use chrono::Utc;

use crate::vo::*;

pub struct ReqNicoVideo {
    client: Client,
}

impl ReqNicoVideo {
    pub fn new() -> ReqNicoVideo {
        let client = Client::builder()
            .user_agent(env!("CARGO_PKG_NAME"))
            .pool_max_idle_per_host(0) // api server close connection in about 90 secs
            .tcp_keepalive(None)
            .build()
            .unwrap();

        ReqNicoVideo { client }
    }

    pub async fn search(&self, query: &str, start_time_gte: &DateTime<Utc>) -> Option<NicoResult> {
        let r = self.request(
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

    async fn request(&self, url: String) -> Result<NicoResult, Error> {
        (&self.client)
            .get(&url)
            .send()
            .await?
            .json::<NicoResult>()
            .await
    }
}
