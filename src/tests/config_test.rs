use crate::config::*;

#[test]
fn valid_config() {
    assert_eq!(
        get_conf(
            Some("token".to_string()),
            Some("chid".to_string()),
            Some("keyword".to_string()),
            Some("cron".to_string()),
            Some("true".to_string()),
        ),
        Config {
            token: "token".to_string(),
            chid: "chid".to_string(),
            keyword: "keyword".to_string(),
            cron: "cron".to_string(),
            dryrun: true,
        }
    );
}

#[test]
#[should_panic]
fn token_null() {
    get_conf(
        None,
        Some("chid".to_string()),
        Some("keyword".to_string()),
        Some("cron".to_string()),
        Some("true".to_string()),
    );
}

#[test]
#[should_panic]
fn chid_null() {
    get_conf(
        Some("token".to_string()),
        None,
        Some("keyword".to_string()),
        Some("cron".to_string()),
        Some("true".to_string()),
    );
}

#[test]
#[should_panic]
fn keyword_null() {
    get_conf(
        Some("token".to_string()),
        Some("chid".to_string()),
        None,
        Some("cron".to_string()),
        Some("true".to_string()),
    );
}

#[test]
#[should_panic]
fn cron_null() {
    get_conf(
        Some("token".to_string()),
        Some("chid".to_string()),
        Some("keyword".to_string()),
        None,
        Some("true".to_string()),
    );
}

#[test]
#[should_panic]
fn dryrun_null() {
    get_conf(
        Some("token".to_string()),
        Some("chid".to_string()),
        Some("keyword".to_string()),
        Some("cron".to_string()),
        None,
    );
}
