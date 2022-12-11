use dotenv::dotenv;
use std::{env, str::FromStr};

pub fn load_conf() -> Config {
    dotenv().ok();

    get_conf(
        env::var("TOKEN").ok(),
        env::var("CHID").ok(),
        env::var("KEYWORD").ok(),
        env::var("CRON").ok(),
        env::var("DRYRUN").ok(),
        env::var("BOT_WATCHING_TARGET").ok(),
    )
}

pub fn get_conf(
    token: Option<String>,
    chid: Option<String>,
    keyword: Option<String>,
    cron: Option<String>,
    dryrun: Option<String>,
    bot_watching_target: Option<String>,
) -> Config {
    Config {
        token: token.unwrap(),
        keyword: keyword.unwrap(),
        chid: chid
            .unwrap()
            .split(",")
            .map(|id| id.parse::<u64>().unwrap())
            .collect(),
        dryrun: bool::from_str(&dryrun.unwrap()).unwrap(),
        cron: cron.unwrap(),
        bot_watching_target: bot_watching_target.unwrap(),
    }
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub token: String,
    pub keyword: String,
    pub chid: Vec<u64>,
    pub dryrun: bool,
    pub cron: String,
    pub bot_watching_target: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config() {
        assert_eq!(
            get_conf(
                Some("token".to_string()),
                Some("1234".to_string()),
                Some("keyword".to_string()),
                Some("cron".to_string()),
                Some("true".to_string()),
                Some("bot_watching_target".to_string()),
            ),
            Config {
                token: "token".to_string(),
                chid: vec![1234],
                keyword: "keyword".to_string(),
                cron: "cron".to_string(),
                dryrun: true,
                bot_watching_target: "bot_watching_target".to_string(),
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
            Some("bot_watching_target".to_string()),
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
            Some("bot_watching_target".to_string()),
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
            Some("bot_watching_target".to_string()),
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
            Some("bot_watching_target".to_string()),
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
            Some("bot_watching_target".to_string()),
        );
    }

    #[test]
    #[should_panic]
    fn bot_watching_target_null() {
        get_conf(
            Some("token".to_string()),
            Some("chid".to_string()),
            Some("keyword".to_string()),
            Some("cron".to_string()),
            Some("true".to_string()),
            None,
        );
    }
}
