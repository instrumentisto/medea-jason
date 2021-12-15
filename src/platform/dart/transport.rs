use std::{
    cell::{Cell, RefCell},
    os::raw::c_char,
    ptr,
    rc::Rc,
};

use futures::{channel::mpsc, prelude::stream::LocalBoxStream};
use medea_client_api_proto::{ClientMsg, CloseReason, ServerMsg};
use medea_macro::dart_bridge;
use medea_reactive::ObservableCell;
use tracerr::Traced;

use crate::{
    api::{c_str_into_string, string_into_c_str},
    platform::{
        dart::utils::{callback::Callback, handle::DartHandle},
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
        pub fn init(url: ptr::NonNull<c_char>) -> Dart_Handle;

        pub fn send(transport: Dart_Handle, msg: ptr::NonNull<c_char>);

        pub fn close(
            transport: Dart_Handle,
            close_code: i32,
            msg: ptr::NonNull<c_char>,
        );

        pub fn on_message(
            transport: Dart_Handle,
            on_message: Dart_Handle,
            on_close: Dart_Handle,
        );
    }
}

/// WebSocket [`RpcTransport`] between a client and a server.
#[derive(Clone, Debug)]
pub struct WebSocketRpcTransport {
    handle: DartHandle,
    on_message_listeners: OnMessageListeners,
    close_reason: Cell<ClientDisconnect>,
    socket_state: Rc<ObservableCell<TransportState>>,
}

/// Notifies all [`OnMessageListeners`] about new received WS message.
#[no_mangle]
pub unsafe extern "C" fn WebSocketRpcTransport__on_message(
    listeners: *const OnMessageListeners,
    msg: ptr::NonNull<c_char>,
) {
    let listeners = listeners.as_ref().unwrap();
    listeners.notify_all(&c_str_into_string(msg));
}

#[derive(Clone, Debug)]
pub struct OnMessageListeners(
    Rc<RefCell<Vec<mpsc::UnboundedSender<ServerMsg>>>>,
);

impl OnMessageListeners {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    fn notify_all(&self, msg: &str) {
        let msg = match serde_json::from_str::<ServerMsg>(msg) {
            Ok(parsed) => parsed,
            Err(e) => {
                // TODO: protocol versions mismatch? should drop
                //       connection if so
                log::error!("{}", tracerr::new!(e));
                return;
            }
        };

        self.0.borrow_mut().retain(|on_message| {
            on_message.unbounded_send(msg.clone()).is_ok()
        });
    }

    fn new_subscriber(&self) -> LocalBoxStream<'static, ServerMsg> {
        let (tx, rx) = mpsc::unbounded();
        self.0.borrow_mut().push(tx);
        Box::pin(rx)
    }
}

impl WebSocketRpcTransport {
    /// Initiates new WebSocket connection. Resolves only when underlying
    /// connection becomes active.
    ///
    /// # Errors
    ///
    /// With [`TransportError::CreateSocket`] if cannot establish WebSocket to
    /// specified URL.
    ///
    /// With [`TransportError::InitSocket`] if [WebSocket.onclose][1] callback
    /// fired before [WebSocket.onopen][2] callback.
    pub async fn new(url: ApiUrl) -> Result<Self> {
        unsafe {
            let handle =
                transport::init(string_into_c_str(url.as_ref().to_string()));
            let on_message_listeners = OnMessageListeners::new();
            let socket_state =
                Rc::new(ObservableCell::new(TransportState::Open));
            transport::on_message(
                handle,
                Callback::from_fn_mut({
                    let on_message_listeners = on_message_listeners.clone();
                    move |msg: String| {
                        on_message_listeners.notify_all(&msg);
                    }
                })
                .into_dart(),
                Callback::from_fn_mut({
                    let socket_state = Rc::clone(&socket_state);
                    move |msg: ()| {
                        socket_state.set(TransportState::Closed(
                            CloseMsg::Normal(1000, CloseReason::Finished),
                        ));
                    }
                })
                .into_dart(),
            );
            Ok(Self {
                handle: DartHandle::new(handle),
                on_message_listeners,
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
        self.on_message_listeners.new_subscriber()
    }

    fn set_close_reason(&self, reason: ClientDisconnect) {
        self.close_reason.set(reason);
    }

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
