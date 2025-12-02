use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use futures::{
    StreamExt as _,
    channel::{mpsc, oneshot},
    stream,
};
use medea_client_api_proto::{
    ClientMsg, Command, ConnectionMode, Event, NegotiationRole,
    PeerConnectionState, PeerId, PeerMetrics, ServerMsg,
};
use medea_jason::{
    api,
    jason::JasonImpl,
    platform::{
        MockRpcTransport, RpcTransport, TransportError, TransportState,
    },
    rpc::WebSocketRpcClient,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use crate::{TEST_ROOM_URL, delay_for, rpc::RPC_SETTINGS, timeout};

wasm_bindgen_test_configure!(run_in_browser);

/// Setups a [`JasonImpl`] with one room and one peer, then calls
/// [`JasonImpl::network_changed()`].
///
/// Asserts:
///
/// 1. First RPC transport is dropped and the second is created.
/// 2. [`Command::SynchronizeMe`] is sent via second transport.
/// 3. [`PeerConnectionState::Failed`] is sent via second transport.
/// 4. Total of two [`Command::JoinRoom`] is sent. One for each transport
///    created.
/// 5. [`RoomHandle::on_connection_loss`] is not called since transport drop is
///    initiated by user.
#[wasm_bindgen_test]
async fn jason_network_changed() {
    // Captures commands sent by room.
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded::<ClientMsg>();

    // Track how many transports were created and whether first got dropped.
    let t1_dropped = Rc::new(RefCell::new(false));
    let created_count = Rc::new(RefCell::new(0));

    // First RPC transport that will be dropped after `network_changed()` call.
    let t1 = {
        let cmd_tx = cmd_tx.clone();
        let t1_dropped = t1_dropped.clone();
        move || {
            let mut t = MockRpcTransport::new();
            t.expect_connect()
                .return_once(|_| Box::pin(futures::future::ok(())));
            // Open once.
            t.expect_on_state_change().return_once(|| {
                Box::pin(stream::once(async { TransportState::Open }))
            });
            // Send` RpcSettings` and then pending.
            t.expect_on_message().returning(|| {
                Box::pin(
                    stream::iter(vec![
                        RPC_SETTINGS,
                        ServerMsg::Event {
                            room_id: "room_id".into(),
                            event: Event::RoomJoined {
                                member_id: "member_id".into(),
                                is_reconnect: false,
                            },
                        },
                        ServerMsg::Event {
                            room_id: "room_id".into(),
                            event: Event::PeerCreated {
                                peer_id: PeerId(1),
                                negotiation_role: NegotiationRole::Offerer,
                                tracks: Vec::new(),
                                ice_servers: Vec::new(),
                                force_relay: false,
                                connection_mode: ConnectionMode::Sfu,
                                stats_scrape_interval_ms: 1000,
                            },
                        },
                    ])
                    .chain(stream::pending()),
                )
            });
            // Forward all client messages into `cmd_tx`.
            let cmd_tx = cmd_tx.clone();
            t.expect_send().returning(move |msg| {
                cmd_tx.unbounded_send(msg.clone()).ok();
                Ok(())
            });
            t.expect_set_close_reason().returning_st({
                let t1_dropped = t1_dropped.clone();
                move |_| {
                    *t1_dropped.borrow_mut() = true;
                }
            });
            Rc::new(t) as Rc<dyn RpcTransport>
        }
    };

    // Second RPC transport mock that will be created after `network_changed()`
    // call.
    let t2 = {
        let cmd_tx = cmd_tx.clone();
        // Channel used to push server messages after transport is created.
        let t2_srv_tx: Arc<Mutex<Option<mpsc::UnboundedSender<ServerMsg>>>> =
            Arc::new(Mutex::new(None));
        move || {
            let mut t = MockRpcTransport::new();
            t.expect_connect()
                .return_once(|_| Box::pin(futures::future::ok(())));
            // Open once.
            t.expect_on_state_change().return_once(|| {
                Box::pin(stream::once(async { TransportState::Open }))
            });
            // `RpcSettings`, `RoomJoined`, and then allow further messages to
            // be pushed (e.g., `StateSynchronized` in response to
            // `SynchronizeMe`).
            t.expect_on_message().returning_st({
                let t2_srv_tx = t2_srv_tx.clone();
                move || {
                    let (tx, rx) = mpsc::unbounded();
                    tx.unbounded_send(RPC_SETTINGS).unwrap();
                    tx.unbounded_send(ServerMsg::Event {
                        room_id: "room_id".into(),
                        event: Event::RoomJoined {
                            member_id: "member_id".into(),
                            is_reconnect: true,
                        },
                    })
                    .unwrap();
                    *t2_srv_tx.lock().unwrap() = Some(tx);
                    Box::pin(rx)
                }
            });
            // Forward sends to the same sink.
            let cmd_tx = cmd_tx.clone();
            let t2_srv_tx = t2_srv_tx.clone();
            t.expect_send().returning(move |msg| {
                // Mirror the sent command into the test channel.
                cmd_tx.unbounded_send(msg.clone()).ok();

                // If this is `SynchronizeMe`, emit `StateSynchronized` back
                // from the server using the same state payload.
                if let ClientMsg::Command { room_id, command } = msg.clone() {
                    if let Command::SynchronizeMe { state } = command {
                        if let Some(tx) =
                            t2_srv_tx.lock().unwrap().as_ref().cloned()
                        {
                            tx.unbounded_send(ServerMsg::Event {
                                room_id,
                                event: Event::StateSynchronized { state },
                            })
                            .ok();
                        }
                    }
                }

                Ok(())
            });
            // Accept close reason calls (not important for second).
            t.expect_set_close_reason().return_const(());
            Rc::new(t) as Rc<dyn RpcTransport>
        }
    };

    // Transport factory expected to return 2 transports.
    let ws = Rc::new(WebSocketRpcClient::new({
        let created_count = created_count.clone();
        Box::new(move || {
            let mut n = *created_count.borrow();
            n += 1;
            *created_count.borrow_mut() = n;
            if n == 1 { t1() } else { t2() }
        })
    }));

    let jason = api::Jason::from(JasonImpl::new(Some(ws.clone())));

    // Init room and join.
    let room = jason.init_room();
    room.on_failed_local_media(Closure::once_into_js(|| {}).into()).unwrap();
    let (loss_tx, loss_rx) = oneshot::channel::<()>();
    let on_loss = wasm_bindgen::closure::Closure::once_into_js(move || {
        loss_tx.send(()).ok();
    });
    room.on_connection_loss(on_loss.into()).unwrap();
    JsFuture::from(room.join(TEST_ROOM_URL.to_string())).await.unwrap();

    // Verify first transport created.
    assert_eq!(*created_count.borrow(), 1);

    // Trigger network change.
    JsFuture::from(jason.network_changed()).await.unwrap();

    // Give reconnection some time.
    delay_for(50).await;

    // Second transport must be created, first must have been dropped with
    // reason.
    assert_eq!(*created_count.borrow(), 2);
    assert!(*t1_dropped.borrow());

    // We expect two `Command::JoinRoom`s - one for each transport,
    // and a single `PeerConnectionState::Failed` so server restarts ICE.
    let mut join_room_sent = 0;
    let mut peer_conn_failed_sent = false;
    let mut sync_request_sent = false;
    let start = web_sys::window().unwrap().performance().unwrap().now();
    while web_sys::window().unwrap().performance().unwrap().now() - start
        < 1000.0
    {
        if let Ok(Some(msg)) = timeout(100, cmd_rx.next()).await {
            if let ClientMsg::Command { command, .. } = msg {
                if matches!(command, Command::JoinRoom { .. }) {
                    join_room_sent += 1;
                } else if matches!(command, Command::SynchronizeMe { .. }) {
                    sync_request_sent = true;
                } else if matches!(
                    command,
                    Command::AddPeerConnectionMetrics {
                        metrics: PeerMetrics::PeerConnectionState(
                            PeerConnectionState::Failed
                        ),
                        ..
                    }
                ) {
                    peer_conn_failed_sent = true;
                    break;
                }
            }
        }
    }

    assert!(
        peer_conn_failed_sent,
        "`PeerConnectionState::Failed` must be sent after \
         `Jason::network_changed()` call",
    );
    assert!(
        sync_request_sent,
        "`Command::SynchronizeMe` must be sent after RPC reconnection",
    );
    assert_eq!(join_room_sent, 2, "`JoinRoom` not sent after reconnection");

    // Ensure `on_connection_loss` callback wasn't called during controlled
    // reconnection flow.
    assert!(
        timeout(300, loss_rx).await.is_err(),
        "`on_connection_loss` callback must not be called during \
         `network_changed()`",
    );
}

/// Checks that [`JasonImpl::network_changed()`] errors if the new transport
/// fails to open.
#[wasm_bindgen_test]
async fn jason_network_changed_errors_on_failed_reconnect() {
    let (cmd_tx, _cmd_rx) = mpsc::unbounded::<ClientMsg>();

    let created_count = Rc::new(RefCell::new(0));

    // First transport: normal open and join.
    let t1 = {
        let cmd_tx = cmd_tx.clone();
        move || {
            let mut t = MockRpcTransport::new();
            t.expect_connect()
                .return_once(|_| Box::pin(futures::future::ok(())));
            t.expect_on_state_change().return_once(|| {
                Box::pin(stream::once(async { TransportState::Open }))
            });
            t.expect_on_message().returning(|| {
                Box::pin(
                    stream::iter(vec![
                        RPC_SETTINGS,
                        ServerMsg::Event {
                            room_id: "room_id".into(),
                            event: Event::RoomJoined {
                                member_id: "member_id".into(),
                                is_reconnect: false,
                            },
                        },
                    ])
                    .chain(stream::pending()),
                )
            });
            let cmd_tx = cmd_tx.clone();
            t.expect_send().returning(move |msg| {
                cmd_tx.unbounded_send(msg.clone()).ok();
                Ok(())
            });
            t.expect_set_close_reason().returning_st(move |_| {});
            Rc::new(t) as Rc<dyn RpcTransport>
        }
    };

    // Second transport: connects but sends wrong first message
    // (not `RpcSettings`), which should make the reconnect fail.
    let t2 = move || {
        let mut t = MockRpcTransport::new();
        t.expect_connect().return_once(|_| {
            Box::pin(futures::future::err(tracerr::new!(
                TransportError::InitSocket
            )))
        });
        t.expect_on_state_change().return_once(|| {
            Box::pin(stream::once(async { TransportState::Open }))
        });
        t.expect_on_message().returning(|| Box::pin(stream::pending()));
        t.expect_send().returning(|_| Ok(()));
        t.expect_set_close_reason().return_const(());
        Rc::new(t) as Rc<dyn RpcTransport>
    };

    let ws = Rc::new(WebSocketRpcClient::new({
        let created_count = created_count.clone();
        Box::new(move || {
            let mut n = *created_count.borrow();
            n += 1;
            *created_count.borrow_mut() = n;
            if n == 1 { t1() } else { t2() }
        })
    }));

    let jason = api::Jason::from(JasonImpl::new(Some(ws.clone())));
    let room = jason.init_room();
    room.on_failed_local_media(Closure::once_into_js(|| {}).into()).unwrap();
    room.on_connection_loss(Closure::once_into_js(|| {}).into()).unwrap();

    JsFuture::from(room.join(TEST_ROOM_URL.to_string())).await.unwrap();
    // This should error due to failed reconnection.
    assert!(
        JsFuture::from(jason.network_changed()).await.is_err(),
        "reconnection should have failed, but it hasn't",
    );
}
