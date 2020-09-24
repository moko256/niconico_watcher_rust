use std::future::Future;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use reqwest::Client;

use crate::vo::*;
use crate::model;
use crate::model::Repo;
use crate::nicovideo;

const max: i32 = 100;

pub struct MainRepo {
    pub client: Client,
    pub query: String
}
#[async_trait]
impl Repo for MainRepo {
    async fn get_videos(&self, filterTimeLatestOrEqual: &DateTime<Utc>) -> Option<Vec<NicoVideo>>{
        Some(nicovideo::search(&self.client, &self.query, max).await?.data)
    }

    async fn post_message(&self, message: &NicoVideo) {
        panic!("a")
    }
}