use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use cron::Schedule;
use log::info;
use log::warn;

mod tests;

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

    let init_time: DateTime<Utc> = Utc::now();
    //DateTime::parse_from_rfc3339("2020-09-28T00:00:00Z").unwrap().with_timezone(&Utc);

    let mut state = State::new(init_time);

    info!(target: "nicow", "main: Ready.");
    for nt in Schedule::from_str(&config.cron).unwrap().upcoming(Utc) {
        time::wait_until(nt.with_timezone(&Utc)).await;

        state.next_state(&mut repo).await;
    }
}
