//! Multiplatform Dart runtime specific utility structs and functions.

#[allow(clippy::pedantic, clippy::all)]
pub mod array;
pub mod callback_listener;
pub mod completer;
pub mod dart_api;
pub mod dart_future;
pub mod function;
pub mod handle;
pub mod list;
pub mod map;
pub mod nullable;
pub mod option;

use medea_client_api_proto::{IceConnectionState, PeerConnectionState};

#[doc(inline)]
pub use self::{completer::Completer, function::Function};

/// Returns [`IceConnectionState`] based on the provided enum index.
#[must_use]
pub fn ice_connection_from_int(i: i32) -> IceConnectionState {
    match i {
        0 => IceConnectionState::New,
        1 => IceConnectionState::Checking,
        2 => IceConnectionState::Connected,
        3 => IceConnectionState::Completed,
        4 => IceConnectionState::Failed,
        5 => IceConnectionState::Disconnected,
        6 => IceConnectionState::Closed,
        _ => unreachable!(),
    }
}

/// Returns [`PeerConnectionState`] based on the provided enum index.
#[must_use]
pub fn peer_connection_state_from_int(i: i32) -> PeerConnectionState {
    match i {
        0 => PeerConnectionState::New,
        1 => PeerConnectionState::Connecting,
        2 => PeerConnectionState::Connected,
        3 => PeerConnectionState::Disconnected,
        4 => PeerConnectionState::Failed,
        5 => PeerConnectionState::Closed,
        _ => unreachable!(),
    }
}
