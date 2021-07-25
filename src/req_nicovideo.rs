use log::error;

use once_cell::sync::Lazy;
use reqwest::{Client, Response};
use std::borrow::Cow;
use std::error::Error;

use chrono::offset::FixedOffset;
use chrono::DateTime;
use chrono::Utc;

use bytes::buf::Buf;

use quick_xml::escape::unescape as entity_unescape;
use rss::Channel;

use form_urlencoded::byte_serialize;

use crate::config::Config;
use crate::vo::*;

static JST: Lazy<FixedOffset> = Lazy::new(|| FixedOffset::east(9 * 3600));

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

    pub async fn search(&self, start_time: &DateTime<Utc>) -> Option<Vec<NicoVideo>> {
        let result = self.search_err(start_time).await;
        match result {
            Ok(video) => Some(video),
            Err(e) => {
                error!(target: "nicow", "HTTP: {}", e);
                None
            }
        }
    }

    async fn search_err(
        &self,
        start_time: &DateTime<Utc>,
    ) -> Result<Vec<NicoVideo>, Box<dyn Error>> {
        let response = self.request(start_time).await?;
        let response = if response.status() == 200 || response.status() == 404 {
            response
        } else {
            response.error_for_status()?
        };
        self.parse(response).await
    }

    async fn request(&self, start_time_gte: &DateTime<Utc>) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "https://www.nicovideo.jp/tag/{}?rss=rss2&sort=f&order=d&start={}&nodescription=1&nothumbnail=1&noinfo=1",
            self.query,
            start_time_gte
                .with_timezone(&*JST)
                .format("%Y-%m-%d")
                .to_string(),
        );
        self.client.get(&url).send().await.map_err(|e| Box::from(e))
    }

    async fn parse(&self, response: Response) -> Result<Vec<NicoVideo>, Box<dyn Error>> {
        let channels = Channel::read_from(response.bytes().await?.reader())?.items;

        let mut videos = Vec::with_capacity(channels.len());
        for channel in channels {
            let title = channel.title().unwrap().as_bytes();
            videos.push(NicoVideo {
                title: String::from_utf8(
                    entity_unescape(title).unwrap_or(Cow::Borrowed(title)).into_owned(),
                )?,
                content_id: channel
                    .guid()
                    .unwrap()
                    .value
                    .split('/')
                    .next_back()
                    .unwrap()
                    .to_owned(),
                start_time: DateTime::parse_from_rfc2822(channel.pub_date().unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
            })
        }

        Ok(videos)
    }
}
