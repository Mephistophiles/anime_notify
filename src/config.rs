use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

use crate::{bot::Bot, site::Site};

#[derive(Deserialize)]
pub struct Config {
    telegram_token: String,
    telegram_user_id: i64,
    sites: Vec<Site>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config_dir =
            dirs::config_dir().with_context(|| "Config directory does not found.".to_string())?;
        config_dir.push("anime_notify");
        fs::create_dir_all(&config_dir).with_context(|| {
            format!("Creating directories {} was failed.", config_dir.display())
        })?;
        config_dir.push("config.yaml");

        let config = fs::read_to_string(&config_dir)
            .with_context(|| format!("Read to string file {} was failed", config_dir.display()))?;
        let config: Config = serde_yaml::from_str(&config)?;

        Ok(config)
    }

    pub fn new_bot(&self) -> Bot {
        Bot::new(&self.telegram_token)
    }

    pub fn telegram_user_id(&self) -> i64 {
        self.telegram_user_id
    }

    pub fn sites(&self) -> impl Iterator<Item = &Site> {
        self.sites.iter()
    }
}
