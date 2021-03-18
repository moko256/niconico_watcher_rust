use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;

use crate::vo::*;

pub struct State {
    pub latest_time: DateTime<Utc>,
    pub movie_latest_time: Vec<NicoVideo>,
}

impl State {
    pub fn new(init_time: DateTime<Utc>) -> Self {
        State {
            latest_time: init_time,
            movie_latest_time: Vec::with_capacity(0),
        }
    }

    pub fn contains_in_movie_latest_time(&self, t: &NicoVideo) -> bool {
        self.movie_latest_time
            .iter()
            .any(|n| n.content_id == t.content_id)
    }

    // Note: this algorithm does not consider when getVideos().len() > api query limit
    pub async fn next_state(&mut self, repo: &mut impl Repo) {
        let last_state_time = self.latest_time;
        let data = repo.get_videos(&last_state_time).await;

        if let Some(mut data) = data {
            if !data.is_empty() {
                let latest_time = data[0].start_time;
                let mut movie_latest_time = Vec::new();
                loop {
                    let n = data.pop();
                    if n.is_none() {
                        break;
                    }
                    let n = n.unwrap();
                    // Post when
                    // * New movies
                    // * Movies that should hit last request, but it wasn't contained and posted.
                    if n.start_time >= last_state_time && (!self.contains_in_movie_latest_time(&n))
                    {
                        repo.post_message(&n).await;
                    }
                    if n.start_time == latest_time {
                        movie_latest_time.push(n);
                    }
                }
                self.latest_time = latest_time;
                self.movie_latest_time = movie_latest_time;
            }
        }
    }
}

#[async_trait]
pub trait Repo {
    async fn get_videos(&self, filter_time_latest_equal: &DateTime<Utc>) -> Option<Vec<NicoVideo>>;
    async fn post_message(&mut self, message: &NicoVideo);
}
