use std::time::Duration;

use cucumber::{given, then, when};
use medea_e2e::object::{
    remote_track::MediaDirection, AwaitCompletion, MediaSourceKind,
};

use crate::World;

use super::{parse_media_kind, parse_media_kinds};

#[given(regex = r"^(\S+)'s `getUserMedia\(\)` request has added latency$")]
async fn given_gum_delay(world: &mut World, id: String) {
    let member = world.get_member(&id).unwrap();
    member.add_gum_latency(Duration::from_millis(500)).await;
}

#[when(regex = r"^(\S+) frees all local tracks$")]
async fn when_member_frees_all_local_tracks(world: &mut World, id: String) {
    let member = world.get_member(&id).unwrap();
    member.room().forget_local_tracks().await;
}

#[then(regex = "^(\\S+)'s (audio|(?:device|display) video) local track is \
                 (not )?muted$")]
async fn then_local_track_mute_state(
    world: &mut World,
    id: String,
    kind: String,
    not_muted: String,
) {
    let member = world.get_member(&id).unwrap();
    let (media_kind, source_kind) = parse_media_kinds(&kind).unwrap();
    let track = member
        .room()
        .local_tracks()
        .await
        .unwrap()
        .get_track(media_kind, source_kind)
        .await
        .unwrap();
    assert_eq!(not_muted.is_empty(), track.muted().await.unwrap());
}

#[then(regex = "^(\\S+)'s (audio|(?:device|display) video) local track is \
                 stopped$")]
async fn then_track_is_stopped(world: &mut World, id: String, kind: String) {
    let member = world.get_member(&id).unwrap();
    let (media_kind, source_kind) = parse_media_kinds(&kind).unwrap();
    let is_stopped = member
        .room()
        .local_tracks()
        .await
        .unwrap()
        .get_track(media_kind, source_kind)
        .await
        .unwrap()
        .free_and_check()
        .await
        .unwrap();
    assert!(is_stopped);
}

#[then(regex = "^(\\S+)'s (audio|video) from (\\S+) has \
             (SendRecv|SendOnly|RecvOnly|Inactive) direction$")]
async fn then_remote_media_direction_is(
    world: &mut World,
    id: String,
    kind: String,
    remote_id: String,
    direction: String,
) {
    let media_kind = kind.parse().unwrap();
    let media_direction = match direction.as_str() {
        "SendRecv" => MediaDirection::SendRecv,
        "SendOnly" => MediaDirection::SendOnly,
        "RecvOnly" => MediaDirection::RecvOnly,
        "Inactive" => MediaDirection::Inactive,
        _ => unreachable!(),
    };

    let member = world.get_member(&id).unwrap();
    let connection = member
        .connections()
        .wait_for_connection(remote_id)
        .await
        .unwrap();
    let tracks_store = connection.tracks_store().await.unwrap();
    let track = tracks_store
        .get_track(media_kind, MediaSourceKind::Device)
        .await
        .unwrap();
    track
        .wait_for_media_direction(media_direction)
        .await
        .unwrap();
}

#[when(regex = "^(\\S+) (enables|disables|mutes|unmutes) (audio|video)\
                 ( and awaits it (complete|error)s)?$")]
async fn when_enables_or_mutes(
    world: &mut World,
    id: String,
    action: String,
    audio_or_video: String,
    awaits: String,
) {
    let member = world.get_member(&id).unwrap();
    let maybe_await = if awaits.is_empty() {
        AwaitCompletion::Dont
    } else {
        AwaitCompletion::Do
    };

    let result = match action.as_str() {
        "enables" => {
            member
                .toggle_media(
                    parse_media_kind(&audio_or_video),
                    None,
                    true,
                    maybe_await,
                )
                .await
        }
        "disables" => {
            member
                .toggle_media(
                    parse_media_kind(&audio_or_video),
                    None,
                    false,
                    maybe_await,
                )
                .await
        }
        "mutes" => {
            member
                .toggle_mute(
                    parse_media_kind(&audio_or_video),
                    None,
                    true,
                    maybe_await,
                )
                .await
        }
        _ => {
            member
                .toggle_mute(
                    parse_media_kind(&audio_or_video),
                    None,
                    false,
                    maybe_await,
                )
                .await
        }
    };

    if maybe_await == AwaitCompletion::Do {
        if awaits.contains("error") {
            result.unwrap_err();
        } else {
            result.unwrap();
        }
    }
}

#[when(regex = "^(\\S+) switches device with latency$")]
async fn when_member_switches_device_with_latency(
    world: &mut World,
    id: String,
) {
    let member = world.get_member(&id).unwrap();

    member.add_gum_latency(Duration::from_secs(3)).await;
    member.switch_video_device().await.unwrap();
}

#[when(regex = "^(\\S+) (enables|disables) remote \
                 (audio|(?:device |display )?video)$")]
async fn when_member_enables_remote_track(
    world: &mut World,
    id: String,
    toggle: String,
    kind: String,
) {
    let member = world.get_member(&id).unwrap();
    let media_kind = kind.parse().unwrap();
    let source_kind = kind.parse().ok();

    if toggle == "enables" {
        member
            .room()
            .enable_remote_media(media_kind, source_kind)
            .await
            .unwrap();
    } else {
        member
            .room()
            .disable_remote_media(media_kind, source_kind)
            .await
            .unwrap();
    }
}
