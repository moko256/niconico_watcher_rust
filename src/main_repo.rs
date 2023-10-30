use async_trait::async_trait;
use log::info;

use crate::model::Repo;
use crate::req_discord::ReqDiscord;
use crate::req_misskey::ReqMisskey;
use crate::req_nicovideo::ReqNicoVideo;
use crate::vo::*;

pub struct MainRepo {
    pub nico: ReqNicoVideo,
    pub discord: Option<ReqDiscord>,
    pub misskey: Option<ReqMisskey>,
}

#[async_trait]
impl Repo for MainRepo {
    async fn get_videos(&self) -> Option<Vec<NicoVideo>> {
        Some(self.nico.search().await?)
    }

    async fn post_message(&mut self, message: &NicoVideo) {
        // New movie: sm000 "title"
        info!("New Movie: {} \"{}\"", message.content_id, message.title);

        // Post functions.
        let handle_discord = async {
            if let Some(discord) = &mut self.discord {
                discord.post(message).await;
            }
        };

        let handle_misskey = async {
            if let Some(misskey) = &mut self.misskey {
                misskey.post(message).await;
            }
        };

        // Wait posting.
        tokio::join!(handle_discord, handle_misskey);
    }
}
