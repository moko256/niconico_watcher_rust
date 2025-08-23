use log::info;
use serenity::Client;
use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use std::sync::Arc;

use crate::config::DiscordConfig;
use crate::vo::NicoVideo;

pub struct ReqDiscord {
    http: Arc<Http>,
    ch: Vec<ChannelId>,
}

impl ReqDiscord {
    pub async fn new_async(config: &DiscordConfig) -> ReqDiscord {
        let mut client = Client::builder(&config.token, Default::default())
            .event_handler(Handler {
                bot_watching_target: config.bot_watching_target.clone(),
            })
            .await
            .unwrap();

        let http = Arc::clone(&client.http);

        tokio::spawn(async move {
            client.start().await.unwrap();
        });

        let ch = config.chid.iter().map(|id| ChannelId::new(*id)).collect();

        ReqDiscord { http, ch }
    }

    pub async fn post(&mut self, video: &NicoVideo) {
        //【新着動画】title
        //https://example.com
        let msg = format!("**【新着動画】**{}\n{}", video.title, video.url);
        for ch in self.ch.iter() {
            let result = ch.say(Arc::clone(&self.http), &msg).await;

            if let Err(err) = result {
                log::error!(
                    "Failed to post `{}` to Discord (chid: {}): {}",
                    video.url,
                    ch,
                    err
                );
            }
        }
    }
}

struct Handler {
    bot_watching_target: String,
}
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Some(ActivityData::watching(&self.bot_watching_target)));
        info!("Discord: Set status.");
    }
}
