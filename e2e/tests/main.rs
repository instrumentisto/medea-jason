#![forbid(non_ascii_idents, unsafe_code)]

mod conf;
mod control;
mod steps;
mod world;

pub use self::world::World;

#[tokio::main]
async fn main() {
    let concurrent =
        if supports_multiple_webdriver_clients().await { 4 } else { 1 };

    <World as cucumber::World>::cucumber()
        .with_writer(cucumber::writer::Libtest::or_basic())
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
    World::try_new().await.and(World::try_new().await).is_ok()
}
