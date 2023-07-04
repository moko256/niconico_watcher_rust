use reqwest::Client;
use serde::Serialize;

use crate::{config::MisskeyConfig, vo::NicoVideo};

pub struct ReqMisskey {
    client: Client,
    config: MisskeyConfig,
}

impl ReqMisskey {
    pub fn new(config: &MisskeyConfig) -> ReqMisskey {
        let client = Client::builder()
            .user_agent(env!("CARGO_PKG_NAME"))
            .pool_max_idle_per_host(0) // api server close connection in about 90 secs
            .tcp_keepalive(None)
            .build()
            .unwrap();
        let config: MisskeyConfig = config.clone();
        ReqMisskey { client, config }
    }

    pub async fn post(&mut self, video: &NicoVideo) {
        let result = self.post_in(video).await;

        if let Err(err) = result {
            log::error!(
                "Failed to post `https://nico.ms/{}` to Misskey: {}",
                video.content_id,
                err
            );
        }
    }

    async fn post_in(&mut self, video: &NicoVideo) -> Result<(), reqwest::Error> {
        //【新着動画】title
        //ttps://nico.ms/sm000
        //
        //footer
        let msg = format!(
            "**【新着動画】**{}\nhttps://nico.ms/{}\n\n{}",
            video.title, video.content_id, self.config.post_footer,
        );

        let url = format!("https://{}/api/notes/create", self.config.server_domain);
        let request_body = RequestBodyCreateNote {
            i: self.config.token.to_string(),
            text: msg,
            local_only: true,
            no_extract_mentions: true,
        };

        let result = self.client.post(&url).json(&request_body).send().await?;

        result.error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestBodyCreateNote {
    i: String,
    text: String,
    local_only: bool,
    no_extract_mentions: bool,
}
