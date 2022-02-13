use async_trait::async_trait;

use crate::vo::*;

pub enum State {
    Unretrieved,
    RetrievedLast { movies: Vec<NicoVideo> },
}

impl State {
    fn movie_not_contains_in_prev(n: &NicoVideo, previous: &Vec<NicoVideo>) -> bool {
        return !previous.iter().any(|p| n.content_id == p.content_id);
    }

    fn movie_newer_than_oldest_prev(n: &NicoVideo, previous: &Vec<NicoVideo>) -> bool {
        let prev_most_old = previous.last();
        return prev_most_old.map_or(true, |next_most_old| {
            n.start_time > next_most_old.start_time
        });
    }

    // Note: this algorithm does not consider when getVideos().len() > api query limit
    pub async fn next_state(&mut self, repo: &mut impl Repo) {
        let data = repo.get_videos().await;
        if let Some(next) = data {
            if let State::RetrievedLast { movies: previous } = self {
                for n in next.iter().rev() {
                    // Exclude when
                    // - The movie is already posted.
                    // - The movie have forgotten whether posted, but older than the oldest in remembered.
                    if State::movie_not_contains_in_prev(n, &previous)
                        && State::movie_newer_than_oldest_prev(n, &previous)
                    {
                        repo.post_message(n).await;
                    }
                }
            }

            *self = State::RetrievedLast { movies: next };
        }
    }
}

#[async_trait]
pub trait Repo {
    async fn get_videos(&self) -> Option<Vec<NicoVideo>>;
    async fn post_message(&mut self, message: &NicoVideo);
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;

    fn test_data() -> (NicoVideo, NicoVideo, NicoVideo) {
        let mut test_data = Vec::new();
        for i in 0..3 {
            test_data.push(NicoVideo {
                title: String::new(),
                content_id: format!("sm{}", i),
                start_time: DateTime::parse_from_rfc3339(&format!("2022-02-1{}T00:00:00Z", i + 3))
                    .unwrap()
                    .with_timezone(&Utc),
            });
        }
        let newer = test_data.pop().unwrap();
        let n = test_data.pop().unwrap();
        let older = test_data.pop().unwrap();
        return (older, n, newer);
    }

    #[test]
    fn test_movie_not_contains_in_prev() {
        let (older, nb, newer) = test_data();
        let (_, n, _) = test_data();
        let contains = false;
        let not_contains = true;
        assert_eq!(State::movie_not_contains_in_prev(&n, &vec![]), not_contains);
        assert_eq!(
            State::movie_not_contains_in_prev(&n, &vec![older]),
            not_contains
        );
        assert_eq!(State::movie_not_contains_in_prev(&n, &vec![nb]), contains);
        assert_eq!(
            State::movie_not_contains_in_prev(&n, &vec![newer]),
            not_contains
        );
    }

    #[test]
    fn test_movie_newer_than_oldest_prev() {
        let (older, nb, newer) = test_data();
        let (_, n, _) = test_data();
        let old = false;
        let new = true;
        assert_eq!(State::movie_newer_than_oldest_prev(&n, &vec![]), new);
        assert_eq!(State::movie_newer_than_oldest_prev(&n, &vec![older]), new);
        assert_eq!(State::movie_newer_than_oldest_prev(&n, &vec![nb]), old);
        assert_eq!(State::movie_newer_than_oldest_prev(&n, &vec![newer]), old);
    }
}
