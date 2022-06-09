#![forbid(non_ascii_idents, unsafe_code)]

mod conf;
mod control;
mod steps;
mod world;

use cucumber::{World as _, WorldInit as _};

pub use self::world::World;

#[tokio::main]
async fn main() {
    let concurrent = supports_multiple_webdriver_clients()
        .await
        .then(|| 4)
        .unwrap_or(1);

    World::cucumber()
        .repeat_failed()
        .fail_on_skipped()
        .max_concurrent_scenarios(concurrent)
        .run_and_exit(conf::FEATURES_PATH.as_str())
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
