use async_trait::async_trait;
use log::info;

use crate::vo::*;

pub enum State {
    Unretrieved,
    RetrievedLast { movies: Vec<NicoVideo> }, // first: oldest, last: newest
}

impl State {
    fn movie_not_contains_in_prev(target: &NicoVideo, previous: &[NicoVideo]) -> bool {
        !previous
            .iter()
            .any(|prev_any| target.content_id == prev_any.content_id)
    }

    fn movie_newer_than_oldest_prev(target: &NicoVideo, oldest: Option<&NicoVideo>) -> bool {
        oldest.is_none_or(|prev_most_old| target.start_time > prev_most_old.start_time)
    }

    fn movie_newer_eq_than_oldest_prev(target: &NicoVideo, oldest: Option<&NicoVideo>) -> bool {
        oldest.is_none_or(|prev_most_old| target.start_time >= prev_most_old.start_time)
    }

    fn movie_postable(
        target: &NicoVideo,
        previous: &[NicoVideo],
        oldest: Option<&NicoVideo>,
    ) -> bool {
        State::movie_not_contains_in_prev(target, previous)
            && State::movie_newer_than_oldest_prev(target, oldest)
    }

    // Note: this algorithm does not consider when getVideos().len() > api query limit
    pub async fn next_state(&mut self, repo: &mut impl Repo) {
        let data = repo.get_videos().await;
        if let Some(next) = data {
            if let State::RetrievedLast {
                movies: current_movies,
            } = self
            {
                // Remove older video from queue.
                let next_oldest = next.last(); // first: newest, last: oldest
                current_movies
                    .retain(|movie| Self::movie_newer_eq_than_oldest_prev(movie, next_oldest));

                // Collect new videos.
                //
                // Exclude when
                // - The movie is already posted.
                // - The movie have forgotten whether posted, but older than the oldest in remembered.
                let current_oldest = current_movies.first(); // first: oldest, last: newest
                let new_movies: Vec<NicoVideo> = next
                    .into_iter()
                    .rev() // RSS has newer first, but bot must post older first.
                    .filter(|video| State::movie_postable(video, current_movies, current_oldest))
                    .collect();

                // For log.
                let queue_modified = !new_movies.is_empty();

                // Post and add to queue.
                for movie in new_movies.into_iter() {
                    repo.post_message(&movie).await;

                    current_movies.push(movie);
                }

                if queue_modified {
                    info!("Current queue item count: {}", current_movies.len());
                }
            } else {
                *self = State::RetrievedLast {
                    movies: next.into_iter().rev().collect(), // newest: first -> newest: last
                };
            }

            // For test
            // repo.post_message(&next[0]).await;
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

    fn test_datum() -> (NicoVideo, NicoVideo, NicoVideo) {
        let mut test_datum = Vec::new();
        for i in 0..3 {
            test_datum.push(test_data(i));
        }
        let newer = test_datum.pop().unwrap();
        let n = test_datum.pop().unwrap();
        let older = test_datum.pop().unwrap();
        (older, n, newer)
    }

    fn test_data(i: i32) -> NicoVideo {
        let title = String::new();
        let content_id = format!("sm{}", i);
        let start_time = DateTime::parse_from_rfc3339(&format!("2022-02-{:02}T00:00:00Z", i + 13))
            .unwrap()
            .with_timezone(&Utc);

        NicoVideo::new(title, content_id, start_time)
    }

    #[test]
    fn test_movie_not_contains_in_prev() {
        let (older, nb, newer) = test_datum();
        let (_, n, _) = test_datum();
        let contains = false;
        let not_contains = true;
        assert_eq!(State::movie_not_contains_in_prev(&n, &[]), not_contains);
        assert_eq!(
            State::movie_not_contains_in_prev(&n, &[older]),
            not_contains
        );
        assert_eq!(State::movie_not_contains_in_prev(&n, &[nb]), contains);
        assert_eq!(
            State::movie_not_contains_in_prev(&n, &[newer]),
            not_contains
        );
    }

    #[test]
    fn test_movie_newer_than_oldest_prev() {
        let (older, nb, newer) = test_datum();
        let (_, n, _) = test_datum();
        let old = false;
        let new = true;
        assert_eq!(State::movie_newer_than_oldest_prev(&n, None), new);
        assert_eq!(State::movie_newer_than_oldest_prev(&n, Some(&older)), new);
        assert_eq!(State::movie_newer_than_oldest_prev(&n, Some(&nb)), old);
        assert_eq!(State::movie_newer_than_oldest_prev(&n, Some(&newer)), old);
    }

    #[test]
    fn test_movie_postable() {
        let assert = |prev: Vec<NicoVideo>, next: Vec<NicoVideo>, expected: Vec<NicoVideo>| {
            let mut r = Vec::new();
            for n in next {
                if State::movie_postable(&n, &prev, prev.first()) {
                    r.push(n);
                }
            }
            assert_eq!(expected, r);
        };

        assert(
            vec![test_data(1)],
            vec![test_data(1), test_data(2)],
            vec![test_data(2)],
        );
        assert(
            vec![test_data(1), test_data(3)],
            vec![test_data(1), test_data(2), test_data(3)],
            vec![test_data(2)],
        );
        assert(vec![test_data(1), test_data(2)], vec![test_data(1)], vec![]);
        assert(
            vec![test_data(1), test_data(2)],
            vec![test_data(3)],
            vec![test_data(3)],
        );
    }

    // test for next_state()
    struct TestRepo {
        remote_movies: Option<Vec<NicoVideo>>,
        posted_movies: Vec<NicoVideo>,
    }

    #[async_trait]
    impl Repo for TestRepo {
        async fn get_videos(&self) -> Option<Vec<NicoVideo>> {
            self.remote_movies.clone()
        }
        async fn post_message(&mut self, message: &NicoVideo) {
            self.posted_movies.push(message.clone());
        }
    }

    impl TestRepo {
        fn with_movies(movies: &[NicoVideo]) -> Self {
            TestRepo {
                remote_movies: Some(Vec::from(movies)),
                posted_movies: Vec::new(),
            }
        }
    }

    async fn test_next_state_assert(
        movies_retrieve_turn: &[&[NicoVideo]],
        actual_posted: &[NicoVideo],
    ) {
        test_next_state_assert_queue(movies_retrieve_turn, actual_posted, None).await;
    }

    async fn test_next_state_assert_queue(
        movies_retrieve_turn: &[&[NicoVideo]],
        actual_posted: &[NicoVideo],
        queue: Option<&[NicoVideo]>,
    ) {
        let mut posted: Vec<NicoVideo> = Vec::new();

        let mut state = State::Unretrieved;
        for movies in movies_retrieve_turn {
            let mut repo = TestRepo::with_movies(movies);
            state.next_state(&mut repo).await;
            posted.append(&mut repo.posted_movies);
        }
        assert_eq!(actual_posted, posted);

        if let Some(queue) = queue {
            if let State::RetrievedLast { movies } = state {
                assert_eq!(queue, movies);
            } else {
                assert_eq!(queue, vec![]);
            }
        }
    }

    #[tokio::test]
    async fn test_next_state_1() {
        test_next_state_assert(
            &[
                &[test_data(1)],
                &[test_data(2), test_data(1)],               // Add 2
                &[test_data(3), test_data(2), test_data(1)], // Add 3
                &[test_data(4), test_data(3), test_data(2), test_data(1)], // Add 4
            ],
            &[test_data(2), test_data(3), test_data(4)],
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_2() {
        test_next_state_assert(
            &[
                &[test_data(4), test_data(2), test_data(1)],
                &[test_data(4), test_data(3), test_data(2)], // Add 3
            ],
            &[test_data(3)],
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_3() {
        test_next_state_assert(
            &[
                &[test_data(4), test_data(3), test_data(2)],
                &[test_data(4), test_data(2), test_data(1)], // Remove 3
            ],
            &[],
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_4() {
        test_next_state_assert(&[&[], &[test_data(1)]], &[test_data(1)]).await; // Add 1
    }

    #[tokio::test]
    async fn test_next_state_5() {
        test_next_state_assert(&[&[test_data(1)], &[]], &[]).await; // Remove 1
    }

    #[tokio::test]
    async fn test_next_state_6() {
        test_next_state_assert(
            &[
                &[test_data(4), test_data(3), test_data(2)],
                &[test_data(4), test_data(2), test_data(1)], // Remove 3
                &[test_data(4), test_data(3), test_data(2)], // Re-add 3
                &[test_data(4), test_data(2), test_data(1)], // Remove 3
                &[test_data(4), test_data(3), test_data(2)], // Re-add 3
            ],
            &[],
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_7() {
        test_next_state_assert(
            &[
                &[test_data(6), test_data(4), test_data(3)],
                &[test_data(6), test_data(5), test_data(4)], // Add 5
                &[test_data(6), test_data(4), test_data(3)], // Remove 5
                &[test_data(6), test_data(5), test_data(4)], // Re-add 5
                &[test_data(6), test_data(4), test_data(3)], // Remove 5
                &[test_data(7), test_data(6), test_data(4)], // Add 7
                &[test_data(7), test_data(6), test_data(5)], // Re-add 5
            ],
            &[test_data(5), test_data(7)],
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_8() {
        test_next_state_assert(
            &[
                &[test_data(7), test_data(6), test_data(5)],
                &[test_data(3), test_data(2), test_data(1)], // Seeing past result
                &[test_data(7), test_data(6), test_data(5)], // Problem was fixed
            ],
            &[],
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_9() {
        test_next_state_assert_queue(
            &[
                &[test_data(1)],
                &[test_data(2), test_data(1)], // Add 2
            ],
            &[test_data(2)],
            Some(&[test_data(1), test_data(2)]), // Queue has 2
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_10() {
        test_next_state_assert_queue(
            &[
                &[test_data(3), test_data(2), test_data(1)],
                &[test_data(6), test_data(5), test_data(4)],
            ],
            &[test_data(4), test_data(5), test_data(6)],
            Some(&[test_data(4), test_data(5), test_data(6)]), // Queue remove older.
        )
        .await;
    }

    #[tokio::test]
    async fn test_next_state_11() {
        test_next_state_assert(
            &[
                &[test_data(3)],
                &[test_data(4), test_data(3)],
                &[test_data(6), test_data(4), test_data(3)],
                &[test_data(7), test_data(6), test_data(4), test_data(3)],
                &[test_data(7), test_data(5), test_data(4), test_data(1)],
            ],
            &[test_data(4), test_data(6), test_data(7), test_data(5)],
        )
        .await;
    }
}
