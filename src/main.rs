extern crate chrono;
extern crate cron;

use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use cron::Schedule;
use log::info;
use reqwest::Client;

mod tests;

mod app_logger;
mod config;
mod main_repo;
mod model;
mod nicovideo;
mod req_discord;
mod time;
mod vo;

use app_logger::AppLogger;
use config::*;
use main_repo::MainRepo;
use model::State;
use req_discord::ReqDiscord;

#[tokio::main]
async fn main() {
    AppLogger::init().unwrap();

    let config = load_conf();

    let req_discord = ReqDiscord::try_new(&config).await;

    let mut repo: MainRepo = MainRepo {
        http: Client::new(),
        discord: req_discord,
        query: config.keyword,
    };

    let init_time: DateTime<Utc> = Utc::now();
    //DateTime::parse_from_rfc3339("2020-09-28T00:00:00Z").unwrap().with_timezone(&Utc);

    let mut last_state: State = State {
        latest_time: init_time,
        movie_latest_time: Vec::with_capacity(0),
    };

    info!(target: "nicow", "main: Ready.");
    for nt in Schedule::from_str(&config.cron).unwrap().upcoming(Utc) {
        time::wait_until(nt.with_timezone(&Utc)).await;

        last_state = model::next_state(last_state, &mut repo).await;
    }
}
