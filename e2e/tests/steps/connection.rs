use cucumber::{then, when};

use crate::steps::parse_media_kind;
use crate::World;

#[then(regex = r"^(\S+) receives connection with (\S+)$")]
async fn then_member_receives_connection(
    world: &mut World,
    id: String,
    responder_id: String,
) {
    let member = world.get_member(&id).unwrap();
    member
        .connections()
        .wait_for_connection(responder_id.clone())
        .await
        .unwrap();
}

#[then(regex = r"^(\S+) doesn't receive connection with (\S+)$")]
async fn then_member_doesnt_receive_connection(
    world: &mut World,
    id: String,
    responder_id: String,
) {
    let member = world.get_member(&id).unwrap();
    assert!(member
        .connections()
        .get(responder_id)
        .await
        .unwrap()
        .is_none());
}

#[then(regex = r"^(\S+)'s connection with (\S+) closes$")]
async fn then_connection_closes(
    world: &mut World,
    id: String,
    partner_id: String,
) {
    let member = world.get_member(&id).unwrap();
    let connection =
        member.connections().get(partner_id).await.unwrap().unwrap();
    connection.wait_for_close().await.unwrap();
}

#[when(regex = r"^(\S+) (enables|disables) (audio|video) receive from (\S+)")]
async fn when_connection_changes_remote_media_state(
    world: &mut World,
    id: String,
    action: String,
    kind: String,
    partner_id: String,
) {
    let kind = parse_media_kind(&kind).unwrap();

    let member = world.get_member(&id).unwrap();
    let connection =
        member.connections().get(partner_id).await.unwrap().unwrap();

    if action.contains("enables") {
        connection.enable_remote_media(kind).await.unwrap();
    } else {
        connection.disable_remote_media(kind).await.unwrap();
    }
}
