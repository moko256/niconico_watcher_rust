use log::info;
use serenity::async_trait;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::http::Http;
use serenity::model::gateway::Activity;
use serenity::model::gateway::ActivityType;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::Client;
use std::sync::Arc;

use crate::config::Config;

pub struct ReqDiscord {
    http: Arc<Http>,
    ch: Vec<ChannelId>,
}

impl ReqDiscord {
    pub async fn try_new(config: &Config) -> Option<ReqDiscord> {
        if !config.dryrun {
            let mut client = Client::builder(&config.token)
                .event_handler(Handler)
                .await
                .unwrap();

            let http = Arc::clone(&client.cache_and_http.http);

            tokio::spawn(async move {
                client.start().await.unwrap();
            });

            let ch = config.chid.iter().map(|id| ChannelId(*id)).collect();
            Some(ReqDiscord { http, ch })
        } else {
            None
        }
    }

    pub async fn post(&mut self, msg: String) {
        for ch in self.ch.iter() {
            ch.say(Arc::clone(&self.http), &msg).await.unwrap();
        }
    }
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(activity_watching()).await;
        info!(target: "nicow", "Discord: Set status.");
    }
}

fn activity_watching() -> Activity {
    // I am very sad, but this is a necessary magic…
    let mut s = Activity::listening("ニコニコ動画(𝚁𝚎)");
    let v = 3;
    unsafe {
        let vp = (&v as *const i32) as *const ActivityType;
        s.kind = *vp;
    }
    s
}
