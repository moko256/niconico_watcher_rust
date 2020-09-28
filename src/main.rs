extern crate chrono;
extern crate cron;

use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use cron::Schedule;
use log::info;
use log::LevelFilter;
use reqwest::Client;
use simple_logger::SimpleLogger;

mod tests;

mod config;
mod main_repo;
mod model;
mod nicovideo;
mod req_discord_post;
mod req_discord_status;
mod time;
mod vo;

use config::*;
use main_repo::MainRepo;
use model::State;
use req_discord_post::ReqDiscordPost;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let config = load_conf();

    req_discord_status::set_status(&config).await;

    let req_discord_post = ReqDiscordPost::new(&config).await;
    let mut repo: MainRepo = MainRepo {
        http: Client::new(),
        discord: req_discord_post,
        query: config.keyword,
    };

    let init_time: DateTime<Utc> = Utc::now();
    //DateTime::parse_from_rfc3339("2020-09-28T00:00:00Z").unwrap().with_timezone(&Utc);

    let mut last_state: State = State {
        latest_time: init_time,
        movie_latest_time: Vec::with_capacity(0),
    };

    info!(target: "main", "Ready.");
    for nt in Schedule::from_str(&config.cron).unwrap().upcoming(Utc) {
        time::wait_until(nt.with_timezone(&Utc)).await;

        last_state = model::next_state(last_state, &mut repo).await;
    }
}
