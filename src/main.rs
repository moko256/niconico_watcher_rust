extern crate chrono;
extern crate cron;

use std::str::FromStr;
use std::time::Duration;
use std::convert::TryFrom;

use chrono::DateTime;
use chrono::Utc;
use cron::Schedule;
use reqwest::Client;

mod tests;

mod model;
mod main_repo;
mod nicovideo;
mod vo;

use model::State;
use main_repo::MainRepo;
use vo::*;


const cron_config: &str = "* * * * * * *";

#[tokio::main]
async fn main() {
    let query: String = "stormworks".to_string();

    let repo: MainRepo = MainRepo {
        client: Client::new(),
        query: query
    };

    let mut init_time: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-09-23T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let mut last_state: State = State {
        latest_time: init_time,
        movie_latest_time: Vec::with_capacity(0)
    };

    for nt in Schedule::from_str(cron_config).unwrap().upcoming(Utc).take(2) {
        let wait_s = Duration::from_secs(
            TryFrom::try_from((nt - Utc::now()).num_seconds()).unwrap(),
        );
        tokio::time::delay_for(wait_s).await;

        println!("{}", nt.format("%F %R").to_string());

        last_state = model::next_state(last_state, &repo).await;
    }
}
