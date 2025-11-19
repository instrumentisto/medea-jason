//! State of member's [`Connection`].

#[cfg(doc)]
use medea_client_api_proto::IceConnectionState;

use crate::{
    api::{PeerConnectionState, dart::api::ForeignClass},
    connection as core,
};
#[cfg(doc)]
use crate::{connection::Connection, peer::PeerConnection};

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
