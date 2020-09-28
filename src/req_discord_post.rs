use serenity::model::id::ChannelId;
use serenity::http::client::Http;

use crate::config::Config;

pub struct ReqDiscordPost {
    http: Option<Http>,
    ch: ChannelId,
}

impl ReqDiscordPost {
    pub async fn new(config: &Config) -> ReqDiscordPost {
        let ch = ChannelId(config.chid.parse().unwrap());
        if !config.dryrun {
            ReqDiscordPost {
                http: Some(Http::new_with_token(&config.token)),
                ch,
            }
        } else {
            ReqDiscordPost {
                http: None,
                ch,
            }
        }
    }

    pub async fn post(&mut self, msg: String) {
        if let Some(http) = &self.http {
            self.ch.say(http, msg).await.unwrap();
        }
    }
}