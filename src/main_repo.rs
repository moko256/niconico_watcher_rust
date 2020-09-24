use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use reqwest::Client;

use crate::vo::*;
use crate::model::Repo;
use crate::nicovideo;

const MAX: i32 = 100;

pub struct MainRepo {
    pub client: Client,
    pub query: String
}
#[async_trait]
impl Repo for MainRepo {
    async fn get_videos(&self, filter_time_latest_equal: &DateTime<Utc>) -> Option<Vec<NicoVideo>>{
        Some(nicovideo::search(&self.client, &self.query, MAX).await?.data)
    }

    async fn post_message(&self, message: &NicoVideo) {
        panic!("a")
    }
}