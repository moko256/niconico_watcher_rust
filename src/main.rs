extern crate chrono;
extern crate cron;

use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use cron::Schedule;
use log::LevelFilter;
use reqwest::Client;
use simple_logger::SimpleLogger;

mod tests;

mod main_repo;
mod model;
mod nicovideo;
mod time;
mod vo;

use main_repo::MainRepo;
use model::State;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let cron_config: &str = "0 0/10 * * * * *";
    let query: String = "Stormworks".to_string();

    let repo: MainRepo = MainRepo {
        client: Client::new(),
        query,
    };

    let init_time: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-09-23T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let mut last_state: State = State {
        latest_time: init_time,
        movie_latest_time: Vec::with_capacity(0),
    };

    for nt in Schedule::from_str(cron_config).unwrap().upcoming(Utc) {
        time::wait_until(nt.with_timezone(&Utc)).await;

        last_state = model::next_state(last_state, &repo).await;
    }
}
