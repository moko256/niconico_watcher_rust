extern crate chrono;
extern crate cron;

use std::str::FromStr;
use std::time::Duration;
use std::convert::TryFrom;

use chrono::DateTime;
use chrono::Utc;
use cron::Schedule;
use reqwest::Client;
use log::LevelFilter;
use log::info;
use simple_logger::SimpleLogger;

mod tests;

mod model;
mod main_repo;
mod nicovideo;
mod vo;

use model::State;
use main_repo::MainRepo;

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let cron_config: &str = "* * * * * * *";
    let query: String = "Stormworks".to_string();

    let repo: MainRepo = MainRepo {
        client: Client::new(),
        query: query
    };

    let init_time: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-09-23T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

        let mut last_state: State = State {
        latest_time: init_time,
        movie_latest_time: Vec::with_capacity(0)
    };

    for nt in Schedule::from_str(cron_config).unwrap().upcoming(Utc).take(1) {
        let wait_s = Duration::from_secs(
            TryFrom::try_from((nt - Utc::now()).num_seconds()).unwrap(),
        );
        tokio::time::delay_for(wait_s).await;

        info!("{}", nt.format("%F %R").to_string());

        last_state = model::next_state(last_state, &repo).await;
    }
}
