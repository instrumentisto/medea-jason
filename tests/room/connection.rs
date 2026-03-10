use std::{cell::Cell, rc::Rc};

use medea_client_api_proto::{
    ConnectionMode, ConnectionQualityScore, MemberId, PeerConnectionState,
    PeerId,
};
use medea_jason::{api, connection::Connection, media::RecvConstraints};
use wasm_bindgen::{JsValue, closure::Closure};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn get_test_connection(mode: ConnectionMode) -> Connection {
    Connection::new(
        MemberId::from("Test"),
        &Rc::new(RecvConstraints::default()),
        mode,
    )
}

#[wasm_bindgen_test]
async fn p2p_initial_state_is_none() {
    let conn = get_test_connection(ConnectionMode::Mesh);
    let handle = api::ConnectionHandle::from(conn.new_handle());

    assert!(handle.get_state().unwrap().is_none());
}

#[wasm_bindgen_test]
async fn p2p_connection_state_accessible() {
    let conn = get_test_connection(ConnectionMode::Mesh);
    let handle = api::ConnectionHandle::from(conn.new_handle());

    conn.update_peer_state(PeerId(0), PeerConnectionState::Connecting);

    let state = handle.get_state().unwrap().unwrap();
    assert_eq!(state.kind(), api::MemberConnectionStateKind::P2P);
    assert_eq!(
        state.value(),
        JsValue::from(api::PeerConnectionState::Connecting as u8),
    );
}

#[wasm_bindgen_test]
async fn p2p_on_state_change_fires() {
    let conn = get_test_connection(ConnectionMode::Mesh);
    let handle = api::ConnectionHandle::from(conn.new_handle());

    let fired: Rc<Cell<Option<api::MemberConnectionState>>> =
        Rc::new(Cell::default());
    {
        let fired = Rc::downgrade(&fired);
        let cb = Closure::once_into_js(move |s: api::MemberConnectionState| {
            fired.upgrade().unwrap().set(Some(s));
        });
        handle.on_state_change(js_sys::Function::from(cb)).unwrap();
    }

    conn.update_peer_state(PeerId(0), PeerConnectionState::Connected);

    let state = fired.get().unwrap();
    assert_eq!(state.kind(), api::MemberConnectionStateKind::P2P);
    assert_eq!(
        state.value(),
        JsValue::from(api::PeerConnectionState::Connected as u8),
    );
}

#[wasm_bindgen_test]
async fn sfu_quality_score_disconnected_when_any_peer_fails() {
    let conn = get_test_connection(ConnectionMode::Sfu);
    let handle = api::ConnectionHandle::from(conn.new_handle());

    // Connect both peers first
    conn.update_peer_state(PeerId(0), PeerConnectionState::Connected);
    conn.update_peer_state(PeerId(1), PeerConnectionState::Connected);

    let score: Rc<Cell<Option<u8>>> = Rc::new(Cell::new(None));
    {
        let score = Rc::clone(&score);
        let cb = Closure::once_into_js(move |s: u8| score.set(Some(s)));
        handle.on_quality_score_update(js_sys::Function::from(cb)).unwrap();
    }

    // One peer disconnects so quality score must immediately become 0.
    conn.update_peer_state(PeerId(0), PeerConnectionState::Disconnected);
    assert_eq!(score.get(), Some(0u8));
}

#[wasm_bindgen_test]
async fn sfu_quality_score_not_emitted_before_quality_arrives() {
    let conn = get_test_connection(ConnectionMode::Sfu);
    let handle = api::ConnectionHandle::from(conn.new_handle());

    let score: Rc<Cell<Option<u8>>> = Rc::new(Cell::new(None));
    {
        let score = Rc::clone(&score);
        let cb = Closure::once_into_js(move |s: u8| score.set(Some(s)));
        handle.on_quality_score_update(js_sys::Function::from(cb)).unwrap();
    }

    conn.update_peer_state(PeerId(0), PeerConnectionState::Connected);
    conn.update_peer_state(PeerId(1), PeerConnectionState::Connected);

    assert!(score.get().is_none());

    conn.update_quality_score(ConnectionQualityScore::Medium);
    assert_eq!(score.get(), Some(ConnectionQualityScore::Medium as u8));
}
