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

mod audio_processing {
    use medea_jason::{api, media::MediaManager};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;

    use crate::{is_chromium, is_firefox, jsval_cast};

    async fn audio_track_for_caps(
        media_manager: Option<MediaManager>,
        caps: api::AudioTrackConstraints,
    ) -> JsValue {
        let media_manager = media_manager.unwrap_or_default();
        let constraints = {
            let mut constraints = api::MediaStreamSettings::new();
            constraints.device_audio(caps);
            constraints
        };
        js_sys::Array::from(
            &JsFuture::from(
                api::MediaManagerHandle::from(media_manager.new_handle())
                    .init_local_tracks(&constraints),
            )
            .await
            .unwrap(),
        )
        .at(0)
    }

    #[wasm_bindgen_test]
    async fn check_chromium_apply_constraints() {
        // Check that `MediaStreamTrack.applyConstraints()` cannot update audio
        // processing in Chrome. So, once this test fails it will mean that it's
        // fixed in Chrome.

        // TODO: Add tests for changing audio processing in runtime when any of
        //       this happens:
        //       - we will run tests in Safari
        //       - Firefox fixes audio processing with fake media
        //       - Chrome implements `MediaStreamTrack.applyConstraints()` for
        //         audio processing

        if !is_chromium() {
            return;
        }

        let track = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(None, api::AudioTrackConstraints::new()).await,
            "LocalMediaTrack",
        )
        .unwrap();
        assert_eq!(
            JsFuture::from(track.is_noise_suppression_enabled()).await.unwrap(),
            JsValue::TRUE
        );

        let exception = jsval_cast::<api::err::InternalException>(
            JsFuture::from(track.set_noise_suppression_enabled(false))
                .await
                .unwrap_err(),
            "InternalException",
        )
        .unwrap();

        assert_eq!(
            exception.message(),
            "Failed to access audio processing of a track"
        );
        assert_eq!(exception.cause().unwrap().message(), "not supported");
        assert_eq!(
            JsFuture::from(track.is_noise_suppression_enabled()).await.unwrap(),
            JsValue::TRUE
        );
    }

    #[wasm_bindgen_test]
    async fn ap_with_default_caps() {
        if is_firefox() {
            // We usually run Firefox with fake audio/video
            // (`media.navigator.streams.fake`) and this makes Firefox to
            // disable audio processing by default for whatever reason.
            return;
        }

        let track = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(None, api::AudioTrackConstraints::new()).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert!(track.is_audio_processing_available());
        assert_eq!(
            JsFuture::from(track.is_noise_suppression_enabled()).await.unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track.is_auto_gain_control_enabled()).await.unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track.is_echo_cancellation_enabled()).await.unwrap(),
            JsValue::TRUE
        );
    }

    #[wasm_bindgen_test]
    async fn ap_disabled_via_ideal_caps() {
        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.ideal_auto_gain_control(false);
        audio_caps.ideal_noise_suppression(false);
        audio_caps.ideal_echo_cancellation(false);

        let track = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(None, audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert!(track.is_audio_processing_available());
        assert_eq!(
            JsFuture::from(track.is_noise_suppression_enabled()).await.unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track.is_auto_gain_control_enabled()).await.unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track.is_echo_cancellation_enabled()).await.unwrap(),
            JsValue::FALSE
        );
    }

    #[wasm_bindgen_test]
    async fn ap_enabled_via_ideal_caps() {
        if is_firefox() {
            // We usually run Firefox with fake audio/video
            // (`media.navigator.streams.fake`) and this makes Firefox to
            // disable audio processing by default for whatever reason.
            return;
        }

        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.ideal_auto_gain_control(true);
        audio_caps.ideal_noise_suppression(true);
        audio_caps.ideal_echo_cancellation(true);

        let track = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(None, audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert!(track.is_audio_processing_available());
        assert_eq!(
            JsFuture::from(track.is_noise_suppression_enabled()).await.unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track.is_auto_gain_control_enabled()).await.unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track.is_echo_cancellation_enabled()).await.unwrap(),
            JsValue::TRUE
        );
    }

    #[wasm_bindgen_test]
    async fn ap_disabled_via_exact_caps() {
        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.exact_auto_gain_control(false);
        audio_caps.exact_noise_suppression(false);
        audio_caps.exact_echo_cancellation(false);

        let track = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(None, audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert!(track.is_audio_processing_available());
        assert_eq!(
            JsFuture::from(track.is_noise_suppression_enabled()).await.unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track.is_auto_gain_control_enabled()).await.unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track.is_echo_cancellation_enabled()).await.unwrap(),
            JsValue::FALSE
        );
    }

    #[wasm_bindgen_test]
    async fn ap_enabled_via_exact_caps() {
        if is_firefox() {
            // We usually run Firefox with fake audio/video
            // (`media.navigator.streams.fake`) and this makes Firefox to
            // disable audio processing by default for whatever reason.
            return;
        }

        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.exact_auto_gain_control(true);
        audio_caps.exact_noise_suppression(true);
        audio_caps.exact_echo_cancellation(true);

        let track = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(None, audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert!(track.is_audio_processing_available());
        assert_eq!(
            JsFuture::from(track.is_noise_suppression_enabled()).await.unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track.is_auto_gain_control_enabled()).await.unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track.is_echo_cancellation_enabled()).await.unwrap(),
            JsValue::TRUE
        );
    }

    #[wasm_bindgen_test]
    async fn new_track_when_different_ap_constraints() {
        if is_firefox() {
            // We usually run Firefox with fake audio/video
            // (`media.navigator.streams.fake`) and this makes Firefox to
            // disable audio processing by default for whatever reason.
            return;
        }

        let media_manager = MediaManager::default();
        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.exact_auto_gain_control(false);
        audio_caps.exact_noise_suppression(false);
        audio_caps.exact_echo_cancellation(false);

        let track1 = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(Some(media_manager.clone()), audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert_eq!(
            JsFuture::from(track1.is_noise_suppression_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track1.is_auto_gain_control_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track1.is_echo_cancellation_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );

        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.exact_auto_gain_control(true);
        audio_caps.exact_noise_suppression(true);
        audio_caps.exact_echo_cancellation(true);

        let track2 = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(Some(media_manager.clone()), audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert_eq!(
            JsFuture::from(track2.is_noise_suppression_enabled())
                .await
                .unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track2.is_auto_gain_control_enabled())
                .await
                .unwrap(),
            JsValue::TRUE
        );
        assert_eq!(
            JsFuture::from(track2.is_echo_cancellation_enabled())
                .await
                .unwrap(),
            JsValue::TRUE
        );

        assert_ne!(track1.get_track().id(), track2.get_track().id());
    }

    #[wasm_bindgen_test]
    async fn same_track_when_same_ap_constraints() {
        let media_manager = MediaManager::default();
        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.exact_auto_gain_control(false);
        audio_caps.exact_noise_suppression(false);
        audio_caps.exact_echo_cancellation(false);

        let track1 = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(Some(media_manager.clone()), audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert_eq!(
            JsFuture::from(track1.is_noise_suppression_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track1.is_auto_gain_control_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track1.is_echo_cancellation_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );

        let mut audio_caps = api::AudioTrackConstraints::new();
        audio_caps.exact_auto_gain_control(false);
        audio_caps.exact_noise_suppression(false);
        audio_caps.exact_echo_cancellation(false);

        let track2 = jsval_cast::<api::LocalMediaTrack>(
            audio_track_for_caps(Some(media_manager.clone()), audio_caps).await,
            "LocalMediaTrack",
        )
        .unwrap();

        assert_eq!(
            JsFuture::from(track2.is_noise_suppression_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track2.is_auto_gain_control_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );
        assert_eq!(
            JsFuture::from(track2.is_echo_cancellation_enabled())
                .await
                .unwrap(),
            JsValue::FALSE
        );

        assert_eq!(track1.get_track().id(), track2.get_track().id());
    }
}
