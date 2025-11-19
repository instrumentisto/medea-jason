//! State of member's [`Connection`].

#[cfg(doc)]
use medea_client_api_proto::IceConnectionState;
use wasm_bindgen::prelude::*;

use crate::{api::PeerConnectionState, connection as core};
#[cfg(doc)]
use crate::{connection::Connection, peer::PeerConnection};

/// Possible kinds of [`Connection`]'s state.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemberConnectionStateKind {
    /// [`Connection`]'s state is in [P2P mesh] mode.
    ///
    /// [P2P mesh]: https://webrtcglossary.com/mesh
    P2P,
}

/// [`Connection`]'s state.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct MemberConnectionState(core::MemberConnectionState);

#[expect( // `wasm_bindgen` doesn't support `const fn`
    clippy::missing_const_for_fn,
    reason = "`wasm_bindgen` doesn't support `const fn`"
)]
#[wasm_bindgen]
impl MemberConnectionState {
    /// Returns the [`Connection`]'s mode.
    #[must_use]
    pub fn kind(&self) -> MemberConnectionStateKind {
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
