#![forbid(non_ascii_idents, unsafe_code)]

mod conf;
mod control;
mod steps;
mod world;

use cucumber_rust::WorldInit as _;

use self::world::World;

#[tokio::main]
async fn main() {
    World::cucumber()
        .fail_on_skipped()
        .max_concurrent_scenarios(10)
        .run_and_exit(conf::FEATURES_PATH.as_str())
        .await;
}
