use crate::config::*;
use dotenv::from_path;
use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;
use std::thread;
use std::time;

lazy_static! {
    static ref TEST_LOCK: Mutex<()> = Mutex::new(());
}

fn env_clear() {
    dotenv::vars();
    env::remove_var("TOKEN");
    env::remove_var("KEYWORD");
    env::remove_var("CHID");
    env::remove_var("DRYRUN");
    env::remove_var("CRON");
}

#[test]
fn chid_null() {
    let _l = TEST_LOCK.lock().unwrap();
    env_clear();
    from_path("./src/tests/config_test_cases/chid_null.env").ok();
    assert_eq!(
        get_conf(),
        Config {
            token: "".to_string(),
            keyword: "keyword_abc".to_string(),
            chid: "".to_string(),
            dryrun: true,
            cron: "0 0 0 0 0 0".to_string(),
        }
    );
}
#[test]
fn dryrun_false() {
    let _l = TEST_LOCK.lock().unwrap();
    env_clear();
    from_path("./src/tests/config_test_cases/dryrun_false.env").ok();
    assert_eq!(
        get_conf(),
        Config {
            token: "token_aaa".to_string(),
            keyword: "keyword_abc".to_string(),
            chid: "chid_123".to_string(),
            dryrun: false,
            cron: "0 0 0 0 0 0".to_string(),
        }
    );
}
#[test]
fn dryrun_null() {
    let _l = TEST_LOCK.lock().unwrap();
    env_clear();
    from_path("./src/tests/config_test_cases/dryrun_null.env").ok();
    assert_eq!(
        get_conf(),
        Config {
            token: "token_aaa".to_string(),
            keyword: "keyword_abc".to_string(),
            chid: "chid_123".to_string(),
            dryrun: true,
            cron: "0 0 0 0 0 0".to_string(),
        }
    );
}
#[test]
fn dryrun_true() {
    let _l = TEST_LOCK.lock().unwrap();
    env_clear();
    from_path("./src/tests/config_test_cases/dryrun_true.env").ok();
    assert_eq!(
        get_conf(),
        Config {
            token: "token_aaa".to_string(),
            keyword: "keyword_abc".to_string(),
            chid: "chid_123".to_string(),
            dryrun: true,
            cron: "0 0 0 0 0 0".to_string(),
        }
    );
}
#[test]
#[should_panic]
fn keyword_null() {
    thread::sleep(time::Duration::from_millis(1)); // To deal with panic, this test should do last.
    let _l = TEST_LOCK.lock().unwrap();
    env_clear();
    from_path("./src/tests/config_test_cases/keyword_null.env").ok();
    get_conf();
}
#[test]
fn token_null() {
    let _l = TEST_LOCK.lock().unwrap();
    env_clear();
    from_path("./src/tests/config_test_cases/token_null.env").ok();
    assert_eq!(
        get_conf(),
        Config {
            token: "".to_string(),
            keyword: "keyword_abc".to_string(),
            chid: "".to_string(),
            dryrun: true,
            cron: "0 0 0 0 0 0".to_string(),
        }
    );
}
