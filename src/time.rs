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
