use log::error;

use reqwest::Client;
use reqwest::Error;

use crate::vo::*;

pub async fn search(client: &Client, query: &String, limit: i32) -> Option<NicoResult> {
    let r = request(
        client,
        format!(
            "https://api.search.nicovideo.jp/api/v2/video/contents/search?q={}&targets=tags&fields=contentId,title,startTime&_sort=-startTime&_limit={}",
            query,
            limit
        )
    ).await;
    match r {
        Ok(v) => {
            Some(v)
        },
        Err(e) => {
            error!("HTTP Error\n{}", e);
            None
        }
    }
}

async fn request(client: &Client, url: String) -> Result<NicoResult, Error> {
    client
        .get(&url)
        .send()
        .await?
        .json::<NicoResult>()
        .await
}