use crate::time::*;
use chrono::Duration;
use chrono::Utc;
use tokio::runtime::Builder;

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
