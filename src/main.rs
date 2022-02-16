use std::str::FromStr;

use chrono::Utc;
use cron::Schedule;
use log::info;
use log::warn;

mod app_logger;
mod config;
mod main_repo;
mod model;
mod req_discord;
mod req_nicovideo;
mod time;
mod vo;

use app_logger::AppLogger;
use config::*;
use main_repo::MainRepo;
use model::State;
use req_discord::ReqDiscord;
use req_nicovideo::ReqNicoVideo;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    AppLogger::init().unwrap();

    let config = load_conf();
    if config.dryrun {
        warn!(target: "nicow", ".env: Running dry-run mode.");
    }

    let mut repo: MainRepo = MainRepo {
        nico: ReqNicoVideo::new(&config),
        discord: ReqDiscord::try_new(&config).await,
    };

    let mut state = State::Unretrieved;

    info!(target: "nicow", "main: Ready.");

    state.next_state(&mut repo).await;

    for nt in Schedule::from_str(&config.cron).unwrap().after(&Utc::now()) {
        time::wait_until(nt).await;

        state.next_state(&mut repo).await;
    }
}
