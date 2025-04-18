use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use async_trait::async_trait;
use futures::{channel::mpsc, prelude::stream::LocalBoxStream};
use medea_client_api_proto::{ClientMsg, ServerMsg};
use medea_macro::dart_bridge;
use medea_reactive::ObservableCell;
use tracerr::Traced;

use crate::{
    platform::{
        RpcTransport, TransportError, TransportState,
        dart::utils::{
            callback::Callback, dart_future::FutureFromDart,
            dart_string_into_rust, handle::DartHandle, string_into_c_str,
        },
    },
    rpc::{ApiUrl, ClientDisconnect, CloseMsg},
};

type TransportResult<T> = Result<T, Traced<TransportError>>;

#[dart_bridge("flutter/lib/src/native/platform/transport.g.dart")]
mod transport {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::platform::Error;

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
        ) -> Result<Dart_Handle, Error>;

        /// [Sends][1] the provided `message` via the provided [`WebSocket`][0].
        ///
        /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
        /// [1]: https://api.dart.dev/stable/dart-io/WebSocket/add.html
        pub fn send(
            transport: Dart_Handle,
            message: ptr::NonNull<c_char>,
        ) -> Result<(), Error>;

        /// [Closes][1] the provided [`WebSocket`][0] connection.
        ///
        /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
        /// [1]: https://api.dart.dev/stable/dart-io/WebSocket/close.html
        pub fn close(
            transport: Dart_Handle,
            close_code: i32,
            close_msg: ptr::NonNull<c_char>,
        ) -> Result<(), Error>;

        /// Returns the [closeCode][0] of the [close frame][1].
        ///
        /// [0]: https://api.dart.dev/stable/dart-io/WebSocket/closeCode.html
        /// [1]: https://tools.ietf.org/html/rfc6455#section-5.5.1
        pub fn close_code(close_frame: Dart_Handle) -> Result<i32, Error>;

        /// Returns the [closeReason][0] of the [close frame][1].
        ///
        /// [0]: https://api.dart.dev/stable/dart-io/WebSocket/closeReason.html
        /// [1]: https://tools.ietf.org/html/rfc6455#section-5.5.1
        pub fn close_reason(
            close_frame: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;
    }
}

/// [`RpcTransport`] implementation of a Dart side [`WebSocket`][0].
///
/// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
#[derive(Clone, Debug)]
pub struct WebSocketRpcTransport {
    /// Handle to the Dart side [`WebSocket`][0].
    ///
    /// If [`DartHandle`] is [`None`], then connection hasn't been instantiated
    /// yet.
    ///
    /// [0]: https://api.dart.dev/stable/dart-io/WebSocket-class.html
    handle: RefCell<Option<DartHandle>>,

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
    /// Creates a new [`WebSocketRpcTransport`] which can be connected to the
    /// server with the [`RpcTransport::connect()`] method call.
    #[must_use]
    pub fn new() -> Self {
        Self {
            handle: RefCell::new(None),
            on_message_subs: Rc::new(RefCell::new(Vec::new())),
            socket_state: Rc::new(ObservableCell::new(
                TransportState::Connecting,
            )),
            close_reason: Cell::new(
                ClientDisconnect::RpcTransportUnexpectedlyDropped,
            ),
        }
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
        // TODO: Propagate execution error.
        #[expect(clippy::map_err_ignore, reason = "needs refactoring")]
        let handle = {
            let on_message = Callback::from_fn_mut({
                let weak_subs = Rc::downgrade(&self.on_message_subs);
                move |msg: String| {
                    if let Some(subs) = weak_subs.upgrade() {
                        let msg = match serde_json::from_str::<ServerMsg>(&msg)
                        {
                            Ok(parsed) => parsed,
                            Err(e) => {
                                // TODO: Protocol versions mismatch?
                                //       Should drop connection if so.
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
            .into_dart();
            let on_close = Callback::from_fn_mut({
                let socket_state = Rc::clone(&self.socket_state);
                move |close_frame: DartHandle| {
                    let code =
                        unsafe { transport::close_code(close_frame.get()) }
                            .unwrap()
                            .try_into()
                            .unwrap_or(1007);
                    let reason =
                        unsafe { transport::close_reason(close_frame.get()) }
                            .unwrap();
                    let reason = unsafe { dart_string_into_rust(reason) };

                    socket_state.set(TransportState::Closed(CloseMsg::from((
                        code, reason,
                    ))));
                }
            })
            .into_dart();

            let fut = unsafe {
                transport::connect(
                    string_into_c_str(url.as_ref().to_owned()),
                    on_message,
                    on_close,
                )
            }
            .unwrap();
            unsafe { FutureFromDart::execute::<DartHandle>(fut) }
        }
        .await
        .map_err(|_| tracerr::new!(TransportError::InitSocket))?;

        *self.handle.borrow_mut() = Some(handle);
        self.socket_state.set(TransportState::Open);

        Ok(())
    }

    fn on_message(&self) -> LocalBoxStream<'static, ServerMsg> {
        let (tx, rx) = mpsc::unbounded();
        self.on_message_subs.borrow_mut().push(tx);
        Box::pin(rx)
    }

    fn set_close_reason(&self, reason: ClientDisconnect) {
        self.close_reason.set(reason);
    }

    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    fn send(&self, msg: &ClientMsg) -> TransportResult<()> {
        let state = self.socket_state.get();
        let handle = self
            .handle
            .borrow()
            .as_ref()
            .cloned()
            .ok_or_else(|| tracerr::new!(TransportError::ClosedSocket))?;
        match state {
            TransportState::Open => unsafe {
                let msg = serde_json::to_string(msg).unwrap();
                transport::send(handle.get(), string_into_c_str(msg))
                    .map_err(TransportError::SendMessage)
                    .map_err(tracerr::wrap!())?;
                Ok(())
            },
            TransportState::Connecting
            | TransportState::Closing
            | TransportState::Closed(_) => {
                Err(tracerr::new!(TransportError::ClosedSocket))
            }
        }
    }

    fn on_state_change(&self) -> LocalBoxStream<'static, TransportState> {
        self.socket_state.subscribe()
    }
}

impl Drop for WebSocketRpcTransport {
    fn drop(&mut self) {
        let rsn = serde_json::to_string(&self.close_reason.get())
            .unwrap_or_else(|e| {
                panic!("Could not serialize close message: {e}")
            });
        if let Some(handle) = self.handle.borrow().as_ref() {
            unsafe {
                transport::close(handle.get(), 1000, string_into_c_str(rsn))
            }
            .unwrap();
        }
    }
}
