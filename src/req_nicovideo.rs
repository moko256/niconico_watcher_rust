use log::error;

use once_cell::sync::Lazy;
use reqwest::Client;
use std::error::Error;

use chrono::offset::FixedOffset;
use chrono::DateTime;
use chrono::Utc;

use bytes::buf::Buf;

use atom_syndication::Feed;
use quick_xml::escape::unescape as entity_unescape;

use form_urlencoded::byte_serialize;

use crate::config::Config;
use crate::vo::*;

const JST: Lazy<FixedOffset> = Lazy::new(|| FixedOffset::east(9 * 3600));

pub struct ReqNicoVideo {
    client: Client,
    query: String,
}

impl ReqNicoVideo {
    pub fn new(config: &Config) -> ReqNicoVideo {
        let client = Client::builder()
            .user_agent(env!("CARGO_PKG_NAME"))
            .pool_max_idle_per_host(0) // api server close connection in about 90 secs
            .tcp_keepalive(None)
            .build()
            .unwrap();
        let query = byte_serialize(config.keyword.as_bytes()).collect();
        ReqNicoVideo { client, query }
    }

    pub async fn search(&self, start_time_gte: &DateTime<Utc>) -> Option<NicoResult> {
        let r = self
            .request(format!(
                "https://www.nicovideo.jp/tag/{}?rss=atom&sort=f&order=d&start={}",
                self.query,
                start_time_gte
                    .with_timezone(&*JST)
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
        let response = self.client.get(&url).send().await?;
        let status = response.status().as_u16();
        let feeds = Feed::read_from(response.bytes().await?.reader())?.entries;

        let mut videos = Vec::with_capacity(feeds.len());
        for feed in feeds {
            videos.push(NicoVideo {
                title: String::from_utf8(entity_unescape(feed.title.as_bytes())?.into_owned())?,
                content_id: feed.id.split("/").collect::<Vec<&str>>()[2].to_string(),
                start_time: feed.published.unwrap().with_timezone(&Utc),
            })
        }

        Ok(NicoResult {
            data: videos,
            meta: NicoMeta { status },
        })
    }
}
