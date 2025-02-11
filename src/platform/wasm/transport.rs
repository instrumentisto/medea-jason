//! [WebSocket] transport wrapper.
//!
//! [WebSocket]: https://developer.mozilla.org/ru/docs/WebSockets

// TODO: Needs refactoring.
#![expect(clippy::unwrap_used, reason = "needs refactoring")]

use std::{cell::RefCell, rc::Rc};

use async_trait::async_trait;
use derive_more::with_trait::{From, Into};
use futures::{channel::mpsc, stream::LocalBoxStream, StreamExt as _};
use medea_client_api_proto::{ClientMsg, ServerMsg};
use medea_reactive::ObservableCell;
use tracerr::Traced;
use web_sys::{CloseEvent, Event, MessageEvent, WebSocket as SysWebSocket};

use crate::{
    platform::{
        transport::{RpcTransport, TransportError, TransportState},
        wasm::utils::EventListener,
    },
    rpc::{websocket::ClientDisconnect, ApiUrl, CloseMsg},
};

/// Wrapper for help to get [`ServerMsg`] from Websocket [MessageEvent][1].
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent
#[derive(Clone, From, Into)]
struct ServerMessage(ServerMsg);

impl TryFrom<&MessageEvent> for ServerMessage {
    type Error = TransportError;

    fn try_from(msg: &MessageEvent) -> Result<Self, Self::Error> {
        use TransportError::{MessageNotString, ParseServerMessage};

        let payload = msg.data().as_string().ok_or(MessageNotString)?;

        serde_json::from_str::<ServerMsg>(&payload)
            .map_err(|e| ParseServerMessage(e.into()))
            .map(Self::from)
    }
}

/// Shortcut for a [`Result`] containing a [`Traced`] [`TransportError`].
type TransportResult<T> = Result<T, Traced<TransportError>>;

/// Inner data of a [`WebSocketRpcTransport`].
#[derive(Debug)]
struct InnerSocket {
    /// JS side [WebSocket].
    ///
    /// If [`SysWebSocket`] is [`None`], then connection hasn't been
    /// instantiated yet.
    ///
    /// [WebSocket]: https://developer.mozilla.org/docs/Web/API/WebSocket
    socket: RefCell<Option<SysWebSocket>>,

    /// State of [`WebSocketRpcTransport`] connection.
    socket_state: ObservableCell<TransportState>,

    /// Listener for [WebSocket] [open event][1].
    ///
    /// [WebSocket]: https://developer.mozilla.org/docs/Web/API/WebSocket
    /// [1]: https://developer.mozilla.org/en-US/Web/API/WebSocket/open_event
    on_open_listener: Option<EventListener<SysWebSocket, Event>>,

    /// Listener for [WebSocket] [message event][1].
    ///
    /// [WebSocket]: https://developer.mozilla.org/docs/Web/API/WebSocket
    /// [1]: https://developer.mozilla.org/docs/Web/API/WebSocket/message_event
    on_message_listener: Option<EventListener<SysWebSocket, MessageEvent>>,

    /// Listener for [WebSocket] [close event][1].
    ///
    /// [WebSocket]: https://developer.mozilla.org/docs/Web/API/WebSocket
    /// [1]: https://developer.mozilla.org/docs/Web/API/WebSocket/close_event
    on_close_listener: Option<EventListener<SysWebSocket, CloseEvent>>,

    /// Subscribers for [`RpcTransport::on_message`] events.
    on_message_subs: Vec<mpsc::UnboundedSender<ServerMsg>>,

    /// Reason of [`WebSocketRpcTransport`] closing.
    /// Will be sent in [WebSocket close frame][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc6455#section-5.5.1
    close_reason: ClientDisconnect,
}

impl InnerSocket {
    /// Creates a new [`InnerSocket`] which can be connected to the server with
    /// the [`RpcTransport::connect()`] method call.
    const fn new() -> Self {
        Self {
            socket_state: ObservableCell::new(TransportState::Connecting),
            socket: RefCell::new(None),
            on_open_listener: None,
            on_message_listener: None,
            on_close_listener: None,
            on_message_subs: Vec::new(),
            close_reason: ClientDisconnect::RpcTransportUnexpectedlyDropped,
        }
    }
}

impl Drop for InnerSocket {
    fn drop(&mut self) {
        if self.socket_state.borrow().can_close() {
            let rsn =
                serde_json::to_string(&self.close_reason).unwrap_or_else(|e| {
                    panic!("Could not serialize close message: {e}")
                });
            if let Some(socket) = self.socket.borrow().as_ref() {
                if let Err(e) = socket.close_with_code_and_reason(1000, &rsn) {
                    log::error!("Failed to normally close socket: {e:?}");
                }
            }
        }
    }
}

/// WebSocket [`RpcTransport`] between a client and a server.
///
/// # Drop
///
/// This structure has __cyclic references__, which are freed in its [`Drop`]
/// implementation.
///
/// If you're adding new cyclic dependencies, then don't forget to drop them in
/// the [`Drop`].
#[derive(Debug)]
pub struct WebSocketRpcTransport(Rc<RefCell<InnerSocket>>);

impl WebSocketRpcTransport {
    /// Returns a new [`WebSocketRpcTransport`] which can be connected to the
    /// server with the [`RpcTransport::connect()`] method call.
    #[must_use]
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(InnerSocket::new())))
    }

    /// Sets [`InnerSocket::on_close_listener`] which will update
    /// [`RpcTransport`]'s [`TransportState`] to [`TransportState::Closed`].
    fn set_on_close_listener(&self, socket: SysWebSocket) {
        let this = Rc::clone(&self.0);
        let on_close = EventListener::new_once(
            Rc::new(socket),
            "close",
            move |msg: CloseEvent| {
                this.borrow().socket_state.set(TransportState::Closed(
                    CloseMsg::from((msg.code(), msg.reason())),
                ));
            },
        )
        .unwrap();
        self.0.borrow_mut().on_close_listener = Some(on_close);
    }

    /// Sets [`InnerSocket::on_message_listener`] which will send
    /// [`ServerMessage`]s to [`WebSocketRpcTransport::on_message`] subscribers.
    fn set_on_message_listener(&self, socket: SysWebSocket) {
        let this = Rc::clone(&self.0);
        let on_message =
            EventListener::new_mut(Rc::new(socket), "message", move |msg| {
                let msg =
                    match ServerMessage::try_from(&msg).map(ServerMsg::from) {
                        Ok(parsed) => parsed,
                        Err(e) => {
                            // TODO: Protocol versions mismatch? Should drop
                            //       connection if so.
                            log::error!("{}", tracerr::new!(e));
                            return;
                        }
                    };

                let mut this_mut = this.borrow_mut();
                this_mut.on_message_subs.retain(|on_message| {
                    on_message.unbounded_send(msg.clone()).is_ok()
                });
            })
            .unwrap();

        self.0.borrow_mut().on_message_listener = Some(on_message);
    }
}

impl Default for WebSocketRpcTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl RpcTransport for WebSocketRpcTransport {
    async fn connect(&self, url: ApiUrl) -> TransportResult<()> {
        let socket = SysWebSocket::new(url.as_ref())
            .map_err(Into::into)
            .map_err(TransportError::CreateSocket)
            .map_err(tracerr::wrap!())?;
        *self.0.borrow_mut().socket.borrow_mut() = Some(socket.clone());
        {
            {
                let inner = Rc::clone(&self.0);
                self.0.borrow_mut().on_close_listener = Some(
                    EventListener::new_once(
                        Rc::clone(&Rc::new(socket.clone())),
                        "close",
                        move |msg: CloseEvent| {
                            inner.borrow().socket_state.set(
                                TransportState::Closed(CloseMsg::from((
                                    msg.code(),
                                    msg.reason(),
                                ))),
                            );
                        },
                    )
                    .unwrap(),
                );
            }

            {
                let inner = Rc::clone(&self.0);
                self.0.borrow_mut().on_open_listener = Some(
                    EventListener::new_once(
                        Rc::clone(&Rc::new(socket.clone())),
                        "open",
                        move |_| {
                            inner
                                .borrow()
                                .socket_state
                                .set(TransportState::Open);
                        },
                    )
                    .unwrap(),
                );
            }
        }

        let state_updates_rx = self.0.borrow().socket_state.subscribe();
        let state = state_updates_rx.skip(1).next().await;

        if state == Some(TransportState::Open) {
            self.set_on_close_listener(socket.clone());
            self.set_on_message_listener(socket);
            Ok(())
        } else {
            Err(tracerr::new!(TransportError::InitSocket))
        }
    }

    fn on_message(&self) -> LocalBoxStream<'static, ServerMsg> {
        let (tx, rx) = mpsc::unbounded();
        self.0.borrow_mut().on_message_subs.push(tx);

        Box::pin(rx)
    }

    fn set_close_reason(&self, reason: ClientDisconnect) {
        self.0.borrow_mut().close_reason = reason;
    }

    fn send(&self, msg: &ClientMsg) -> TransportResult<()> {
        let inner = self.0.borrow();
        let message = serde_json::to_string(msg)
            .map_err(|e| TransportError::SerializeClientMessage(e.into()))
            .map_err(tracerr::wrap!())?;

        let state = &*inner.socket_state.borrow();
        match state {
            TransportState::Open => inner.socket.borrow().as_ref().map_or_else(
                || Err(tracerr::new!(TransportError::ClosedSocket)),
                |socket| {
                    socket
                        .send_with_str(&message)
                        .map_err(Into::into)
                        .map_err(TransportError::SendMessage)
                        .map_err(tracerr::wrap!())
                },
            ),
            TransportState::Connecting
            | TransportState::Closing
            | TransportState::Closed(_) => {
                Err(tracerr::new!(TransportError::ClosedSocket))
            }
        }
    }

    fn on_state_change(&self) -> LocalBoxStream<'static, TransportState> {
        self.0.borrow().socket_state.subscribe()
    }
}

impl Drop for WebSocketRpcTransport {
    /// Don't forget that [`WebSocketRpcTransport`] is a [`Rc`] and this
    /// [`Drop`] implementation will be called on each drop of its references.
    fn drop(&mut self) {
        let mut inner = self.0.borrow_mut();
        drop(inner.on_open_listener.take());
        drop(inner.on_message_listener.take());
        drop(inner.on_close_listener.take());
    }
}
