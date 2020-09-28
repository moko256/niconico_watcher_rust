use dotenv::dotenv;
use log::warn;

pub fn load_conf() -> Config {
    dotenv().ok();

    get_conf()
}

pub fn get_conf() -> Config {
    let token = dotenv::var("TOKEN");
    let chid = dotenv::var("CHID");
    let keyword = dotenv::var("KEYWORD").expect(".env: KEYWORD is null value.");
    let cron = dotenv::var("CRON").expect(".env: CRON is null value.");
    let dryrun = dotenv::var("DRYRUN")
        .map(|s| {
            let p = s.parse::<bool>();
            if let Ok(v) = p {
                if v {
                    warn!(target: "nicow", ".env: Running dry-run mode.");    
                }
            } else {
                warn!(target: "nicow", ".env: Cannot parse DRYRUN value: {}. Fall-back to dry-run mode.", s);
            }
            p.unwrap_or(true)
        })
        .unwrap_or(true);
    if let (Ok(token), Ok(chid)) = (token, chid) {
        Config {
            token,
            keyword,
            chid,
            dryrun,
            cron,
        }
    } else {
        warn!(target: "nicow", ".env: TOKEN or CHID is null value. Fall-back to dry-run mode.");
        Config {
            token: "".to_string(),
            keyword,
            chid: "".to_string(),
            dryrun: true,
            cron,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub token: String,
    pub keyword: String,
    pub chid: String,
    pub dryrun: bool,
    pub cron: String,
}
