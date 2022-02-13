use log::error;

use chrono::DateTime;
use chrono::Utc;

use tokio::time::sleep;

pub async fn wait_until(when: DateTime<Utc>) {
    let wait_until = when - Utc::now();
    match wait_until.to_std() {
        Ok(wait_until) => {
            sleep(wait_until).await;
        }
        Err(_err) => {
            error!(target: "nicow", "time: Schedule was gone. Skipped.");
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use tokio::runtime::Builder;

    use super::*;

    #[test]
    fn time_test() {
        let rt = Builder::new_current_thread().enable_time().build().unwrap();
        rt.block_on(async {
            //Not panic normally.
            wait_until(Utc::now() + Duration::seconds(1)).await;

            //Not panic though `when` is older than now.
            wait_until(Utc::now() - Duration::seconds(10)).await;
        });
    }
}
