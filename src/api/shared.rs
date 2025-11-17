//! API objects that can be shared between `dart` and `wasm`.

use medea_client_api_proto as proto;
#[cfg(doc)]
use medea_client_api_proto::IceConnectionState;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::rpc::ClientDisconnect;
#[cfg(doc)]
use crate::{api::RoomCloseReason, peer::PeerConnection};

/// The reason of why `Room` was closed.
///
/// Provided in a [`RoomCloseReason`]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoomCloseKind {
    /// Unexpected client error.
    InternalClientError,

    /// Unexpected server error.
    InternalServerError,

    /// Room was normally closed by client via `Jason::close_room()`.
    Finished,

    /// Connection has been inactive for a while and thus considered idle
    /// by a server.
    Idle,

    /// Establishing of connection with a server was rejected on server side.
    ///
    /// Most likely because of incorrect `Member` credentials.
    Rejected,

    /// Client was evicted on the server side.
    ///
    /// Usually this means that either `Member` or `Room` was deleted from the
    /// server.
    Evicted,
}

impl From<proto::CloseReason> for RoomCloseKind {
    fn from(val: proto::CloseReason) -> Self {
        // While `Reconnected` is a valid close reason for transport it's not
        // a valid Room close reason, since if client initiates reconnect then
        // closed transport is immediately detached from Room and new transport
        // is created.
        match val {
            proto::CloseReason::Finished => Self::Finished,
            proto::CloseReason::Idle => Self::Idle,
            proto::CloseReason::Rejected => Self::Rejected,
            proto::CloseReason::Evicted => Self::Evicted,
            proto::CloseReason::Reconnected
            | proto::CloseReason::InternalError => Self::InternalServerError,
        }
    }
}

impl From<ClientDisconnect> for RoomCloseKind {
    fn from(val: ClientDisconnect) -> Self {
        match val {
            ClientDisconnect::CloseForReconnection
            | ClientDisconnect::RoomUnexpectedlyDropped
            | ClientDisconnect::RpcClientUnexpectedlyDropped
            | ClientDisconnect::RpcTransportUnexpectedlyDropped
            | ClientDisconnect::SessionUnexpectedlyDropped => {
                Self::InternalClientError
            }
            ClientDisconnect::RoomClosed => Self::Finished,
        }
    }
}

/// Possible connection states of a [`PeerConnection`].
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeerConnectionState {
    /// At least one of the connection's [ICE] transports are in the
    /// [`IceConnectionState::New`] state, and none of them are in one
    /// of the following states: [`IceConnectionState::Checking`],
    /// [`IceConnectionState::Failed`], or
    /// [`IceConnectionState::Disconnected`], or all of the connection's
    /// transports are in the [`IceConnectionState::Closed`] state.
    ///
    /// [ICE]: https://webrtcglossary.com/ice
    New,

    /// One or more of the [ICE] transports are currently in the process of
    /// establishing a connection; that is, their [`IceConnectionState`] is
    /// either [`IceConnectionState::Checking`] or
    /// [`IceConnectionState::Connected`], and no transports are in the
    /// [`IceConnectionState::Failed`] state.
    ///
    /// [ICE]: https://webrtcglossary.com/ice
    Connecting,

    /// Every [ICE] transport used by the connection is either in use (state
    /// [`IceConnectionState::Connected`] or [`IceConnectionState::Completed`])
    /// or is closed ([`IceConnectionState::Closed`]).
    ///
    /// In addition, at least one transport is either
    /// [`IceConnectionState::Connected`] or [`IceConnectionState::Completed`].
    ///
    /// [ICE]: https://webrtcglossary.com/ice
    Connected,

    /// At least one of the [ICE] transports for the connection is in the
    /// [`IceConnectionState::Disconnected`] state and none of the other
    /// transports are in the state [`IceConnectionState::Failed`] or
    /// [`IceConnectionState::Checking`].
    ///
    /// It's not a terminal state, and it can go back to
    /// [`PeerConnectionState::Connecting`] and then
    /// [`PeerConnectionState::Connected`] on its own.
    ///
    /// [ICE]: https://webrtcglossary.com/ice
    Disconnected,

    /// One or more of the [ICE] transports on the connection is in the
    /// [`IceConnectionState::Failed`] state.
    ///
    /// It's not a terminal state, and it can be fixed with [ICE] restart if
    /// signalling connection is alive.
    ///
    /// [ICE]: https://webrtcglossary.com/ice
    Failed,

    /// [`PeerConnection`] is closed.
    ///
    /// It's a terminal state.
    Closed,
}

impl From<proto::PeerConnectionState> for PeerConnectionState {
    fn from(r: proto::PeerConnectionState) -> Self {
        match r {
            proto::PeerConnectionState::New => Self::New,
            proto::PeerConnectionState::Connecting => Self::Connecting,
            proto::PeerConnectionState::Connected => Self::Connected,
            proto::PeerConnectionState::Disconnected => Self::Disconnected,
            proto::PeerConnectionState::Failed => Self::Failed,
            proto::PeerConnectionState::Closed => Self::Closed,
        }
    }
}
