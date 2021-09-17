#![forbid(non_ascii_idents, unsafe_code)]

mod conf;
mod control;
mod steps;
mod world;

use cucumber_rust::WorldInit as _;
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

    let filter = conf.scenario.as_ref().map(|s| Regex::new(s).unwrap());
    World::cucumber()
        .fail_on_skipped()
        .max_concurrent_scenarios(Some(10))
        .filter_run_and_exit(conf::FEATURES_PATH.as_str(), move |_, _, s| {
            filter
                .as_ref()
                .map_or(true, |filter| filter.find(&s.name).is_some())
        })
        .await;
}
