#![forbid(non_ascii_idents, unsafe_code)]

mod conf;
mod control;
mod steps;
mod world;

use cucumber_rust::{World as _, WorldInit as _};
use regex::Regex;
use structopt::StructOpt;

use self::world::World;

#[derive(StructOpt)]
struct Conf {
    #[structopt(long)]
    scenario: Option<String>,
}

#[tokio::main]
async fn main() {
    let conf: Conf = Conf::from_args();

    let concurrent = supports_multiple_webdriver_clients()
        .await
        .then(|| 5)
        .unwrap_or(1);

    let filter = conf.scenario.as_ref().map(|s| Regex::new(s).unwrap());
    World::cucumber()
        .fail_on_skipped()
        .max_concurrent_scenarios(concurrent)
        .filter_run_and_exit(conf::FEATURES_PATH.as_str(), move |_, _, s| {
            filter.as_ref().map_or(true, |f| f.find(&s.name).is_some())
        })
        .await;
}

/// Indicates whether `WebDriver` implementation supports multiple simultaneous
/// clients.
///
/// This is done, because geckodriver [doesn't support this feature][1], so
/// tests won't run concurrently on Firefox.
///
/// [1]: https://github.com/mozilla/geckodriver/issues/1523
async fn supports_multiple_webdriver_clients() -> bool {
    World::new().await.and(World::new().await).is_ok()
}
