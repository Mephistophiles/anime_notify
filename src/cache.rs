use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate};
use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Anime {
    do_not_disturb_until: String,
}

type URL = String;

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    cache: HashMap<URL, Anime>,
}

impl Cache {
    pub fn load() -> Result<Self> {
        let mut cache_dir =
            dirs::cache_dir().with_context(|| "Cache directory does not found.".to_string())?;
        cache_dir.push("anime_notify");
        fs::create_dir_all(&cache_dir)?;
        cache_dir.push("cache.yaml");

        let cache = fs::read_to_string(&cache_dir)?;
        let cache: Cache = serde_yaml::from_str(&cache)?;

        Ok(cache)
    }

    pub fn is_released(&self, name: &str) -> bool {
        return _is_released(self, name).unwrap_or_default();

        fn _is_released(cache: &Cache, url: &str) -> Option<bool> {
            let today = chrono::offset::Local::today().naive_local();
            cache.cache.get(url).map(|item| {
                let parse_from_str = chrono::NaiveDate::parse_from_str;

                let date: NaiveDate =
                    parse_from_str(&item.do_not_disturb_until, "%Y-%m-%d").unwrap();

                if date - today >= Duration::days(7) {
                    return true;
                }

                false
            })
        }
    }

    pub fn on_release(&mut self, name: &str) {
        let key = self
            .cache
            .entry(name.to_string())
            .or_insert_with(Default::default);

        key.do_not_disturb_until = (chrono::offset::Local::today() + Duration::days(7))
            .format("%Y-%m-%d")
            .to_string();
    }

    pub fn store(&mut self) -> Result<()> {
        let mut cache_dir =
            dirs::cache_dir().with_context(|| "Cache directory does not found.".to_string())?;
        cache_dir.push("anime_notify");
        fs::create_dir_all(&cache_dir)?;
        cache_dir.push("cache.yaml");

        let cache = serde_yaml::to_string(&self)?;

        fs::write(&cache_dir, cache)?;

        Ok(())
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        self.store().unwrap()
    }
}
