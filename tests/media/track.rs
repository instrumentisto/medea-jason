#![cfg(target_arch = "wasm32")]

use std::rc::{Rc, Weak};

use futures::{
    StreamExt as _,
    channel::{mpsc, oneshot},
};
use medea_jason::{
    api::MediaDirection,
    media::{
        DeviceVideoTrackConstraints, MediaManager, MediaStreamSettings,
        track::remote,
    },
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen_test::*;

use crate::{get_audio_track, timeout};

/// Assert that track is stopped when all strong refs are dropped.
#[wasm_bindgen_test]
async fn track_autostop() {
    let media_manager = MediaManager::default();
    let mut caps = MediaStreamSettings::new();
    caps.device_video(DeviceVideoTrackConstraints::new());

    let mut tracks = media_manager.get_tracks(caps).await.unwrap();

    assert_eq!(1, tracks.len());
    let (strong_track, strong_track_is_new) = tracks.pop().unwrap();
    assert!(strong_track_is_new);
    let sys_track = Clone::clone(strong_track.as_ref().as_ref().as_ref());
    let weak_track = Rc::downgrade(&strong_track);

    assert!(sys_track.ready_state() == web_sys::MediaStreamTrackState::Live);
    drop(strong_track);
    assert!(sys_track.ready_state() == web_sys::MediaStreamTrackState::Ended);
    assert_eq!(Weak::strong_count(&weak_track), 0);
}

#[wasm_bindgen_test]
async fn on_track_unmuted_works() {
    let api_track = get_audio_track().await;
    let core_track: remote::Track = api_track.clone().into();

    let core_track_clone = core_track.clone();
    let (test_tx, test_rx) = oneshot::channel();
    api_track.on_unmuted(
        Closure::once_into_js(move || {
            assert!(!core_track_clone.muted());
            test_tx.send(()).unwrap();
        })
        .into(),
    );

    let (dont_fire_tx, mut dont_fire_rx) = mpsc::unbounded();
    let dont_fire = || {
        let tx = dont_fire_tx.clone();
        Closure::once_into_js(move || {
            tx.unbounded_send(()).unwrap();
        })
        .into()
    };
    api_track.on_media_direction_changed(dont_fire());
    api_track.on_stopped(dont_fire());

    core_track.set_muted(true);
    assert_eq!(api_track.media_direction(), MediaDirection::SendRecv);
    assert!(api_track.muted());
    core_track.set_muted(false);
    assert_eq!(api_track.media_direction(), MediaDirection::SendRecv);
    assert!(!api_track.muted());

    timeout(100, test_rx).await.unwrap().unwrap();
    timeout(100, dont_fire_rx.next()).await.unwrap_err();
}

#[wasm_bindgen_test]
async fn on_track_muted_works() {
    let api_track = get_audio_track().await;
    let core_track: remote::Track = api_track.clone().into();

    let core_track_clone = core_track.clone();
    let (test_tx, test_rx) = oneshot::channel();
    api_track.on_muted(
        Closure::once_into_js(move || {
            assert!(core_track_clone.muted());
            test_tx.send(()).unwrap();
        })
        .into(),
    );

    let (dont_fire_tx, mut dont_fire_rx) = mpsc::unbounded();
    let dont_fire = || {
        let tx = dont_fire_tx.clone();
        Closure::once_into_js(move || {
            tx.unbounded_send(()).unwrap();
        })
        .into()
    };
    api_track.on_unmuted(dont_fire());
    api_track.on_media_direction_changed(dont_fire());
    api_track.on_stopped(dont_fire());

    assert_eq!(api_track.media_direction(), MediaDirection::SendRecv);
    assert!(!api_track.muted());
    core_track.set_muted(true);
    assert_eq!(api_track.media_direction(), MediaDirection::SendRecv);

    timeout(100, test_rx).await.unwrap().unwrap();
    timeout(100, dont_fire_rx.next()).await.unwrap_err();
}
