//! State of member's [`Connection`].

use medea_client_api_proto as proto;
#[cfg(doc)]
use medea_client_api_proto::IceConnectionState;

use crate::{api::dart::api::ForeignClass, connection as core};
#[cfg(doc)]
use crate::{connection::Connection, peer::PeerConnection};

/// Possible connection states of a [`PeerConnection`].
#[derive(Clone, Copy, Debug)]
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

/// State of member's [`Connection`].
#[derive(Clone, Copy, Debug)]
pub enum MemberConnectionState {
    /// State in [P2P mesh] mode.
    ///
    /// [P2P mesh]: https://webrtcglossary.com/mesh
    P2P(PeerConnectionState),
}

impl From<core::MemberConnectionState> for MemberConnectionState {
    fn from(r: core::MemberConnectionState) -> Self {
        match r {
            core::MemberConnectionState::P2P(state) => Self::P2P(state.into()),
        }
    }
}

impl ForeignClass for MemberConnectionState {}
