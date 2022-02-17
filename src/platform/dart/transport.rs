use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use futures::{channel::mpsc, prelude::stream::LocalBoxStream};
use medea_client_api_proto::{ClientMsg, CloseReason, ServerMsg};
use medea_macro::dart_bridge;
use medea_reactive::ObservableCell;
use tracerr::Traced;

use crate::{
    api::string_into_c_str,
    platform::{
        dart::utils::{
            callback::Callback, dart_future::FutureFromDart, handle::DartHandle,
        },
        RpcTransport, TransportError, TransportState,
    },
    rpc::{ApiUrl, ClientDisconnect, CloseMsg},
};

type Result<T, E = Traced<TransportError>> = std::result::Result<T, E>;

#[dart_bridge("flutter/lib/src/native/platform/transport.g.dart")]
mod transport {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    extern "C" {
        /// [Connects][1] to the provided `url` and returns the created
        /// [`WebSocket`][0].
        ///
        /// [Subscribes][2] to the created [`WebSocket`][0] passing the given
        /// `on_message` and `on_close` callbacks.
        ///
        /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
        /// [1]: https://api.dart.dev/stable/dart-io/WebSocket/connect.html
        /// [2]: https://api.dart.dev/stable/dart-async/Stream/listen.html
        pub fn connect(
            url: ptr::NonNull<c_char>,
            on_message: Dart_Handle,
            on_close: Dart_Handle,
        ) -> Dart_Handle;

        /// [Sends][1] the provided `message` via the provided [`WebSocket`][0].
        ///
        /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
        /// [1]: https://api.dart.dev/stable/dart-io/WebSocket/add.html
        pub fn send(transport: Dart_Handle, message: ptr::NonNull<c_char>);

        /// [Closes][1] the provided [`WebSocket`][0] connection.
        ///
        /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
        /// [1]: https://api.dart.dev/stable/dart-io/WebSocket/close.html
        pub fn close(
            transport: Dart_Handle,
            close_code: i32,
            close_msg: ptr::NonNull<c_char>,
        );
    }
}

/// [`RpcTransport`] implementation of a Dart side [`WebSocket`][0].
///
/// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
#[derive(Clone, Debug)]
pub struct WebSocketRpcTransport {
    /// Handle to the Dart side [`WebSocket`][0].
    ///
    /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
    handle: DartHandle,

    /// Subscribers to the messages received by this transport.
    on_message_subs: Rc<RefCell<Vec<mpsc::UnboundedSender<ServerMsg>>>>,

    /// Reason of [`WebSocketRpcTransport`] closing.
    ///
    /// Is sent in a [WebSocket close frame][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc6455#section-5.5.1
    close_reason: Cell<ClientDisconnect>,

    /// State of this [`WebSocketRpcTransport`] connection.
    socket_state: Rc<ObservableCell<TransportState>>,
}

impl WebSocketRpcTransport {
    /// Initiates a new [`WebSocketRpcTransport`] connection.
    ///
    /// Only resolves once the underlying connection becomes active.
    ///
    /// # Errors
    ///
    /// With [`TransportError::CreateSocket`] if cannot establish
    /// [`WebSocket`][0] to the specified `url`.
    ///
    /// With [`TransportError::InitSocket`] if [WebSocket.onclose][1] callback
    /// fired before [WebSocket.onopen][2] callback.
    ///
    /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
    /// [1]: https://developer.mozilla.org/docs/Web/API/WebSocket/onclose
    /// [2]: https://developer.mozilla.org/docs/Web/API/WebSocket/onopen
    pub async fn new(url: ApiUrl) -> Result<Self> {
        unsafe {
            let on_message_subs = Rc::new(RefCell::new(Vec::new()));
            let socket_state =
                Rc::new(ObservableCell::new(TransportState::Open));

            // TODO: Propagate execution error.
            #[allow(clippy::map_err_ignore)]
            let handle =
                FutureFromDart::execute::<DartHandle>(
                    transport::connect(
                        string_into_c_str(url.as_ref().to_owned()),
                        Callback::from_fn_mut({
                            let weak_subs = Rc::downgrade(&on_message_subs);
                            move |msg: String| {
                                if let Some(subs) = weak_subs.upgrade() {
                                    let msg = match serde_json::from_str::<
                                        ServerMsg,
                                    >(
                                        &msg
                                    ) {
                                        Ok(parsed) => parsed,
                                        Err(e) => {
                                            // TODO: Protocol versions mismatch?
                                            //       should drop connection if
                                            // so.
                                            log::error!("{}", tracerr::new!(e));
                                            return;
                                        }
                                    };

                                    subs.borrow_mut().retain(
                                    |sub: &mpsc::UnboundedSender<ServerMsg>| {
                                        sub.unbounded_send(msg.clone()).is_ok()
                                    },
                                );
                                }
                            }
                        })
                        .into_dart(),
                        Callback::from_once({
                            let socket_state = Rc::clone(&socket_state);
                            move |_: ()| {
                                socket_state.set(TransportState::Closed(
                                    CloseMsg::Normal(
                                        1000,
                                        CloseReason::Finished,
                                    ),
                                ));
                            }
                        })
                        .into_dart(),
                    ),
                )
                .await
                .map_err(|_| tracerr::new!(TransportError::InitSocket))?;

            Ok(Self {
                handle,
                on_message_subs,
                socket_state,
                close_reason: Cell::new(
                    ClientDisconnect::RpcTransportUnexpectedlyDropped,
                ),
            })
        }
    }
}

impl RpcTransport for WebSocketRpcTransport {
    fn on_message(&self) -> LocalBoxStream<'static, ServerMsg> {
        let (tx, rx) = mpsc::unbounded();
        self.on_message_subs.borrow_mut().push(tx);
        Box::pin(rx)
    }

    fn set_close_reason(&self, reason: ClientDisconnect) {
        self.close_reason.set(reason);
    }

    #[allow(clippy::unwrap_in_result)]
    fn send(&self, msg: &ClientMsg) -> Result<(), Traced<TransportError>> {
        let msg = serde_json::to_string(msg).unwrap();
        unsafe {
            transport::send(self.handle.get(), string_into_c_str(msg));
        }
        Ok(())
    }

    fn on_state_change(&self) -> LocalBoxStream<'static, TransportState> {
        self.socket_state.subscribe()
    }
}

impl Drop for WebSocketRpcTransport {
    fn drop(&mut self) {
        let rsn = serde_json::to_string(&self.close_reason.get())
            .expect("Could not serialize close message");
        unsafe {
            transport::close(self.handle.get(), 1000, string_into_c_str(rsn));
        }
    }
}
