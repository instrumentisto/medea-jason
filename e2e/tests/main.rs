#![forbid(non_ascii_idents, unsafe_code)]

mod conf;
mod control;
mod steps;
mod world;

use std::sync::atomic::Ordering;

use atomic::Atomic;
use cucumber_rust::{World as _, WorldInit as _};
use once_cell::sync::Lazy;
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
        .then(|| 10)
        .unwrap_or(1);

    let filter = conf.scenario.as_ref().map(|s| Regex::new(s).unwrap());
    World::cucumber()
        .fail_on_skipped()
        .max_concurrent_scenarios(concurrent)
        .filter_run_and_exit(conf::FEATURES_PATH.as_str(), move |_, _, s| {
            filter
                .as_ref()
                .map_or(true, |filter| filter.find(&s.name).is_some())
        })
        .await;
}

/// Indicates whether `WebDriver` implementation supports multiple simultaneous
/// connections.
///
/// This is done, because `geckodriver` [doesn't support this feature][1]
///
/// [1]: https://github.com/mozilla/geckodriver/issues/1523
async fn supports_multiple_webdriver_clients() -> bool {
    static VALUE: Lazy<Atomic<Option<bool>>> = Lazy::new(|| Atomic::new(None));

    if let Some(v) = VALUE.load(Ordering::SeqCst) {
        v
    } else {
        let v = World::new().await.and(World::new().await).is_ok();
        VALUE.store(Some(v), Ordering::SeqCst);
        v
    }
}
