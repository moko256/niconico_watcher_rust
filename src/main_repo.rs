use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use chrono::SecondsFormat;
use log::info;
use reqwest::Client;

use crate::model::Repo;
use crate::nicovideo;
use crate::req_discord_post::ReqDiscordPost;
use crate::vo::*;

pub struct MainRepo {
    pub http: Client,
    pub discord: ReqDiscordPost,
    pub query: String,
}
#[async_trait]
impl Repo for MainRepo {
    async fn get_videos(&self, filter_time_latest_equal: &DateTime<Utc>) -> Option<Vec<NicoVideo>> {
        Some(
            nicovideo::search(
                &self.http,
                &self.query,
                filter_time_latest_equal.to_rfc3339_opts(SecondsFormat::Millis, true),
            )
            .await?
            .data,
        )
    }

    async fn post_message(&mut self, message: &NicoVideo) {
        // New: sm000 "title"
        info!(target: "nicow", "New Movie: {} \"{}\"", message.content_id, message.title);

        //【新着動画】title
        //ttps://nico.ms/sm000
        self.discord
            .post(format!(
                "**【新着動画】**{}\nhttps://nico.ms/{}",
                message.title, message.content_id
            ))
            .await;
    }
}
