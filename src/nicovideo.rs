use log::error;

use reqwest::Client;
use reqwest::Error;

use crate::vo::*;

pub async fn search(
    client: &Client,
    query: &String,
    filter_time_latest_equal: String,
) -> Option<NicoResult> {
    let r = request(
        client,
        format!(
            "https://api.search.nicovideo.jp/api/v2/video/contents/search?q={}&targets=tags&fields=contentId,title,startTime&_sort=-startTime&_limit=100&filters[startTime][gte]={}",
            query,
            filter_time_latest_equal.replace("+", "%2B")
        )
    ).await;
    match r {
        Ok(v) => {
            let status_code = v.meta.status;
            if status_code == 200 {
                Some(v)
            } else {
                error!("HTTP Response != 200: {}", status_code);
                None
            }
        }
        Err(e) => {
            error!("HTTP Error\n{}", e);
            None
        }
    }
}

async fn request(client: &Client, url: String) -> Result<NicoResult, Error> {
    client.get(&url).send().await?.json::<NicoResult>().await
}
