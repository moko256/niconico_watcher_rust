use std::fs::read_to_string;

use log::info;
use serde::Deserialize;

pub fn load_conf() -> Config {
    let raw_config = read_to_string("bot_config.toml");
    match raw_config {
        Ok(raw_config) => {
            let config = get_conf(&raw_config);

            info!("Parsing `bot_config.toml` was successful!");

            config
        }
        Err(err) => {
            panic!("Failed to read `bot_config.toml`: {}", err);
        }
    }
}

pub fn get_conf(content: &str) -> Config {
    let config = toml::from_str::<Config>(content);
    match config {
        Ok(config) => config,
        Err(err) => {
            panic!("Failed to parse toml: {}", err);
        }
    }
}

#[derive(PartialEq, Eq, Debug, Deserialize)]
pub struct Config {
    pub keyword: String,
    pub dryrun: bool,
    pub cron: String,

    pub discord: Option<DiscordConfig>,
    pub misskey: Option<MisskeyConfig>,
}

#[derive(PartialEq, Eq, Debug, Deserialize)]
pub struct DiscordConfig {
    pub token: String,
    pub chid: Vec<u64>,
    pub bot_watching_target: String,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Clone)]
pub struct MisskeyConfig {
    pub server_domain: String,
    pub token: String,
    pub post_footer: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config_all() {
        assert_eq!(
            get_conf(
                r#"
                keyword = "keyword"
                dryrun = true
                cron = "cron"

                [discord]
                token = "token"
                chid = [12,34]
                bot_watching_target = "bot_watching_target"

                [misskey]
                server_domain = "example.com"
                token = "token"
                post_footer = "footer"
                "#,
            ),
            Config {
                keyword: "keyword".to_string(),
                dryrun: true,
                cron: "cron".to_string(),

                discord: Some(DiscordConfig {
                    token: "token".to_string(),
                    chid: vec![12, 34],
                    bot_watching_target: "bot_watching_target".to_string(),
                }),
                misskey: Some(MisskeyConfig {
                    server_domain: "example.com".to_string(),
                    token: "token".to_string(),
                    post_footer: "footer".to_string(),
                })
            }
        );
    }

    #[test]
    #[should_panic]
    fn invalid_nothing_all() {
        get_conf("");
    }
}
