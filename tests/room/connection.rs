use std::{cell::Cell, rc::Rc};

use medea_client_api_proto::{ConnectionMode, MemberId, PeerConnectionState};
use medea_jason::{api, connection::Connection, media::RecvConstraints};
use wasm_bindgen::{JsValue, closure::Closure};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn get_test_connection() -> Connection {
    let member_id = MemberId::from("Test");
    let constraints = Rc::new(RecvConstraints::default());

    Connection::new(member_id, &constraints, ConnectionMode::Mesh)
}

#[wasm_bindgen_test]
async fn initial_state_is_null() {
    let connection = get_test_connection();
    let connection_handle =
        api::ConnectionHandle::from(connection.new_handle());

    assert!(connection_handle.get_state().unwrap().is_none());
}

#[wasm_bindgen_test]
async fn p2p_connection_state_accessible() {
    let connection = get_test_connection();
    let connection_handle =
        api::ConnectionHandle::from(connection.new_handle());

    connection.update_peer_state(PeerConnectionState::Connecting);

    let state = connection_handle.get_state().unwrap().unwrap();

    assert_eq!(state.kind(), api::MemberConnectionStateKind::P2P);
    assert_eq!(
        state.value(),
        JsValue::from(api::PeerConnectionState::Connecting as u8),
    );
}

#[wasm_bindgen_test]
async fn callback_fires_on_p2p_state_change() {
    let connection = get_test_connection();
    let connection_handle =
        api::ConnectionHandle::from(connection.new_handle());

    let state: Rc<Cell<Option<api::MemberConnectionState>>> =
        Rc::new(Cell::default());

    {
        let state = Rc::downgrade(&state);
        let cb = Closure::once_into_js(
            move |updated: api::MemberConnectionState| {
                state.upgrade().unwrap().set(Some(updated));
            },
        );
        connection_handle.on_state_change(js_sys::Function::from(cb)).unwrap();
    }

    connection.update_peer_state(PeerConnectionState::Connected);

    let state = state.get().unwrap();

    assert_eq!(state.kind(), api::MemberConnectionStateKind::P2P);
    assert_eq!(
        state.value(),
        JsValue::from(api::PeerConnectionState::Connected as u8),
    );
}
