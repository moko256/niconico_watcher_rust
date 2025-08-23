use log::error;

use reqwest::{Client, Response};
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::error::Error;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;

use quick_xml::escape::unescape as entity_unescape;

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
                error!("HTTP: {}", e);
                None
            }
        }
    }

    async fn search_err(&self) -> Result<Vec<NicoVideo>, Box<dyn Error>> {
        let response = self.request().await?;
        let response = if response.status().is_success() {
            response
        } else {
            response.error_for_status()?
        };
        self.parse(response).await
    }

    async fn request(&self) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "https://snapshot.search.nicovideo.jp/api/v2/snapshot/video/contents/search?q={}&targets=tags&fields=contentId,title,startTime&_sort=-startTime&_limit={}",
            self.query, MAX_VIDEO_COUNT,
        );
        Ok(self.client.get(&url).send().await?)
    }

    async fn parse(&self, response: Response) -> Result<Vec<NicoVideo>, Box<dyn Error>> {
        let result = response.json::<NicoVideoApiResult>().await?;

        let mut videos = Vec::with_capacity(result.data.len());
        for video in result.data {
            let raw_title = video.title;

            let title = String::from_str(
                entity_unescape(&raw_title)
                    .unwrap_or(Cow::Borrowed(&raw_title))
                    .borrow(),
            )?;

            let content_id = video.content_id;
            let start_time = video.start_time;

            videos.push(NicoVideo::new(title, content_id, start_time))
        }

        Ok(videos)
    }
}

const MAX_VIDEO_COUNT: u8 = 20;

#[derive(Clone, Serialize, Deserialize)]
struct NicoVideoApiResult {
    pub data: Vec<NicoVideoApiVideo>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NicoVideoApiVideo {
    pub title: String,
    pub content_id: String,
    pub start_time: DateTime<Utc>,
}
