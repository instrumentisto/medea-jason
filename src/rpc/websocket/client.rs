//! [WebSocket] client.
//!
//! [WebSocket]: https://developer.mozilla.org/ru/docs/WebSockets

use std::{cell::RefCell, rc::Rc, time::Duration};

use derive_more::with_trait::{Debug, Display};
use futures::{
    channel::{mpsc, oneshot},
    future::LocalBoxFuture,
    stream::{LocalBoxStream, StreamExt as _},
};
use medea_client_api_proto::{
    Capabilities, ClientMsg, CloseReason as CloseByServerReason, Command,
    Credential, Event, MemberId, RoomId, RpcSettings, ServerMsg,
};
use medea_macro::dispatchable;
use medea_reactive::ObservableCell;
use serde::Serialize;
use tracerr::Traced;

use crate::{
    platform,
    rpc::{
        ApiUrl, CloseMsg, CloseReason, ClosedStateReason, ConnectionLostReason,
        Heartbeat, IdleTimeout, PingInterval, RpcClientError,
    },
};

/// Reasons of closing WebSocket RPC connection by a client side.
#[derive(Copy, Clone, Display, Debug, Eq, PartialEq, Serialize)]
pub enum ClientDisconnect {
    /// [`Room`] was dropped without any [`CloseReason`].
    ///
    /// [`Room`]: crate::room::Room
    RoomUnexpectedlyDropped,

    /// [`Room`] was normally closed by client.
    ///
    /// [`Room`]: crate::room::Room
    RoomClosed,

    /// [`WebSocketRpcClient`] was unexpectedly dropped.
    RpcClientUnexpectedlyDropped,

    /// [`platform::RpcTransport`] was unexpectedly dropped.
    RpcTransportUnexpectedlyDropped,

    /// [`WebSocketRpcSession`] was unexpectedly dropped.
    ///
    /// [`WebSocketRpcSession`]: crate::rpc::WebSocketRpcSession
    SessionUnexpectedlyDropped,

    /// Client initiated reconnection for whatever reason.
    CloseForReconnection,
}

impl ClientDisconnect {
    /// Returns a close code for this [`ClientDisconnect`] reason.
    #[must_use]
    pub const fn code(self) -> u16 {
        match self {
            Self::RoomClosed
            | Self::RoomUnexpectedlyDropped
            | Self::RpcClientUnexpectedlyDropped
            | Self::RpcTransportUnexpectedlyDropped
            | Self::SessionUnexpectedlyDropped => 1000,
            Self::CloseForReconnection => {
                // Only 1000 or [3000; 4999] can be used, with 1000 as a normal
                // close, and everything else is protocol-defined, which is
                // abnormal in our case.
                3000
            }
        }
    }
}

impl From<ClientDisconnect> for CloseReason {
    fn from(value: ClientDisconnect) -> Self {
        Self::ByClient { reason: value }
    }
}

/// State of a [`WebSocketRpcClient`] and a [`platform::RpcTransport`].
#[derive(Clone, Debug, PartialEq)]
pub enum ClientState {
    /// [`WebSocketRpcClient`] is currently establishing a connection to RPC
    /// server.
    Connecting,

    /// Connection with RPC Server is active.
    Open,

    /// Connection with RPC server is currently closed.
    Closed(ClosedStateReason),
}

/// Inner state of [`WebSocketRpcClient`].
#[derive(Debug)]
struct Inner {
    /// Transport connection with remote media server.
    #[debug(skip)]
    sock: Option<Rc<dyn platform::RpcTransport>>,

    /// Connection loss detector via ping/pong mechanism.
    heartbeat: Option<Heartbeat>,

    /// Event's subscribers list.
    subs: Vec<mpsc::UnboundedSender<RpcEvent>>,

    /// Subscribers that will be notified with [`CloseReason`] when underlying
    /// transport is gracefully closed.
    on_close_subscribers: Vec<oneshot::Sender<CloseReason>>,

    /// Reason of [`WebSocketRpcClient`] closing.
    ///
    /// This reason will be provided to the underlying
    /// [`platform::RpcTransport`].
    close_reason: ClientDisconnect,

    /// Subscribers that will be notified when underlying transport connection
    /// is lost.
    on_connection_loss_subs: Vec<mpsc::UnboundedSender<ConnectionLostReason>>,

    /// Closure which will create new [`platform::RpcTransport`]s for this
    /// [`WebSocketRpcClient`] on each
    /// [`WebSocketRpcClient:: establish_connection`] call.
    #[debug("{rpc_transport_factory:p}")]
    rpc_transport_factory: RpcTransportFactory,

    /// URL that [`platform::RpcTransport`] will connect to.
    ///
    /// [`None`] if this [`WebSocketRpcClient`] has never been connected to
    /// a sever.
    url: Option<ApiUrl>,

    /// Current [`ClientState`] of this [`WebSocketRpcClient`].
    state: ObservableCell<ClientState>,
}

/// Factory closure producing a [`platform::RpcTransport`].
pub type RpcTransportFactory = Box<dyn Fn() -> Rc<dyn platform::RpcTransport>>;

impl Inner {
    /// Instantiates new [`Inner`] state of [`WebSocketRpcClient`].
    fn new(rpc_transport_factory: RpcTransportFactory) -> RefCell<Self> {
        RefCell::new(Self {
            sock: None,
            on_close_subscribers: Vec::new(),
            subs: Vec::new(),
            heartbeat: None,
            close_reason: ClientDisconnect::RpcClientUnexpectedlyDropped,
            on_connection_loss_subs: Vec::new(),
            rpc_transport_factory,
            url: None,
            state: ObservableCell::new(ClientState::Closed(
                ClosedStateReason::NeverConnected,
            )),
        })
    }
}

/// Events which can be thrown by [`WebSocketRpcClient`].
#[dispatchable(self: &Self)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RpcEvent {
    /// Notification of the subscribers that [`WebSocketRpcClient`] is joined
    /// [`Room`] on Media Server.
    ///
    /// [`Room`]: crate::room::Room
    JoinedRoom {
        /// ID of the joined [`Room`].
        ///
        /// [`Room`]: crate::room::Room
        room_id: RoomId,

        /// ID of the joined `Member`.
        member_id: MemberId,

        /// Indicator whether this join is a reconnect.
        is_reconnect: bool,
    },

    /// Notification of the subscribers that [`WebSocketRpcClient`] left
    /// [`Room`] on Media Server.
    ///
    /// [`Room`]: crate::room::Room
    LeftRoom {
        /// ID of the [`Room`] being left.
        ///
        /// [`Room`]: crate::room::Room
        room_id: RoomId,

        /// Reason of why the [`Room`] has been left.
        ///
        /// [`Room`]: crate::room::Room
        close_reason: CloseReason,
    },

    /// [`WebSocketRpcClient`] received [`Event`] from Media Server.
    Event {
        /// ID of the [`Room`] for that this [`Event`] has been received for.
        ///
        /// [`Room`]: crate::room::Room
        room_id: RoomId,

        /// Received [`Event`].
        event: Event,
    },
}

/// Client API RPC client to talk with server via [WebSocket].
///
/// [WebSocket]: https://developer.mozilla.org/ru/docs/WebSockets
#[derive(Debug)]
pub struct WebSocketRpcClient(RefCell<Inner>);

impl WebSocketRpcClient {
    /// Creates new [`WebSocketRpcClient`] with provided [`RpcTransportFactory`]
    /// closure.
    #[must_use]
    pub fn new(rpc_transport_factory: RpcTransportFactory) -> Self {
        Self(Inner::new(rpc_transport_factory))
    }

    /// Authorizes [`WebSocketRpcClient`] on the Media Server.
    pub fn join_room(
        &self,
        room_id: RoomId,
        member_id: MemberId,
        credential: Credential,
        capabilities: Capabilities,
    ) {
        self.send_command(
            room_id,
            Command::JoinRoom { member_id, credential, capabilities },
        );
    }

    /// Leaves `Room` with a provided [`RoomId`].
    pub fn leave_room(&self, room_id: RoomId, member_id: MemberId) {
        self.send_command(room_id, Command::LeaveRoom { member_id });
    }

    /// Stops [`Heartbeat`] and notifies all
    /// [`WebSocketRpcClient::on_connection_loss`] subs about connection
    /// loss.
    fn handle_connection_loss(&self, close_msg: ConnectionLostReason) {
        self.0.borrow().state.set(ClientState::Closed(
            ClosedStateReason::ConnectionLost(close_msg),
        ));
        drop(self.0.borrow_mut().heartbeat.take());
        self.0
            .borrow_mut()
            .on_connection_loss_subs
            .retain(|sub| sub.unbounded_send(close_msg).is_ok());
    }

    /// Handles [`CloseMsg`] from a remote server.
    ///
    /// This function will be called on every WebSocket close (normal and
    /// abnormal) regardless of the [`CloseReason`].
    fn handle_close_message(&self, close_msg: CloseMsg) {
        drop(self.0.borrow_mut().heartbeat.take());

        match close_msg {
            CloseMsg::Normal(_, reason) => match reason {
                CloseByServerReason::Reconnected => (),
                CloseByServerReason::Idle => {
                    self.handle_connection_loss(ConnectionLostReason::Idle);
                }
                CloseByServerReason::Finished
                | CloseByServerReason::Rejected
                | CloseByServerReason::InternalError
                | CloseByServerReason::Evicted => {
                    self.0.borrow().state.set(ClientState::Closed(
                        ClosedStateReason::ConnectionLost(
                            ConnectionLostReason::WithMessage(close_msg),
                        ),
                    ));
                    drop(self.0.borrow_mut().sock.take());
                    self.0
                        .borrow_mut()
                        .on_close_subscribers
                        .drain(..)
                        .for_each(|sub| {
                            _ = sub.send(CloseReason::ByServer(reason));
                        });
                }
            },
            CloseMsg::Abnormal(_) => {
                if self.0.borrow_mut().close_reason
                    == ClientDisconnect::CloseForReconnection
                {
                    // Client-side initiated reconnect. Not checking the actual
                    // close code since it can't known whether the server was
                    // able to handle out close frame and send it back or the
                    // connection was broken, so any non-1000 code is OK here.
                    return;
                }
                self.handle_connection_loss(ConnectionLostReason::WithMessage(
                    close_msg,
                ));
            }
        }
    }

    /// Handles [`ServerMsg`]s from a remote server.
    fn on_transport_message(&self, msg: ServerMsg) {
        let msg = match msg {
            ServerMsg::Event { room_id, event } => match event {
                Event::RoomJoined { member_id, is_reconnect } => {
                    Some(RpcEvent::JoinedRoom {
                        room_id,
                        member_id,
                        is_reconnect,
                    })
                }
                Event::RoomLeft { close_reason } => Some(RpcEvent::LeftRoom {
                    room_id,
                    close_reason: CloseReason::ByServer(close_reason),
                }),
                Event::PeerCreated { .. }
                | Event::SdpAnswerMade { .. }
                | Event::LocalDescriptionApplied { .. }
                | Event::IceCandidateDiscovered { .. }
                | Event::PeersRemoved { .. }
                | Event::PeerUpdated { .. }
                | Event::ConnectionQualityUpdated { .. }
                | Event::StateSynchronized { .. } => {
                    Some(RpcEvent::Event { room_id, event })
                }
            },
            ServerMsg::RpcSettings(settings) => {
                self.0.borrow_mut().heartbeat.as_ref().map_or_else(
                    || {
                        log::error!(
                            "Failed to update socket settings because \
                             Heartbeat is None",
                        );
                    },
                    |heartbeat| {
                        heartbeat.update_settings(
                            IdleTimeout(Duration::from_millis(
                                settings.idle_timeout_ms.into(),
                            )),
                            PingInterval(Duration::from_millis(
                                settings.ping_interval_ms.into(),
                            )),
                        );
                    },
                );
                None
            }
            ServerMsg::Ping(_) => None,
        };
        if let Some(m) = msg {
            self.0
                .borrow_mut()
                .subs
                .retain(|sub| sub.unbounded_send(m.clone()).is_ok());
        }
    }

    /// Starts [`Heartbeat`] with provided [`RpcSettings`] for provided
    /// [`platform::RpcTransport`].
    fn start_heartbeat(
        self: Rc<Self>,
        transport: Rc<dyn platform::RpcTransport>,
        rpc_settings: RpcSettings,
    ) {
        let idle_timeout = IdleTimeout(Duration::from_millis(
            rpc_settings.idle_timeout_ms.into(),
        ));
        let ping_interval = PingInterval(Duration::from_millis(
            rpc_settings.ping_interval_ms.into(),
        ));

        let heartbeat =
            Heartbeat::start(transport, ping_interval, idle_timeout);

        let mut on_idle = heartbeat.on_idle();
        let weak_this = Rc::downgrade(&self);
        platform::spawn(async move {
            while on_idle.next().await.is_some() {
                if let Some(this) = weak_this.upgrade() {
                    this.handle_connection_loss(ConnectionLostReason::Idle);
                }
            }
        });
        self.0.borrow_mut().heartbeat = Some(heartbeat);
    }

    /// Tries to establish [`WebSocketRpcClient`] connection.
    async fn establish_connection(
        self: Rc<Self>,
        url: ApiUrl,
    ) -> Result<(), Traced<RpcClientError>> {
        self.0.borrow_mut().url = Some(url.clone());
        self.0.borrow().state.set(ClientState::Connecting);

        // Wait for transport opening.
        let transport = (self.0.borrow().rpc_transport_factory)();
        let mut on_message = transport.on_message();
        transport.connect(url).await.map_err(|e| {
            let transport_err = e.into_inner();
            self.0.borrow().state.set(ClientState::Closed(
                ClosedStateReason::CouldNotEstablish(transport_err.clone()),
            ));
            tracerr::new!(RpcClientError::from(
                ClosedStateReason::CouldNotEstablish(transport_err)
            ))
        })?;

        // Wait for `ServerMsg::RpcSettings`.
        if let Some(msg) = on_message.next().await {
            if let ServerMsg::RpcSettings(rpc_settings) = msg {
                Rc::clone(&self)
                    .start_heartbeat(Rc::clone(&transport), rpc_settings);
            } else {
                let close_reason =
                    ClosedStateReason::FirstServerMsgIsNotRpcSettings;
                self.0
                    .borrow()
                    .state
                    .set(ClientState::Closed(close_reason.clone()));
                return Err(tracerr::new!(RpcClientError::ConnectionFailed(
                    close_reason
                )));
            }
        } else {
            self.0.borrow().state.set(ClientState::Closed(
                ClosedStateReason::FirstServerMsgIsNotRpcSettings,
            ));
            return Err(tracerr::new!(RpcClientError::ConnectionFailed(
                ClosedStateReason::FirstServerMsgIsNotRpcSettings
            )));
        }

        // Subscribe to transport closing.
        {
            let mut transport_state_changes = transport.on_state_change();
            let weak_this = Rc::downgrade(&self);
            platform::spawn(async move {
                while let Some(state) = transport_state_changes.next().await {
                    if let Some(this) = weak_this.upgrade() {
                        if let platform::TransportState::Closed(msg) = state {
                            this.handle_close_message(msg);
                        }
                    }
                }
            });
        }

        // Subscribe to transport message receiving.
        {
            let weak_this = Rc::downgrade(&self);
            let mut on_socket_message = transport.on_message();
            platform::spawn(async move {
                while let Some(msg) = on_socket_message.next().await {
                    if let Some(this) = weak_this.upgrade() {
                        this.on_transport_message(msg);
                    }
                }
            });
        }

        drop(self.0.borrow_mut().sock.replace(transport));
        self.0.borrow().state.set(ClientState::Open);

        Ok(())
    }

    /// Subscribes to [`WebSocketRpcClient`]'s [`ClientState`] changes and when
    /// [`ClientState::Connecting`] will be changed to something else, then this
    /// [`Future`] will be resolved and based on new [`ClientState`] [`Result`]
    /// will be returned.
    async fn connecting_result(&self) -> Result<(), Traced<RpcClientError>> {
        let mut state_changes = self.0.borrow().state.subscribe();
        while let Some(state) = state_changes.next().await {
            match state {
                ClientState::Open => {
                    return Ok(());
                }
                ClientState::Closed(
                    ClosedStateReason::ClosedForReconnection,
                )
                | ClientState::Connecting => (),
                ClientState::Closed(reason) => {
                    return Err(tracerr::new!(
                        RpcClientError::ConnectionFailed(reason)
                    ));
                }
            }
        }
        Err(tracerr::new!(RpcClientError::RpcClientGone))
    }

    /// Tries to upgrade [`ClientState`] of this [`WebSocketRpcClient`] to
    /// [`ClientState::Open`].
    ///
    /// This function is also used for reconnecting this [`WebSocketRpcClient`].
    ///
    /// If [`WebSocketRpcClient`] is closed than this function will try to
    /// establish new RPC connection.
    ///
    /// If [`WebSocketRpcClient`] already in [`ClientState::Connecting`] then
    /// this function will not perform one more connection try. It will
    /// subscribe to [`ClientState`] changes and wait for first connection
    /// result, and, based on this result, this function will be resolved.
    ///
    /// If [`WebSocketRpcClient`] already in [`ClientState::Open`] then this
    /// function will be instantly resolved.
    ///
    /// # Errors
    ///
    /// Errors if [`WebSocketRpcClient`] fails to establish connection with a
    /// server.
    pub async fn connect(
        self: Rc<Self>,
        url: ApiUrl,
    ) -> Result<(), Traced<RpcClientError>> {
        let current_url = self.0.borrow().url.clone();
        if current_url.as_ref() == Some(&url) {
            let state = self.0.borrow().state.borrow().clone();
            match state {
                ClientState::Open => Ok(()),
                ClientState::Connecting => self.connecting_result().await,
                ClientState::Closed(_) => self.establish_connection(url).await,
            }
        } else {
            self.establish_connection(url).await
        }
    }

    /// Subscribes on this [`WebSocketRpcClient`]'s [`RpcEvent`]s.
    pub fn subscribe(&self) -> LocalBoxStream<'static, RpcEvent> {
        let (tx, rx) = mpsc::unbounded();
        self.0.borrow_mut().subs.push(tx);

        Box::pin(rx)
    }

    /// Sends [`Command`] for the provided [`RoomId`] to server.
    pub fn send_command(&self, room_id: RoomId, command: Command) {
        let socket_borrow = &self.0.borrow().sock;

        if let Some(socket) = socket_borrow.as_ref() {
            if let Err(e) = socket
                .send(&ClientMsg::Command { room_id, command })
                .map_err(tracerr::map_from_and_wrap!(=> RpcClientError))
            {
                log::error!("{e}");
            }
        }
    }

    /// [`Future`] resolving on normal [`WebSocketRpcClient`] connection
    /// closing.
    ///
    /// This [`Future`] wouldn't be resolved on abnormal closes.
    /// An [`WebSocketRpcClient::on_connection_loss`] will be thrown instead.
    pub fn on_normal_close(
        &self,
    ) -> LocalBoxFuture<'static, Result<CloseReason, oneshot::Canceled>> {
        let (tx, rx) = oneshot::channel();
        self.0.borrow_mut().on_close_subscribers.push(tx);
        Box::pin(rx)
    }

    /// Subscribe to connection loss events.
    ///
    /// Connection loss is any unexpected [`platform::RpcTransport`] close. In
    /// case of connection loss, client side user should select reconnection
    /// strategy with [`ReconnectHandle`] (or simply close [`Room`]).
    ///
    /// [`ReconnectHandle`]: crate::rpc::ReconnectHandleImpl
    /// [`Room`]: crate::room::Room
    /// [`Stream`]: futures::Stream
    pub fn on_connection_loss(
        &self,
    ) -> LocalBoxStream<'static, ConnectionLostReason> {
        let (tx, rx) = mpsc::unbounded();
        self.0.borrow_mut().on_connection_loss_subs.push(tx);
        Box::pin(rx)
    }

    /// Forces the underlying transport to close immediately.
    ///
    /// This triggers normal or abnormal close handling and propagates to
    /// session watchers.
    pub fn close_for_reconnection(&self) {
        drop(self.0.borrow_mut().heartbeat.take());
        self.set_close_reason(ClientDisconnect::CloseForReconnection);
        if let Some(sock) = self.0.borrow_mut().sock.take() {
            sock.set_close_reason(ClientDisconnect::CloseForReconnection);
        }
        self.0.borrow().state.set(ClientState::Closed(
            ClosedStateReason::ConnectionLost(
                ConnectionLostReason::WithMessage(CloseMsg::Normal(
                    1000,
                    CloseByServerReason::Reconnected,
                )),
            ),
        ));
    }

    /// Sets reason being passed to the underlying transport when this client is
    /// dropped.
    pub fn set_close_reason(&self, close_reason: ClientDisconnect) {
        self.0.borrow_mut().close_reason = close_reason;
    }
}

impl Drop for Inner {
    /// Drops the related connection and its [`Heartbeat`].
    fn drop(&mut self) {
        if let Some(socket) = self.sock.take() {
            socket.set_close_reason(self.close_reason);
        }
    }
}
