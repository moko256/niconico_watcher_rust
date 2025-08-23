use std::str::FromStr;

use chrono::Utc;
use cron::Schedule;
use log::info;
use log::warn;
use moko256_systemd_stdio_logger as logger;

mod config;
mod main_repo;
mod model;
mod req_discord;
mod req_misskey;
mod req_nicovideo;
mod time;
mod vo;

use config::*;
use main_repo::MainRepo;
use model::State;
use req_discord::ReqDiscord;
use req_nicovideo::ReqNicoVideo;

use crate::req_misskey::ReqMisskey;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    logger::init([
        logger::LoggerModuleFilterKey::Module(module_path!(), log::LevelFilter::Info),
        logger::LoggerModuleFilterKey::Default(log::LevelFilter::Warn),
    ])
    .unwrap();

    let config = load_conf();
    if config.dryrun {
        warn!("main: Running dry-run mode.");
    }

    // Create dest repositories.
    let (discord_repo, misskey_repo) = if !config.dryrun {
        let discord_repo = if let Some(config) = &config.discord {
            Some(ReqDiscord::new_async(config).await)
        } else {
            None
        };

        let misskey_repo = config
            .misskey
            .as_ref()
            .map(ReqMisskey::new);

        (discord_repo, misskey_repo)
    } else {
        (None, None)
    };

    // Create repository.
    let mut repo: MainRepo = MainRepo {
        nico: ReqNicoVideo::new(&config),
        discord: discord_repo,
        misskey: misskey_repo,
    };

    let mut state = State::Unretrieved;
    // For test
    // let mut state = State::RetrievedLast {
    //     movies: vec![NicoVideo::new(
    //         String::new(),
    //         String::new(),
    //         DateTime::parse_from_rfc3339("2025-08-18T12:30:00Z")
    //             .unwrap()
    //             .into(),
    //     )],
    // };

    info!("main: Ready.");

    // Acquire current server state.
    state.next_state(&mut repo).await;

    // Acquire with schedule.
    for nt in Schedule::from_str(&config.cron).unwrap().after(&Utc::now()) {
        time::wait_until(nt).await;

        state.next_state(&mut repo).await;
    }
}
