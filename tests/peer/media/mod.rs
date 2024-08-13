#![cfg(target_arch = "wasm32")]

mod transitable_state;

use std::{mem, rc::Rc};

use futures::channel::mpsc;
use medea_client_api_proto::{ConnectionMode, TrackId, TrackPatchEvent};
use medea_jason::{
    media::{LocalTracksConstraints, MediaManager, RecvConstraints},
    peer::{
        media_exchange_state, LocalStreamUpdateCriteria, MediaConnections,
        MediaStateControllable, SimpleTracksRequest,
    },
    platform::RtcPeerConnection,
    utils::Updatable as _,
};
use wasm_bindgen_test::*;

use crate::{
    get_media_stream_settings, get_test_unrequired_tracks, local_constraints,
};

wasm_bindgen_test_configure!(run_in_browser);

async fn get_test_media_connections(
    enabled_audio: bool,
    enabled_video: bool,
) -> (MediaConnections, TrackId, TrackId) {
    let (tx, rx) = mpsc::unbounded();
    mem::forget(rx);
    let media_connections = MediaConnections::new(
        Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
        tx,
    );
    let (audio_track, video_track) = get_test_unrequired_tracks();
    let audio_track_id = audio_track.id;
    let video_track_id = video_track.id;
    media_connections
        .create_tracks(
            vec![audio_track, video_track],
            &get_media_stream_settings(enabled_audio, enabled_video).into(),
            &RecvConstraints::default(),
            ConnectionMode::Mesh,
        )
        .await
        .unwrap();
    let request = media_connections
        .get_tracks_request(LocalStreamUpdateCriteria::all())
        .unwrap();
    let caps = SimpleTracksRequest::try_from(request).unwrap();
    let manager = Rc::new(MediaManager::default());
    let tracks = manager.get_tracks(&caps).await.unwrap();

    media_connections
        .insert_local_tracks(
            &caps
                .parse_tracks(tracks.into_iter().map(|(t, _)| t).collect())
                .await
                .unwrap(),
        )
        .await
        .unwrap();

    media_connections
        .get_sender_state_by_id(audio_track_id)
        .unwrap()
        .media_state_transition_to(
            media_exchange_state::Stable::from(enabled_audio).into(),
        )
        .unwrap();
    media_connections
        .get_sender_state_by_id(video_track_id)
        .unwrap()
        .media_state_transition_to(
            media_exchange_state::Stable::from(enabled_video).into(),
        )
        .unwrap();

    (media_connections, audio_track_id, video_track_id)
}

#[wasm_bindgen_test]
async fn get_tracks_request1() {
    let (tx, rx) = mpsc::unbounded();
    mem::forget(rx);
    let media_connections = MediaConnections::new(
        Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
        tx,
    );
    let (audio_track, video_track) = get_test_unrequired_tracks();
    media_connections
        .create_tracks(
            vec![audio_track, video_track],
            &local_constraints(true, true),
            &RecvConstraints::default(),
            ConnectionMode::Mesh,
        )
        .await
        .unwrap();
    let request =
        media_connections.get_tracks_request(LocalStreamUpdateCriteria::all());
    assert!(request.is_some());
}

#[wasm_bindgen_test]
async fn get_tracks_request2() {
    let (tx, rx) = mpsc::unbounded();
    mem::forget(rx);
    let media_connections = MediaConnections::new(
        Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
        tx,
    );
    media_connections
        .create_tracks(
            Vec::new(),
            &LocalTracksConstraints::default(),
            &RecvConstraints::default(),
            ConnectionMode::Mesh,
        )
        .await
        .unwrap();
    let request =
        media_connections.get_tracks_request(LocalStreamUpdateCriteria::all());
    assert!(request.is_none());
}

#[wasm_bindgen_test]
async fn new_media_connections_with_disabled_audio_tracks() {
    let (media_connections, audio_track_id, video_track_id) =
        get_test_media_connections(false, true).await;

    let audio_track = media_connections
        .get_sender_state_by_id(audio_track_id)
        .unwrap();
    let video_track = media_connections
        .get_sender_state_by_id(video_track_id)
        .unwrap();

    assert!(!audio_track.enabled());
    assert!(video_track.enabled());
}

#[wasm_bindgen_test]
async fn new_media_connections_with_disabled_video_tracks() {
    let (media_connections, audio_track_id, video_track_id) =
        get_test_media_connections(true, false).await;

    let audio_track = media_connections
        .get_sender_state_by_id(audio_track_id)
        .unwrap();
    let video_track = media_connections
        .get_sender_state_by_id(video_track_id)
        .unwrap();

    assert!(audio_track.enabled());
    assert!(!video_track.enabled());
}

/// Tests for [`Sender::update`] function.
///
/// This tests checks that [`TrackPatch`] works as expected.
mod sender_patch {
    use medea_client_api_proto::{
        AudioSettings, ConnectionMode, MediaDirection, MediaType,
    };
    use medea_jason::{
        peer::{sender, MediaExchangeState},
        utils::{AsProtoState, SynchronizableState},
    };

    use super::*;

    async fn get_sender() -> (sender::Component, TrackId, MediaConnections) {
        let (tx, rx) = mpsc::unbounded();
        mem::forget(rx);
        let media_connections = MediaConnections::new(
            Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
            tx,
        );
        let sender = media_connections
            .create_sender(
                TrackId(0),
                MediaType::Audio(AudioSettings { required: false }),
                MediaDirection::SendRecv,
                false,
                None,
                vec!["bob".into()],
                &LocalTracksConstraints::default(),
                ConnectionMode::Mesh,
            )
            .await
            .unwrap();

        (sender, TrackId(0), media_connections)
    }

    #[wasm_bindgen_test]
    async fn wrong_track_id() {
        let (sender, track_id, _media_connections) = get_sender().await;
        sender.state().update(TrackPatchEvent {
            id: TrackId(track_id.0 + 100),
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        sender.state().when_updated().await;

        assert!(!sender.general_disabled());
    }

    #[wasm_bindgen_test]
    async fn disable() {
        let (sender, track_id, _media_connections) = get_sender().await;
        sender.state().update(TrackPatchEvent {
            id: track_id,
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        sender.state().when_updated().await;

        assert!(sender.general_disabled());
    }

    #[wasm_bindgen_test]
    async fn enabled_enabled() {
        let (sender, track_id, _media_connections) = get_sender().await;
        sender.state().update(TrackPatchEvent {
            id: track_id,
            receivers: None,
            media_direction: Some(MediaDirection::SendRecv),
            muted: None,
            encoding_parameters: None,
        });
        sender.state().when_updated().await;

        assert!(!sender.general_disabled());
    }

    #[wasm_bindgen_test]
    async fn disable_disabled() {
        let (sender, track_id, _media_connections) = get_sender().await;
        sender.state().update(TrackPatchEvent {
            id: track_id,
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        sender.state().when_updated().await;
        assert!(sender.general_disabled());

        sender.state().update(TrackPatchEvent {
            id: track_id,
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        sender.state().when_updated().await;

        assert!(sender.general_disabled());
    }

    #[wasm_bindgen_test]
    async fn empty_patch() {
        let (sender, track_id, _media_connections) = get_sender().await;
        sender.state().update(TrackPatchEvent {
            id: track_id,
            receivers: None,
            media_direction: None,
            muted: None,
            encoding_parameters: None,
        });
        sender.state().when_updated().await;

        assert!(!sender.general_disabled());
    }

    /// Checks that [`Sender`]'s mute and media exchange states can be changed
    /// by [`SenderState`] update.
    #[wasm_bindgen_test]
    async fn update_by_state() {
        let (sender, _, _media_connections) = get_sender().await;

        let mut proto_state = sender.state().as_proto();
        proto_state.media_direction = MediaDirection::RecvOnly;
        proto_state.muted = true;
        sender
            .state()
            .apply(proto_state, &LocalTracksConstraints::default());
        sender.state().when_updated().await;

        assert!(sender.general_disabled());
        assert_eq!(
            sender.state().media_exchange_state(),
            MediaExchangeState::Stable(media_exchange_state::Stable::Disabled)
        );
        assert!(sender.muted());
    }
}

mod receiver_patch {
    use medea_client_api_proto::{
        AudioSettings, MediaDirection, MediaType, MemberId,
    };
    use medea_jason::{
        media::RecvConstraints,
        peer::{receiver, MediaExchangeState, PeerEvent},
        utils::{AsProtoState, SynchronizableState},
    };

    use super::*;

    const TRACK_ID: TrackId = TrackId(0);
    const MID: &str = "mid";
    const SENDER_ID: &str = "sender";

    async fn get_receiver(
    ) -> (receiver::Component, mpsc::UnboundedReceiver<PeerEvent>) {
        let (tx, rx) = mpsc::unbounded();
        let media_connections = MediaConnections::new(
            Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
            tx,
        );
        let recv = media_connections
            .create_receiver(
                TRACK_ID,
                MediaType::Audio(AudioSettings { required: true }).into(),
                MediaDirection::SendRecv,
                false,
                Some(MID.to_string()),
                MemberId(SENDER_ID.to_string()),
                &RecvConstraints::default(),
                ConnectionMode::Mesh,
            )
            .await;

        (recv, rx)
    }

    #[wasm_bindgen_test]
    async fn wrong_track_id() {
        let (receiver, _tx) = get_receiver().await;
        receiver.state().update(&TrackPatchEvent {
            id: TrackId(TRACK_ID.0 + 100),
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        receiver.state().when_updated().await;

        assert!(receiver.enabled_general());
    }

    #[wasm_bindgen_test]
    async fn disable() {
        let (receiver, _tx) = get_receiver().await;
        receiver.state().update(&TrackPatchEvent {
            id: TRACK_ID,
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        receiver.state().when_updated().await;

        assert!(!receiver.enabled_general());
    }

    #[wasm_bindgen_test]
    async fn enabled_enabled() {
        let (receiver, _tx) = get_receiver().await;
        receiver.state().update(&TrackPatchEvent {
            id: TRACK_ID,
            receivers: None,
            media_direction: Some(MediaDirection::SendRecv),
            muted: None,
            encoding_parameters: None,
        });
        receiver.state().when_updated().await;

        assert!(receiver.enabled_general());
    }

    #[wasm_bindgen_test]
    async fn disable_disabled() {
        let (receiver, _tx) = get_receiver().await;
        receiver.state().update(&TrackPatchEvent {
            id: TRACK_ID,
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        receiver.state().when_updated().await;
        assert!(!receiver.enabled_general());

        receiver.state().update(&TrackPatchEvent {
            id: TRACK_ID,
            receivers: None,
            media_direction: Some(MediaDirection::RecvOnly),
            muted: None,
            encoding_parameters: None,
        });
        receiver.state().when_updated().await;

        assert!(!receiver.enabled_general());
    }

    #[wasm_bindgen_test]
    async fn empty_patch() {
        let (receiver, _tx) = get_receiver().await;
        receiver.state().update(&TrackPatchEvent {
            id: TRACK_ID,
            receivers: None,
            media_direction: None,
            muted: None,
            encoding_parameters: None,
        });
        receiver.state().when_updated().await;

        assert!(receiver.enabled_general());
    }

    /// Checks that [`Receiver`]'s media exchange state can be changed by
    /// [`ReceiverState`] update.
    #[wasm_bindgen_test]
    async fn update_by_state() {
        let (receiver, _tx) = get_receiver().await;

        let mut proto_state = receiver.state().as_proto();
        proto_state.media_direction = MediaDirection::SendOnly;

        receiver
            .state()
            .apply(proto_state, &LocalTracksConstraints::default());

        receiver.state().when_updated().await;
        assert!(!receiver.state().enabled_general());
        assert_eq!(
            receiver.state().media_exchange_state(),
            MediaExchangeState::Stable(media_exchange_state::Stable::Disabled)
        );
    }
}

mod codec_probing {
    use medea_client_api_proto::{Codec, ScalabilityMode, SvcSettings};
    use medea_jason::{
        media::MediaKind, peer::media::probe_video_codecs,
        platform::CodecCapability,
    };

    use super::*;

    fn target_codecs_mime_types(codecs: &[CodecCapability]) -> Vec<String> {
        let mut mime_types: Vec<_> =
            codecs.iter().map(|c| c.mime_type()).collect();
        mime_types.sort();
        mime_types.dedup();
        mime_types
    }

    #[wasm_bindgen_test]
    async fn probes_only_one_codec() {
        let (target_codecs, svc) = probe_video_codecs(&vec![SvcSettings {
            codec: Codec::VP8,
            scalability_mode: ScalabilityMode::L1T1,
        }])
        .await;

        assert_eq!(svc, Some(ScalabilityMode::L1T1));
        assert_eq!(
            target_codecs_mime_types(&target_codecs),
            vec!["video/VP8", "video/red", "video/rtx", "video/ulpfec"]
        );
    }

    #[wasm_bindgen_test]
    async fn probes_first_codec_when_many() {
        {
            let (target_codecs, svc) = probe_video_codecs(&vec![
                SvcSettings {
                    codec: Codec::VP9,
                    scalability_mode: ScalabilityMode::L1T2,
                },
                SvcSettings {
                    codec: Codec::VP8,
                    scalability_mode: ScalabilityMode::L1T1,
                },
            ])
            .await;

            assert_eq!(svc, Some(ScalabilityMode::L1T2));
            assert_eq!(
                target_codecs_mime_types(&target_codecs),
                vec!["video/VP9", "video/red", "video/rtx", "video/ulpfec"]
            );
        }

        {
            let (target_codecs, svc) = probe_video_codecs(&vec![
                SvcSettings {
                    codec: Codec::VP8,
                    scalability_mode: ScalabilityMode::L1T1,
                },
                SvcSettings {
                    codec: Codec::VP9,
                    scalability_mode: ScalabilityMode::L1T2,
                },
            ])
            .await;

            assert_eq!(svc, Some(ScalabilityMode::L1T1));
            assert_eq!(
                target_codecs_mime_types(&target_codecs),
                vec!["video/VP8", "video/red", "video/rtx", "video/ulpfec"]
            );
        }
    }

    #[wasm_bindgen_test]
    async fn empty_codec_probe_when_target_codec_not_found() {
        let (target_codecs, svc) = probe_video_codecs(&vec![]).await;
        assert!(target_codecs.is_empty());
        assert_eq!(svc, None);
    }

    // This test is necessary to identify changes in the list of available
    // browser codecs, allowing us to add new required service codecs or
    // support new video codecs.
    #[wasm_bindgen_test]
    async fn codec_capability_not_changed() {
        // List of codecs which are not fully supported by all browsers.
        const NOT_FULLY_SUPPORTED_CODECS: &[&str] = &["video/AV1"];

        let caps =
            CodecCapability::get_sender_codec_capabilities(MediaKind::Video)
                .await
                .unwrap();

        // Filter codecs which are not fully supported by all browsers.
        let mut codecs_caps: Vec<_> = target_codecs_mime_types(&caps)
            .into_iter()
            .filter(|c| !NOT_FULLY_SUPPORTED_CODECS.contains(&c.as_str()))
            .collect();
        codecs_caps.sort();
        assert_eq!(
            codecs_caps,
            vec![
                "video/H264",
                "video/VP8",
                "video/VP9",
                "video/red",
                "video/rtx",
                "video/ulpfec"
            ],
            "Browser available codecs are changed, check new codecs \
            and update this test"
        );
    }
}
