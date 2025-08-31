mod constraints;
mod manager;
mod track;

use std::rc::Rc;

use futures::channel::mpsc;
use medea_client_api_proto::{
    AudioSettings, ConnectionMode, Direction, MediaType, MemberId, Track,
    TrackId,
};
use medea_jason::{
    media::{MediaDirection, MediaManager, RecvConstraints},
    peer::{LocalStreamUpdateCriteria, MediaConnections, SimpleTracksRequest},
    platform::{RtcPeerConnection, TransceiverDirection},
};
use wasm_bindgen_test::*;

use crate::{MediaSourceKind, get_media_stream_settings};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn sendrecv_works() {
    let (tx, _rx) = mpsc::unbounded();
    let media_connections = MediaConnections::new(
        Rc::new(RtcPeerConnection::new(Vec::new(), false).await.unwrap()),
        tx,
    );
    let send_audio_track = Track {
        id: TrackId(1),
        direction: Direction::Send {
            receivers: vec![MemberId::from("bob")],
            mid: None,
        },
        media_direction: MediaDirection::SendRecv.into(),
        muted: false,
        media_type: MediaType::Audio(AudioSettings {
            required: false,
            source_kind: MediaSourceKind::Device,
        }),
    };
    let recv_audio_track = Track {
        id: TrackId(2),
        direction: Direction::Recv {
            mid: None,
            sender: MemberId::from("alice"),
        },
        media_direction: MediaDirection::SendRecv.into(),
        muted: false,
        media_type: MediaType::Audio(AudioSettings {
            required: false,
            source_kind: MediaSourceKind::Device,
        }),
    };
    media_connections
        .create_tracks(
            vec![send_audio_track, recv_audio_track],
            &get_media_stream_settings(true, false).into(),
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
    let sender = media_connections.get_sender_by_id(TrackId(1)).unwrap();
    let receiver = media_connections.get_receiver_by_id(TrackId(2)).unwrap();

    assert!(sender.is_publishing().await);
    assert!(receiver.is_receiving().await);
    assert_eq!(receiver.direction(), MediaDirection::SendRecv);

    assert!(
        sender
            .transceiver()
            .has_direction(
                TransceiverDirection::SEND | TransceiverDirection::RECV
            )
            .await,
    );
    assert!(
        receiver
            .transceiver()
            .unwrap()
            .has_direction(
                TransceiverDirection::SEND | TransceiverDirection::RECV
            )
            .await,
    );
}
