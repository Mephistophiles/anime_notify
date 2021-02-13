use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::{Datelike, Weekday};
use scraper::{Html, Selector};
use serde::{Deserialize, Deserializer};

#[derive(PartialEq, Eq, Hash)]
struct MyWeekday(Weekday);

#[derive(Deserialize)]
pub struct Site {
    site: String,
    release_days: HashMap<MyWeekday, Vec<Subscription>>,
}
#[derive(Deserialize, Clone)]
pub struct Subscription {
    #[serde(skip)]
    new: bool,
    url: String,
    name: String,
    new_episode_selector: String,
}

impl Subscription {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn is_new(&self) -> bool {
        self.new
    }
}

impl Site {
    pub async fn check_for_new_episodes(&self) -> Result<Vec<Subscription>> {
        let res = reqwest::get(&self.site)
            .await
            .with_context(|| format!("Failed to get site {:?}", self.site))?;
        let body = res
            .text()
            .await
            .with_context(|| "Getting the site's body was failing.".to_string())?;
        let dom = Html::parse_document(&body);

        let today = chrono::offset::Local::today().naive_local();

        let current_release_day = self
            .release_days
            .get(&MyWeekday(today.weekday()))
            .with_context(|| "No such animes today".to_string())?;

        let mut output = Vec::with_capacity(current_release_day.len());

        for anime in current_release_day {
            let selector = Selector::parse(&anime.new_episode_selector).unwrap();

            let mut subscription = anime.clone();
            subscription.new = dom.select(&selector).count() > 0;

            output.push(subscription);
        }

        Ok(output)
    }
}

impl<'de> Deserialize<'de> for MyWeekday {
    fn deserialize<D>(deserializer: D) -> std::result::Result<MyWeekday, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let weekday = match s.as_str() {
            "Monday" => Weekday::Mon,
            "Tuesday" => Weekday::Tue,
            "Wednesday" => Weekday::Wed,
            "Thursday" => Weekday::Thu,
            "Friday" => Weekday::Fri,
            "Saturday" => Weekday::Sat,
            "Sunday" => Weekday::Sun,
            _ => return Err(serde::de::Error::custom("Invalid date format")),
        };

        Ok(MyWeekday(weekday))
    }
}
