use async_trait::async_trait;
use log::info;

use crate::model::Repo;
use crate::req_discord::ReqDiscord;
use crate::req_nicovideo::ReqNicoVideo;
use crate::vo::*;

pub struct MainRepo {
    pub nico: ReqNicoVideo,
    pub discord: Option<ReqDiscord>,
}

#[async_trait]
impl Repo for MainRepo {
    async fn get_videos(&self) -> Option<Vec<NicoVideo>> {
        Some(self.nico.search().await?)
    }

    async fn post_message(&mut self, message: &NicoVideo) {
        // New movie: sm000 "title"
        info!(target: "nicow", "New Movie: {} \"{}\"", message.content_id, message.title);

        if let Some(discord) = &mut self.discord {
            //【新着動画】title
            //ttps://nico.ms/sm000
            discord
                .post(format!(
                    "**【新着動画】**{}\nhttps://nico.ms/{}",
                    message.title, message.content_id
                ))
                .await;
        }
    }
}
