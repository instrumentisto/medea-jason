//! Platform-agnostic functionality of RPC transport.

use async_trait::async_trait;
use derive_more::Display;
use futures::stream::LocalBoxStream;
use medea_client_api_proto::{ClientMsg, ServerMsg};
use tracerr::Traced;

use crate::{
    platform,
    rpc::{ApiUrl, ClientDisconnect, CloseMsg},
    utils::{Caused, JsonParseError},
};

/// Possible states of a [`RpcTransport`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportState {
    /// Socket has been created. The connection is not opened yet.
    Connecting,

    /// The connection is opened and ready to communicate.
    Open,

    /// The connection is in the process of closing.
    Closing,

    /// The connection is closed or couldn't be opened.
    ///
    /// [`CloseMsg`] is the reason of why [`RpcTransport`] went into this
    /// [`TransportState`].
    Closed(CloseMsg),
}

impl TransportState {
    /// Indicates whether the socket can be closed.
    #[must_use]
    pub const fn can_close(self) -> bool {
        matches!(self, Self::Connecting | Self::Open)
    }
}

/// RPC transport between a client and a server.
#[allow(unused_lifetimes)]
#[async_trait(?Send)]
#[cfg_attr(feature = "mockable", mockall::automock)]
#[cfg_attr(feature = "mockable", allow(clippy::missing_docs_in_private_items))]
pub trait RpcTransport {
    /// Initiates a new [WebSocket] connection to the provided `url`.
    ///
    /// Resolves only when the underlying connection becomes active.
    ///
    /// # Errors
    ///
    /// With [`TransportError::CreateSocket`] if cannot establish [WebSocket] to
    /// the provided `url`.
    ///
    /// With [`TransportError::InitSocket`] if [WebSocket.onclose][1] callback
    /// fired before [WebSocket.onopen][2] callback.
    ///
    /// # Panics
    ///
    /// If the binding to [`close`][3] or [`open`][4] events fails. Not supposed
    /// to ever happen.
    ///
    /// [1]: https://developer.mozilla.org/docs/Web/API/WebSocket/onclose
    /// [2]: https://developer.mozilla.org/docs/Web/API/WebSocket/onopen
    /// [3]: https://html.spec.whatwg.org#event-close
    /// [4]: https://html.spec.whatwg.org#event-open
    /// [WebSocket]: https://developer.mozilla.org/docs/Web/API/WebSocket
    async fn connect(&self, url: ApiUrl) -> Result<(), Traced<TransportError>>;

    /// Returns [`LocalBoxStream`] of all messages received by this transport.
    fn on_message(&self) -> LocalBoxStream<'static, ServerMsg>;

    /// Sets reason, that will be sent to remote server when this transport will
    /// be dropped.
    fn set_close_reason(&self, reason: ClientDisconnect);

    /// Sends given [`ClientMsg`] to a server.
    ///
    /// # Errors
    ///
    /// Errors if sending [`ClientMsg`] fails.
    fn send(&self, msg: &ClientMsg) -> Result<(), Traced<TransportError>>;

    /// Subscribes to a [`RpcTransport`]'s [`TransportState`] changes.
    fn on_state_change(&self) -> LocalBoxStream<'static, TransportState>;
}

/// Errors that may occur when working with a [`RpcTransport`].
#[derive(Caused, Clone, Debug, Display, PartialEq)]
#[cause(error = platform::Error)]
pub enum TransportError {
    /// Error encountered when trying to establish connection.
    #[display("Failed to create WebSocket: {_0}")]
    CreateSocket(platform::Error),

    /// Connection was closed before becoming active.
    #[display("Failed to init WebSocket")]
    InitSocket,

    /// Occurs when [`ClientMsg`] cannot be serialized.
    #[display("Failed to parse client message: {_0}")]
    SerializeClientMessage(JsonParseError),

    /// Occurs when [`ServerMsg`] cannot be parsed.
    #[display("Failed to parse server message: {_0}")]
    ParseServerMessage(JsonParseError),

    /// Occurs if the parsed message is not string.
    #[display("Message is not a string")]
    MessageNotString,

    /// Occurs when a message cannot be sent to server.
    #[display("Failed to send message: {_0}")]
    SendMessage(platform::Error),

    /// Occurs when message is sent to a closed socket.
    #[display("Underlying socket is closed")]
    ClosedSocket,
}
