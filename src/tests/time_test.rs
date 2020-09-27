use crate::time::*;
use tokio::runtime::Runtime;
use chrono::Duration;
use chrono::Utc;

#[test]
fn time_test() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        //Not panic normally.
        wait_until(Utc::now() + Duration::seconds(1)).await;

        //Not panic though `when` is older than now.
        wait_until(Utc::now() - Duration::seconds(10)).await;
    });
}