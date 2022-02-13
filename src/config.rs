use dotenv::dotenv;
use std::str::FromStr;

pub fn load_conf() -> Config {
    dotenv().ok();

    get_conf(
        dotenv::var("TOKEN").ok(),
        dotenv::var("CHID").ok(),
        dotenv::var("KEYWORD").ok(),
        dotenv::var("CRON").ok(),
        dotenv::var("DRYRUN").ok(),
    )
}

pub fn get_conf(
    token: Option<String>,
    chid: Option<String>,
    keyword: Option<String>,
    cron: Option<String>,
    dryrun: Option<String>,
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
    }
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub token: String,
    pub keyword: String,
    pub chid: Vec<u64>,
    pub dryrun: bool,
    pub cron: String,
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
            ),
            Config {
                token: "token".to_string(),
                chid: vec![1234],
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
}
