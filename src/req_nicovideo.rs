use log::error;

use reqwest::Client;
use std::error::Error;

use chrono::offset::FixedOffset;
use chrono::DateTime;
use chrono::Utc;

use bytes::buf::Buf;

use atom_syndication::Feed;

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
        let r = self
            .request(format!(
                "http://www.nicovideo.jp/tag/{}?rss=atom&sort=f&order=d&start={}",
                query,
                start_time_gte
                    .with_timezone::<FixedOffset>(&FixedOffset::east(9 * 3600))
                    .format("%Y-%m-%d")
                    .to_string(),
            ))
            .await;
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

    async fn request(&self, url: String) -> Result<NicoResult, Box<dyn Error>> {
        let response = (&self.client).get(&url).send().await?;
        let status = response.status().as_u16() as i64;
        let feeds = Feed::read_from(response.bytes().await?.reader())?.entries;

        let mut videos = Vec::with_capacity(feeds.len());
        for feed in feeds {
            videos.push(NicoVideo {
                title: feed.title,
                content_id: feed.id.split("/").collect::<Vec<&str>>()[2].to_string(),
                start_time: feed.published.unwrap().with_timezone::<Utc>(&Utc),
            })
        }

        Ok(NicoResult {
            data: videos,
            meta: NicoMeta { status },
        })
    }
}
