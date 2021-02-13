use anyhow::Result;
use bot::Bot;
use cache::Cache;
use chrono::Local;
use config::Config;
use indoc::indoc;
use tokio::time::{self, Duration};

mod bot;
mod cache;
mod config;
mod site;

async fn check_for_new_episodes(config: &Config, bot: &mut Bot) -> Duration {
    let mut cache = Cache::load().unwrap_or_default();
    let next_day = Local::today().naive_local().succ().and_hms(0, 0, 0);
    let now = Local::now().naive_local();

    let mut sleep_until = (next_day - now).to_std().unwrap();

    for site in config.sites() {
        let new_episodes = match site.check_for_new_episodes().await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        for ep in new_episodes {
            let name = ep.name();

            if cache.is_released(name) {
                continue;
            }

            if ep.is_new() {
                cache.on_release(name);

                bot.send_message(
                    config.telegram_user_id(),
                    &format!(
                        indoc! {"
                    New anime episode:
                    {}
                    {}
                    "},
                        name,
                        ep.url()
                    ),
                )
                .await
                .unwrap();
            } else {
                sleep_until = Duration::from_secs(5 * 60);
            }
        }
    }

    sleep_until
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let mut bot = config.new_bot();

    loop {
        let sleep_until = check_for_new_episodes(&config, &mut bot).await;
        println!("Wait for {:?}", sleep_until);
        time::sleep(sleep_until).await;
    }
}
