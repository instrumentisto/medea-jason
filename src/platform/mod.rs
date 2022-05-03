//! Platform-specific functionality.

pub mod callback;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
pub mod transceiver_direction;
pub mod transport;

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        mod wasm;
        pub use self::wasm::*;
    } else {
        mod dart;
        pub use self::dart::*;
    }
}

use derive_more::Display;

use crate::utils::Caused;

pub use self::{
    callback::Callback,
    peer_connection::{IceCandidate, RtcPeerConnectionError, SdpType},
    rtc_stats::RtcStatsError,
    transceiver_direction::TransceiverDirection,
    transport::{RpcTransport, TransportError, TransportState},
};

#[cfg(feature = "mockable")]
pub use self::transport::MockRpcTransport;

#[derive(Caused, Clone, Debug, Display)]
#[cause(error = "Error")]
pub enum GetUserMediaError {
    Audio(Error),
    Video(Error),
    Unknown(Error),
}
