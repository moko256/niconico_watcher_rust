use log::info;
use serenity::async_trait;
use serenity::Client;
use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::client::EventHandler;
use serenity::model::gateway::Activity;
use serenity::model::gateway::ActivityType;

use crate::config::Config;

pub async fn set_status(config: &Config) {
    if !config.dryrun {
        let client = Client::new(config.token.to_string())
            .event_handler(Handler)
            .await
            .unwrap();
        tokio::spawn(async {
            let mut client = client;
            client.start().await.unwrap();
        });
    }
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(gen_status()).await;
        info!(target: "Discord", "Set status.");
    }
}

fn gen_status() -> Activity {
    unsafe { // I am very sad, but this is a necessary magic…
        let mut s = Activity::listening("ニコニコ動画(Re)");
        let v = 3;
        let vp = (&v as *const i32) as *const ActivityType;
        s.kind = *vp;
        return s
    }
}