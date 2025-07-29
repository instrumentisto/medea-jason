//! State of a member [`Connection`].

use medea_client_api_proto as proto;
#[cfg(doc)]
use medea_client_api_proto::IceConnectionState;
use wasm_bindgen::prelude::*;

use crate::connection as core;
#[cfg(doc)]
use crate::connection::Connection;

/// `PeerConnection`'s connection state.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum PeerConnectionState {
    /// At least one of the connection's ICE transports are in the
    /// [`IceConnectionState::New`] state, and none of them are in one
    /// of the following states: [`IceConnectionState::Checking`],
    /// [`IceConnectionState::Failed`], or
    /// [`IceConnectionState::Disconnected`], or all of the connection's
    /// transports are in the [`IceConnectionState::Closed`] state.
    New,

    /// One or more of the ICE transports are currently in the process of
    /// establishing a connection; that is, their [`IceConnectionState`] is
    /// either [`IceConnectionState::Checking`] or
    /// [`IceConnectionState::Connected`], and no transports are in the
    /// [`IceConnectionState::Failed`] state.
    Connecting,

    /// Every ICE transport used by the connection is either in use (state
    /// [`IceConnectionState::Connected`] or [`IceConnectionState::Completed`])
    /// or is closed ([`IceConnectionState::Closed`]); in addition,
    /// at least one transport is either [`IceConnectionState::Connected`] or
    /// [`IceConnectionState::Completed`].
    Connected,

    /// At least one of the ICE transports for the connection is in the
    /// [`IceConnectionState::Disconnected`] state and none of the other
    /// transports are in the state [`IceConnectionState::Failed`] or
    /// [`IceConnectionState::Checking`].
    ///
    /// It's not a terminal state, and it can go back to `Connecting`
    /// and then `Connected` on its own.
    Disconnected,

    /// One or more of the ICE transports on the connection is in the
    /// [`IceConnectionState::Failed`] state.
    ///
    /// It's not a terminal state, and it can be fixed with ICE restart if
    /// signalling connection is alive.
    Failed,

    /// The `PeerConnection` is closed.
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

/// [`Connection`]'s state kind.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemberConnectionStateKind {
    /// [`Connection`]'s state is in P2P mode.
    P2P,
}

/// [`Connection`]'s state.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct MemberConnectionState(core::MemberConnectionState);

#[wasm_bindgen]
impl MemberConnectionState {
    /// Returns the [`Connection`]'s mode.
    #[must_use]
    pub const fn kind(&self) -> MemberConnectionStateKind {
        match self.0 {
            core::MemberConnectionState::P2P(_) => {
                MemberConnectionStateKind::P2P
            }
        }
    }

    /// Returns the [`Connection`]'s state associated with its mode.
    #[must_use]
    pub fn value(&self) -> JsValue {
        match self.0 {
            core::MemberConnectionState::P2P(state) => {
                Into::<PeerConnectionState>::into(state).into()
            }
        }
    }
}

impl From<core::MemberConnectionState> for MemberConnectionState {
    fn from(r: core::MemberConnectionState) -> Self {
        Self(r)
    }
}
