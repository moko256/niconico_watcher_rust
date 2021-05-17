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
        chid: chid.unwrap().split(",").map(|id| id.parse::<u64>().unwrap()).collect(),
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
