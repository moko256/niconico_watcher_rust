use log::error;
use std::convert::TryFrom;

use chrono::DateTime;
use chrono::Utc;

use tokio::time::sleep;
use tokio::time::Duration;

pub async fn wait_until(when: DateTime<Utc>) {
    let until = when - Utc::now();
    let wait_s = until.num_seconds();
    let wait_s = TryFrom::try_from(wait_s);
    match wait_s {
        Ok(wait_s) => {
            let wait_s = Duration::from_secs(wait_s);
            sleep(wait_s).await;
        }
        Err(_err) => {
            error!(target: "nicow", "time: Schedule was gone. Skipped.");
        }
    }
}
