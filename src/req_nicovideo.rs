use log::error;

use reqwest::{Client, Response};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::error::Error;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;

use bytes::buf::Buf;

use quick_xml::escape::unescape as entity_unescape;
use rss::Channel;

use form_urlencoded::byte_serialize;

use crate::config::Config;
use crate::vo::*;

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

    pub async fn search(&self) -> Option<Vec<NicoVideo>> {
        let result = self.search_err().await;
        match result {
            Ok(video) => Some(video),
            Err(e) => {
                error!(target: "nicow", "HTTP: {}", e);
                None
            }
        }
    }

    async fn search_err(&self) -> Result<Vec<NicoVideo>, Box<dyn Error>> {
        let response = self.request().await?;
        let response = if response.status() == 200 || response.status() == 404 {
            response
        } else {
            response.error_for_status()?
        };
        self.parse(response).await
    }

    async fn request(&self) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "https://www.nicovideo.jp/tag/{}?rss=rss2&sort=f&order=d&nodescription=1&nothumbnail=1&noinfo=1",
            self.query,
        );
        Ok(self.client.get(&url).send().await?)
    }

    async fn parse(&self, response: Response) -> Result<Vec<NicoVideo>, Box<dyn Error>> {
        let channels = Channel::read_from(response.bytes().await?.reader())?.items;

        let mut videos = Vec::with_capacity(channels.len());
        for channel in channels {
            let title = channel.title().unwrap();
            videos.push(NicoVideo {
                title: String::from_str(
                    entity_unescape(title)
                        .unwrap_or(Cow::Borrowed(title))
                        .borrow(),
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
