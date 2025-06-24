//! State of a member [`Connection`].

#[cfg(doc)]
use crate::connection::Connection;
use medea_client_api_proto::PeerConnectionState;

use crate::{api::dart::api::ForeignClass, connection as core};

#[derive(Clone, Copy, Debug)]
pub enum MemberConnectionState {
    /// State in P2P mode.
    P2P(PeerConnectionState),
}

impl From<core::MemberConnectionState> for MemberConnectionState {
    fn from(r: core::MemberConnectionState) -> Self {
        match r {
            core::MemberConnectionState::P2P(state) => Self::P2P(state),
        }
    }
}

impl ForeignClass for MemberConnectionState {}
