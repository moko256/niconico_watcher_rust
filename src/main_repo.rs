use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use log::info;
use reqwest::Client;

use crate::model::Repo;
use crate::nicovideo;
use crate::vo::*;

pub struct MainRepo {
    pub client: Client,
    pub query: String,
}
#[async_trait]
impl Repo for MainRepo {
    async fn get_videos(&self, filter_time_latest_equal: &DateTime<Utc>) -> Option<Vec<NicoVideo>> {
        Some(
            nicovideo::search(
                &self.client,
                &self.query,
                filter_time_latest_equal.to_rfc3339(),
            )
            .await?
            .data,
        )
    }

    async fn post_message(&self, message: &NicoVideo) {
        info!("{}", message.title);
    }
}
