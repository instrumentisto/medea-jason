#![cfg(target_arch = "wasm32")]

mod transitable_state;

use std::{mem, rc::Rc};

use futures::channel::mpsc;
use medea_client_api_proto::{ConnectionMode, TrackId, TrackPatchEvent};
use medea_jason::{
    media::{LocalTracksConstraints, MediaManager, RecvConstraints},
    peer::{
        LocalStreamUpdateCriteria, MediaConnections, MediaStateControllable,
        SimpleTracksRequest, media_exchange_state,
    },
    platform::RtcPeerConnection,
    utils::Updatable as _,
};
use wasm_bindgen_test::*;

use crate::{
    get_media_stream_settings, get_test_unrequired_tracks, is_firefox,
    local_constraints,
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

    let audio_track =
        media_connections.get_sender_state_by_id(audio_track_id).unwrap();
    let video_track =
        media_connections.get_sender_state_by_id(video_track_id).unwrap();

    assert!(!audio_track.enabled());
    assert!(video_track.enabled());
}

#[wasm_bindgen_test]
async fn new_media_connections_with_disabled_video_tracks() {
    let (media_connections, audio_track_id, video_track_id) =
        get_test_media_connections(true, false).await;

    let audio_track =
        media_connections.get_sender_state_by_id(audio_track_id).unwrap();
    let video_track =
        media_connections.get_sender_state_by_id(video_track_id).unwrap();

    assert!(audio_track.enabled());
    assert!(!video_track.enabled());
}

/// Tests for [`Sender::update`] function.
///
/// This tests checks that [`TrackPatch`] works as expected.
mod sender_patch {
    use medea_client_api_proto::{
        AudioSettings, ConnectionMode, EncodingParameters, MediaDirection,
        MediaSourceKind, MediaType, ScalabilityMode, VideoSettings,
    };
    use medea_jason::{
        peer::{MediaExchangeState, sender},
        utils::{AsProtoState, SynchronizableState, Updatable},
    };

    use super::*;

    async fn audio_sender() -> (sender::Component, TrackId, MediaConnections) {
        build_sender(MediaType::Audio(AudioSettings {
            required: false,
            source_kind: MediaSourceKind::Device,
        }))
        .await
    }

    async fn video_sender(
        encoding_parameters: Vec<EncodingParameters>,
    ) -> (sender::Component, TrackId, MediaConnections) {
        build_sender(MediaType::Video(VideoSettings {
            required: false,
            source_kind: MediaSourceKind::Device,
            encoding_parameters,
        }))
        .await
    }

    async fn build_sender(
        kind: MediaType,
    ) -> (sender::Component, TrackId, MediaConnections) {
        let (tx, rx) = mpsc::unbounded();
        mem::forget(rx);
        let media_connections = MediaConnections::new(
            Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
            tx,
        );
        let sender = media_connections
            .create_sender(
                TrackId(0),
                kind,
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
        let (sender, track_id, _media_connections) = audio_sender().await;
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
        let (sender, track_id, _media_connections) = audio_sender().await;
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
        let (sender, track_id, _media_connections) = audio_sender().await;
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
        let (sender, track_id, _media_connections) = audio_sender().await;
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
        let (sender, track_id, _media_connections) = audio_sender().await;
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

    #[wasm_bindgen_test]
    async fn add_transceiver_send_encodings() {
        let (sender, _, _media_connections) = video_sender(vec![
            EncodingParameters {
                rid: "h".to_owned(),
                scalability_mode: None,
                active: true,
                max_bitrate: None,
                scale_resolution_down_by: None,
                codec: None,
            },
            EncodingParameters {
                rid: "l".to_owned(),
                scalability_mode: Some(ScalabilityMode::L3T3),
                active: true,
                max_bitrate: Some(100),
                scale_resolution_down_by: Some(2),
                codec: None,
            },
        ])
        .await;

        let encs = sender.get_send_encodings().await;
        assert_eq!(encs.len(), 2);

        assert_eq!(encs[0].rid(), Some("h".to_owned()));
        assert_eq!(encs[0].active(), true);
        assert_eq!(encs[0].scale_resolution_down_by(), 1.0);
        assert_eq!(encs[0].max_bitrate(), None);
        assert_eq!(encs[0].scalability_mode(), None);

        assert_eq!(encs[1].rid(), Some("l".to_owned()));
        assert_eq!(encs[1].active(), true);
        assert_eq!(encs[1].scale_resolution_down_by(), 2.0);
        assert_eq!(encs[1].max_bitrate(), Some(100));
        if !is_firefox() {
            // TODO: Scalability mode is not supported in Firefox as of v135:
            // https://bugzilla.mozilla.org/show_bug.cgi?id=1571470
            assert_eq!(encs[1].scalability_mode(), Some("L3T3".to_owned()));
        }
    }

    #[wasm_bindgen_test]
    async fn update_send_encodings() {
        let (sender, track_id, _media_connections) =
            video_sender(vec![EncodingParameters {
                rid: "0".to_owned(),
                scalability_mode: None,
                active: true,
                max_bitrate: None,
                scale_resolution_down_by: None,
                codec: None,
            }])
            .await;

        sender.state().when_updated().await;

        sender.state().update(TrackPatchEvent {
            id: track_id,
            receivers: None,
            media_direction: None,
            muted: None,
            encoding_parameters: Some(vec![EncodingParameters {
                rid: "asdasd".to_owned(), // does not matter
                active: false,
                codec: None,
                max_bitrate: Some(100),
                scale_resolution_down_by: Some(2),
                scalability_mode: Some(ScalabilityMode::L1T2),
            }]),
        });

        sender.state().when_updated().await;

        let encs = sender.get_send_encodings().await;
        assert_eq!(encs.len(), 1);

        // Only one encoding so rid is empty
        assert_eq!(encs[0].rid(), None);
        assert_eq!(encs[0].active(), false);
        assert_eq!(encs[0].scale_resolution_down_by(), 2.0);
        assert_eq!(encs[0].max_bitrate(), Some(100));
        if !is_firefox() {
            // TODO: Scalability mode is not supported in Firefox as of v135:
            //       https://bugzilla.mozilla.org/show_bug.cgi?id=1571470
            assert_eq!(encs[0].scalability_mode(), Some("L1T2".to_owned()));
        }
    }

    /// Checks that [`Sender`]'s mute and media exchange states can be changed
    /// by [`SenderState`] update.
    #[wasm_bindgen_test]
    async fn update_by_state() {
        let (sender, _, _media_connections) = audio_sender().await;

        let mut proto_state = sender.state().as_proto();
        proto_state.media_direction = MediaDirection::RecvOnly;
        proto_state.muted = true;
        sender.state().apply(proto_state, &LocalTracksConstraints::default());
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
        peer::{MediaExchangeState, PeerEvent, receiver},
        utils::{AsProtoState, SynchronizableState},
    };

    use super::*;
    use crate::MediaSourceKind;

    const TRACK_ID: TrackId = TrackId(0);
    const MID: &str = "mid";
    const SENDER_ID: &str = "sender";

    async fn get_receiver()
    -> (receiver::Component, mpsc::UnboundedReceiver<PeerEvent>) {
        let (tx, rx) = mpsc::unbounded();
        let media_connections = MediaConnections::new(
            Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
            tx,
        );
        let recv = media_connections
            .create_receiver(
                TRACK_ID,
                MediaType::Audio(AudioSettings {
                    required: true,
                    source_kind: MediaSourceKind::Device,
                })
                .into(),
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

        receiver.state().apply(proto_state, &LocalTracksConstraints::default());

        receiver.state().when_updated().await;
        assert!(!receiver.state().enabled_general());
        assert_eq!(
            receiver.state().media_exchange_state(),
            MediaExchangeState::Stable(media_exchange_state::Stable::Disabled)
        );
    }
}

mod codec_probing {
    use std::collections::HashMap;

    use medea_client_api_proto::Codec;
    use medea_jason::{
        media::MediaKind,
        platform::{CodecCapability, transceiver::probe_target_codecs},
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
        let target_codecs = probe_target_codecs(&[Codec {
            mime_type: "video/VP8".to_owned(),
            clock_rate: 90000,
            channels: None,
            parameters: HashMap::new(),
        }])
        .await
        .expect("all clients are expected to support VP8");

        assert_eq!(
            target_codecs_mime_types(&target_codecs),
            vec!["video/VP8", "video/red", "video/rtx", "video/ulpfec"]
        );
    }

    #[wasm_bindgen_test]
    async fn bad_params() {
        let data = &[
            vec![Codec {
                mime_type: "video/VP8".to_owned(),
                clock_rate: 90000,
                channels: None,
                // non existent param
                parameters: HashMap::from([(
                    "unknown_param".to_owned(),
                    "value".to_owned(),
                )]),
            }],
            vec![Codec {
                mime_type: "video/VP9".to_owned(),
                clock_rate: 90000,
                channels: None,
                // good param + non existent param
                parameters: HashMap::from([
                    ("profile-id".to_owned(), "0".to_owned()),
                    ("unknown_param".to_owned(), "value".to_owned()),
                ]),
            }],
            vec![Codec {
                mime_type: "video/H264".to_owned(),
                clock_rate: 90000,
                channels: None,
                // bad param value
                parameters: HashMap::from([(
                    "profile-level-id".to_owned(),
                    "ffffff".to_owned(),
                )]),
            }],
            vec![], // empty target list
        ];

        for bad_target in data {
            assert!(probe_target_codecs(bad_target).await.is_none());
        }
    }

    #[wasm_bindgen_test]
    async fn correct_priority() {
        let mut target = vec![
            Codec {
                mime_type: "video/VP8".to_owned(),
                clock_rate: 90000,
                channels: None,
                parameters: HashMap::new(),
            },
            Codec {
                mime_type: "video/VP9".to_owned(),
                clock_rate: 90000,
                channels: None,
                parameters: HashMap::new(),
            },
        ];

        let mut res: Vec<String> = probe_target_codecs(&target)
            .await
            .expect("all clients are expected to support VP8 and VP9")
            .into_iter()
            .map(|c| c.mime_type())
            .collect();
        res.dedup();

        assert_eq!(res[0..2].to_vec(), &["video/VP8", "video/VP9"]);

        // reverse and repeat
        target.reverse();

        let mut res: Vec<String> = probe_target_codecs(&target)
            .await
            .expect("all clients are expected to support VP8 and VP9")
            .into_iter()
            .map(|c| c.mime_type())
            .collect();
        res.dedup();
        assert_eq!(res[0..2].to_vec(), &["video/VP9", "video/VP8"]);
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
                "video/ulpfec",
            ],
            "Browser available codecs are changed, check new codecs \
             and update this test",
        );
    }
}
