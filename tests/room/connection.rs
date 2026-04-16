use std::{cell::Cell, rc::Rc};

use medea_client_api_proto::{
    ConnectionMode, ConnectionQualityScore, MemberId, PeerConnectionState,
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
async fn sfu_quality_score_follows_server_not_peer_state() {
    let conn = get_test_connection(ConnectionMode::Sfu);
    let handle = api::ConnectionHandle::from(conn.new_handle());

    conn.update_peer_state(PeerConnectionState::Connected);

    let score: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    {
        let score = Rc::clone(&score);
        let cb = Closure::once_into_js(move |q: JsValue| {
            score.set(Some(q.as_f64().expect("number") as i32));
        });
        handle.on_quality_score_update(js_sys::Function::from(cb)).unwrap();
    }

    conn.update_quality_score(ConnectionQualityScore::Connected(70));
    assert_eq!(score.get(), Some(70));

    // SFU partner connection quality is driven by the server score, not by
    // local `PeerConnectionState` updates.
    conn.update_peer_state(PeerConnectionState::Disconnected);
    assert_eq!(score.get(), Some(70));

    conn.update_quality_score(ConnectionQualityScore::Disconnected);
    assert_eq!(score.get(), Some(-1));
}

#[wasm_bindgen_test]
async fn sfu_quality_score_not_emitted_before_quality_arrives() {
    let conn = get_test_connection(ConnectionMode::Sfu);
    let handle = api::ConnectionHandle::from(conn.new_handle());

    let score: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    {
        let score = Rc::clone(&score);
        let cb = Closure::once_into_js(move |q: JsValue| {
            score.set(Some(q.as_f64().expect("number") as i32));
        });
        handle.on_quality_score_update(js_sys::Function::from(cb)).unwrap();
    }

    conn.update_peer_state(PeerConnectionState::Connected);

    assert!(score.get().is_none());

    conn.update_quality_score(ConnectionQualityScore::Connected(81));
    assert_eq!(score.get(), Some(81));
}
