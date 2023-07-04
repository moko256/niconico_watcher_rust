use log::info;
use serenity::async_trait;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::http::Http;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::Client;
use std::sync::Arc;

use crate::config::DiscordConfig;

pub struct ReqDiscord {
    http: Arc<Http>,
    ch: Vec<ChannelId>,
}

impl ReqDiscord {
    pub async fn new(config: &DiscordConfig) -> ReqDiscord {
        let mut client = Client::builder(&config.token, Default::default())
            .event_handler(Handler {
                bot_watching_target: config.bot_watching_target.clone(),
            })
            .await
            .unwrap();

        let http = Arc::clone(&client.cache_and_http.http);

        tokio::spawn(async move {
            client.start().await.unwrap();
        });

        let ch = config.chid.iter().map(|id| ChannelId(*id)).collect();

        ReqDiscord { http, ch }
    }

    pub async fn post(&mut self, msg: String) {
        for ch in self.ch.iter() {
            ch.say(Arc::clone(&self.http), &msg).await.unwrap();
        }
    }
}

struct Handler {
    bot_watching_target: String,
}
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Activity::watching(&self.bot_watching_target))
            .await;
        info!(target: "nicow", "Discord: Set status.");
    }
}
