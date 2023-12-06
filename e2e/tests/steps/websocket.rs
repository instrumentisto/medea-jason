use std::time::Duration;

use cucumber::{then, when};
use tokio::time::sleep;

use crate::World;

#[when(regex = r"^(\S+) loses WS connection$")]
async fn ws_connection_loss(world: &mut World, id: String) {
    let member = world.get_member(&id).unwrap();
    member.ws_mock().enable_connection_loss(3999).await;
    sleep(Duration::from_millis(500)).await;
}

#[when(regex = r"^(\S+) restores WS connection$")]
async fn ws_connection_restore(world: &mut World, id: String) {
    let member = world.get_member(&id).unwrap();
    member.ws_mock().disable_connection_loss().await;
    member.room().start_ws_reconnect().await.unwrap();
}

#[then(regex = r"^(\S+)'s WS connection is lost$")]
async fn connection_is_lost(world: &mut World, id: String) {
    let member = world.get_member(&id).unwrap();
    member.room().wait_for_connection_loss().await.unwrap();
}
