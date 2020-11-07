use async_trait::async_trait;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use log::info;
use log::LevelFilter;
use reqwest::Client;
use simple_logger::SimpleLogger;
use std::marker::Sync;

use crate::model::*;
use crate::nicovideo;
use crate::vo::*;

#[test]
fn test_no_error() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let time = DateTime::parse_from_rfc3339("2020-09-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let day1 = Duration::days(1);
    let mut r: Vec<NicoVideo> = Vec::new();
    let repo = TestRepo {
        v: || vec![vg("2", time), vg("1", time - day1)],
    };
    let init_state = State {
        latest_time: time,
        movie_latest_time: Vec::new(),
    };
    let state = next_state(init_state, &repo);
}

fn vg(id: &str, datetime: DateTime<Utc>) -> NicoVideo {
    NicoVideo {
        content_id: id.to_string(),
        title: id.to_string(),
        start_time: datetime,
    }
}

pub struct TestRepo<F>
where
    F: Fn() -> (Vec<NicoVideo>) + Sync,
{
    pub v: F,
}

#[async_trait]
impl<F> Repo for TestRepo<F>
where
    F: Fn() -> (Vec<NicoVideo>) + Sync,
{
    async fn get_videos(&self, filter_time_latest_equal: &DateTime<Utc>) -> Option<Vec<NicoVideo>> {
        Some((&self.v)())
    }

    async fn post_message(&mut self, message: &NicoVideo) {
        info!("{}", message.title)
    }
}
