#![cfg(target_arch = "wasm32")]

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

use futures::{FutureExt as _, StreamExt as _, future, stream};
use medea_client_api_proto::{
    ClientMsg, CloseReason, Command, Event, ServerMsg,
};
use medea_jason::{
    platform::{
        self, MockRpcTransport, RpcTransport, TransportState,
        WebSocketRpcTransport,
    },
    rpc::{
        CloseMsg, ConnectionInfo, RpcSession, SessionError, WebSocketRpcClient,
        WebSocketRpcSession,
    },
};
use wasm_bindgen_test::*;

use crate::{TEST_ROOM_URL, delay_for, rpc::RPC_SETTINGS, timeout};

wasm_bindgen_test_configure!(run_in_browser);

/// Makes sure that `connect` fails immediately if `JoinRoom` request is
/// answered with `RoomLeft` message.
#[wasm_bindgen_test]
async fn could_not_auth_err() {
    let session = WebSocketRpcSession::new(Rc::new(WebSocketRpcClient::new(
        Box::new(move || {
            let mut transport = MockRpcTransport::new();
            transport
                .expect_connect()
                .return_once(|_| Box::pin(future::ok(())));
            transport.expect_on_message().returning_st(|| {
                Box::pin(stream::iter(vec![
                    RPC_SETTINGS,
                    ServerMsg::Event {
                        room_id: "room_id".into(),
                        event: Event::RoomLeft {
                            close_reason: CloseReason::InternalError,
                        },
                    },
                ]))
            });
            transport.expect_send().returning(|_| Ok(()));
            transport.expect_set_close_reason().return_once(drop);
            transport.expect_on_state_change().return_once_st(move || {
                Box::pin(stream::once(async { TransportState::Open }))
            });
            let transport = Rc::new(transport);
            transport as Rc<dyn RpcTransport>
        }),
    )));

    let mut on_normal_close = session.on_normal_close().fuse();
    let mut on_reconnected = session.on_reconnected().fuse();
    let mut on_connection_loss = session.on_connection_loss().fuse();

    let connect_fut = Rc::clone(&session)
        .connect(ConnectionInfo::from_str(TEST_ROOM_URL).unwrap());
    let connect_err =
        timeout(100, connect_fut).await.unwrap().unwrap_err().into_inner();
    assert!(matches!(connect_err, SessionError::AuthorizationFailed));

    // other callbacks should not fire
    futures::select! {
        _ = delay_for(100).fuse() => (),
        _ = on_normal_close => panic!("on_normal_close fired"),
        _ = on_connection_loss.next() => panic!("on_connection_loss fired"),
        _ = on_reconnected.next() => panic!("on_reconnected fired")
    };
}

/// Makes sure that if multiple concurrent `connect` and `reconnect` calls are
/// made, only one `JoinRoom` message will be sent.
#[wasm_bindgen_test]
async fn concurrent_connect_requests() {
    let join_room_sent = Rc::new(AtomicBool::new(false));

    let join_room_sent_clone = Rc::clone(&join_room_sent);
    let session = WebSocketRpcSession::new(Rc::new(WebSocketRpcClient::new({
        Box::new(move || {
            let join_room_sent = Rc::clone(&join_room_sent_clone);
            let mut transport = MockRpcTransport::new();
            transport
                .expect_connect()
                .return_once(|_| Box::pin(future::ok(())));
            transport.expect_on_message().returning_st(|| {
                Box::pin(stream::iter(vec![
                    RPC_SETTINGS,
                    ServerMsg::Event {
                        room_id: "room_id".into(),
                        event: Event::RoomJoined {
                            member_id: "member_id".into(),
                            is_reconnect: false,
                        },
                    },
                ]))
            });
            let join_room_sent = Rc::clone(&join_room_sent);
            transport.expect_send().returning_st(move |msg| {
                if matches!(
                    msg,
                    ClientMsg::Command {
                        command: Command::JoinRoom { .. },
                        ..
                    }
                ) {
                    let already_sent =
                        join_room_sent.fetch_or(true, Ordering::Relaxed);
                    assert!(!already_sent, "only one JoinRoom should be sent");
                }
                Ok(())
            });
            transport.expect_set_close_reason().return_once(drop);
            transport.expect_on_state_change().return_once_st(move || {
                Box::pin(stream::once(async { TransportState::Open }))
            });
            let transport = Rc::new(transport);
            transport as Rc<dyn RpcTransport>
        })
    })));

    let connection_info = ConnectionInfo::from_str(TEST_ROOM_URL).unwrap();

    let connect1 = Rc::clone(&session).connect(connection_info.clone());
    let reconnect1 = Rc::clone(&session).reconnect();
    let connect2 = Rc::clone(&session).connect(connection_info);
    let reconnect2 = Rc::clone(&session).reconnect();

    future::try_join_all(vec![connect1, reconnect1, connect2, reconnect2])
        .await
        .unwrap();
    assert!(join_room_sent.load(Ordering::Relaxed));
}

/// Makes sure that `connect` fails immediately if transport establishment
/// failed.
#[wasm_bindgen_test]
async fn could_not_open_transport() {
    let session = WebSocketRpcSession::new(Rc::new(WebSocketRpcClient::new(
        Box::new(|| {
            let ws = WebSocketRpcTransport::new();
            Rc::new(ws) as Rc<dyn RpcTransport>
        }),
    )));

    let mut on_normal_close = session.on_normal_close().fuse();
    let mut on_reconnected = session.on_reconnected().fuse();
    let mut on_connection_loss = session.on_connection_loss().fuse();

    let connect_fut = Rc::clone(&session).connect(
        ConnectionInfo::from_str(
            "ws://localhost:55555/some/fake?token=endpoint",
        )
        .unwrap(),
    );

    // connect resolve with err
    timeout(100, connect_fut).await.unwrap().unwrap_err();

    // other callbacks should not fire
    futures::select! {
        _ = delay_for(100).fuse() => (),
        _ = on_normal_close => panic!("on_normal_close fired"),
        _ = on_connection_loss.next() => panic!("on_connection_loss fired"),
        _ = on_reconnected.next() => panic!("on_reconnected fired")
    };
}

/// Makes sure that `on_connection_loss` is fired when transport closes with
/// non-normal close and reconnect works as expected.
#[wasm_bindgen_test]
async fn reconnect_after_transport_abnormal_close() {
    let commands_sent = Rc::new(RefCell::new(Vec::new()));

    let is_reconnect = Rc::new(AtomicBool::new(false));
    let commands_sent_clone = Rc::clone(&commands_sent);
    let session =
        WebSocketRpcSession::new(Rc::new(WebSocketRpcClient::new(Box::new({
            let is_reconnect = Rc::clone(&is_reconnect);
            move || {
                let commands_sent_clone = Rc::clone(&commands_sent_clone);
                let is_reconnect = Rc::clone(&is_reconnect);
                let mut transport = MockRpcTransport::new();
                transport
                    .expect_connect()
                    .return_once(|_| Box::pin(future::ok(())));
                transport.expect_on_message().returning_st(move || {
                    Box::pin(stream::iter(vec![
                        RPC_SETTINGS,
                        ServerMsg::Event {
                            room_id: "room_id".into(),
                            event: Event::RoomJoined {
                                member_id: "member_id".into(),
                                is_reconnect: is_reconnect
                                    .load(Ordering::Relaxed),
                            },
                        },
                    ]))
                });
                let commands_sent = Rc::clone(&commands_sent_clone);
                transport.expect_send().returning_st(move |msg| {
                    commands_sent.borrow_mut().push(msg.clone());
                    Ok(())
                });
                transport.expect_set_close_reason().return_once(drop);
                transport.expect_on_state_change().return_once_st(move || {
                    Box::pin(
                        stream::once(future::ready(TransportState::Open))
                            .chain(stream::once(async {
                                delay_for(20).await;
                                TransportState::Closed(CloseMsg::Abnormal(999))
                            })),
                    )
                });
                let transport = Rc::new(transport);
                transport as Rc<dyn RpcTransport>
            }
        }))));

    let mut on_normal_close = session.on_normal_close().fuse();
    let mut on_reconnected = session.on_reconnected().fuse();
    let mut on_connection_loss = session.on_connection_loss().fuse();

    let connect_fut = Rc::clone(&session)
        .connect(ConnectionInfo::from_str(TEST_ROOM_URL).unwrap());
    timeout(100, connect_fut).await.unwrap().unwrap();

    // on_connection_loss fires
    futures::select! {
        _ = delay_for(100).fuse() => panic!("on_connection_loss should fire"),
        _ = on_normal_close => panic!("on_normal_close fired"),
        _ = on_connection_loss.next() => (),
        _ = on_reconnected.next() => panic!("on_reconnected fired")
    };

    // successful reconnect after connection loss
    is_reconnect.store(true, Ordering::Relaxed);
    Rc::clone(&session).reconnect().await.unwrap();
    on_reconnected.select_next_some().await;

    let capabilities = platform::get_capabilities().await;

    drop(session);
    assert_eq!(
        *commands_sent.borrow(),
        vec![
            // connect
            ClientMsg::Command {
                room_id: "room_id".into(),
                command: Command::JoinRoom {
                    member_id: "member_id".into(),
                    credential: "token".into(),
                    capabilities: capabilities.clone(),
                },
            },
            // reconnect
            ClientMsg::Command {
                room_id: "room_id".into(),
                command: Command::JoinRoom {
                    member_id: "member_id".into(),
                    credential: "token".into(),
                    capabilities,
                },
            },
        ],
    );
}

/// Ensures that `on_reconnected()` is emitted only when
/// `Event::RoomJoined { is_reconnect: true }` is received (and not on the
/// initial join with `is_reconnect: false`).
#[wasm_bindgen_test]
async fn on_reconnected_emits_only_on_true() {
    // 3 joins total: with is_reconnect [false, false, true]
    let is_reconnect = Rc::new(AtomicBool::new(false));
    let commands_sent = Rc::new(RefCell::new(Vec::new()));
    let commands_sent_clone = Rc::clone(&commands_sent);
    let is_reconnect_clone = Rc::clone(&is_reconnect);
    let session =
        WebSocketRpcSession::new(Rc::new(WebSocketRpcClient::new(Box::new({
            move || {
                let is_reconnect = Rc::clone(&is_reconnect_clone);
                let commands_sent = Rc::clone(&commands_sent_clone);
                let mut transport = MockRpcTransport::new();
                transport
                    .expect_connect()
                    .return_once(|_| Box::pin(future::ok(())));
                transport.expect_on_message().returning_st(move || {
                    let is_reconnect = is_reconnect.load(Ordering::Relaxed);
                    Box::pin(stream::iter(vec![
                        RPC_SETTINGS,
                        ServerMsg::Event {
                            room_id: "room_id".into(),
                            event: Event::RoomJoined {
                                member_id: "member_id".into(),
                                is_reconnect,
                            },
                        },
                    ]))
                });
                transport.expect_send().returning_st({
                    let commands_sent = Rc::clone(&commands_sent);
                    move |msg| {
                        commands_sent.borrow_mut().push(msg.clone());
                        Ok(())
                    }
                });
                transport.expect_set_close_reason().return_once(drop);
                transport.expect_on_state_change().return_once_st(move || {
                    Box::pin(
                        stream::once(future::ready(TransportState::Open))
                            .chain(stream::once(async {
                                delay_for(20).await;
                                TransportState::Closed(CloseMsg::Abnormal(999))
                            })),
                    )
                });
                let transport = Rc::new(transport);
                transport as Rc<dyn RpcTransport>
            }
        }))));

    let mut on_reconnected = session.on_reconnected().fuse();
    let mut on_connection_loss = session.on_connection_loss().fuse();

    // Simulate client behavior: on reconnected, send SynchronizeMe
    {
        let session_for_sync = Rc::clone(&session);
        platform::spawn(async move {
            let mut rx = session_for_sync.on_reconnected();
            while rx.next().await.is_some() {
                session_for_sync.send_command(Command::SynchronizeMe {
                    state: medea_client_api_proto::state::Room {
                        peers: HashMap::new(),
                    },
                });
            }
        });
    }

    // Initial connect with is_reconnect = false
    let connect_fut = Rc::clone(&session)
        .connect(ConnectionInfo::from_str(TEST_ROOM_URL).unwrap());
    timeout(100, connect_fut).await.unwrap().unwrap();

    // on_reconnected must NOT fire on the initial join
    futures::select! {
        _ = delay_for(100).fuse() => (),
        _ = on_reconnected.next() =>
            panic!("on_reconnected fired on initial join"),
    };

    // Ensure no SynchronizeMe sent yet
    assert_eq!(
        commands_sent
            .borrow()
            .iter()
            .filter(|m| matches!(
                m,
                ClientMsg::Command {
                    command: Command::SynchronizeMe { .. },
                    ..
                }
            ))
            .count(),
        0
    );

    // Wait for connection loss #1
    on_connection_loss.select_next_some().await;
    // First reconnect (is_reconnect = false) -> on_reconnected must NOT fire
    Rc::clone(&session).reconnect().await.unwrap();
    futures::select! {
        _ = delay_for(100).fuse() => (),
        _ = on_reconnected.next() => panic!("on_reconnected fired on first \
            reconnect with is_reconnect = false"),
    };

    // Still no SynchronizeMe on first reconnect
    assert_eq!(
        commands_sent
            .borrow()
            .iter()
            .filter(|m| matches!(
                m,
                ClientMsg::Command {
                    command: Command::SynchronizeMe { .. },
                    ..
                }
            ))
            .count(),
        0
    );
    // Wait for connection loss #2
    on_connection_loss.select_next_some().await;

    is_reconnect.store(true, Ordering::Relaxed);
    // Second reconnect (is_reconnect = true) -> on_reconnected MUST fire
    Rc::clone(&session).reconnect().await.unwrap();
    on_reconnected.select_next_some().await;

    // Exactly one SynchronizeMe sent after `is_reconnect = true` reconnect
    assert_eq!(
        commands_sent
            .borrow()
            .iter()
            .filter(|m| matches!(
                m,
                ClientMsg::Command {
                    command: Command::SynchronizeMe { .. },
                    ..
                }
            ))
            .count(),
        1
    );
}
