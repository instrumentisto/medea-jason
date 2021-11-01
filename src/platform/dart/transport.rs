use std::{
    cell::{Cell, RefCell},
    os::raw::c_char,
    ptr,
    rc::Rc,
};

use dart_sys::Dart_Handle;
use futures::{channel::mpsc, prelude::stream::LocalBoxStream};
use medea_client_api_proto::{ClientMsg, CloseReason, ServerMsg};
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

type NewFunction = extern "C" fn(ptr::NonNull<c_char>) -> Dart_Handle;

type SendFunction = extern "C" fn(Dart_Handle, ptr::NonNull<c_char>);

type CloseFunction = extern "C" fn(Dart_Handle, i32, ptr::NonNull<c_char>);

type ListenWsFunction = extern "C" fn(Dart_Handle, Dart_Handle, Dart_Handle);

/// Stores pointer to the [`NewFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_FUNCTION: Option<NewFunction> = None;

/// Stores pointer to the [`SendFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SEND_FUNCTION: Option<SendFunction> = None;

/// Stores pointer to the [`CloseFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CLOSE_FUNCTION: Option<CloseFunction> = None;

/// Stores pointer to the [`ListenWsFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut LISTEN_WS_FUNCTION: Option<ListenWsFunction> = None;

/// Registers the provided [`NewFunction`] as [`NEW_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_WebSocketRpcTransport__new(f: NewFunction) {
    NEW_FUNCTION = Some(f);
}

/// Registers the provided [`SendFunction`] as [`SEND_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_WebSocketRpcTransport__send(f: SendFunction) {
    SEND_FUNCTION = Some(f);
}

/// Registers the provided [`CloseFunction`] as [`CLOSE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_WebSocketRpcTransport__close(
    f: CloseFunction,
) {
    CLOSE_FUNCTION = Some(f);
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
            let handle = NEW_FUNCTION.unwrap()(string_into_c_str(
                url.as_ref().to_string(),
            ));
            let on_message_listeners = OnMessageListeners::new();
            let socket_state =
                Rc::new(ObservableCell::new(TransportState::Open));
            LISTEN_WS_FUNCTION.unwrap()(
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

/// Registers the provided [`ListenWsFunction`] as [`LISTEN_WS_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_WebSocketRpcTransport__listen_ws(
    f: ListenWsFunction,
) {
    LISTEN_WS_FUNCTION = Some(f);
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
            SEND_FUNCTION.unwrap()(self.handle.get(), string_into_c_str(msg));
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
            CLOSE_FUNCTION.unwrap()(
                self.handle.get(),
                1000,
                string_into_c_str(rsn),
            );
        }
    }
}
