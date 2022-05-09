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

/// Represents where the [`Error`] has appeared.
#[derive(Caused, Clone, Debug, Display)]
#[cause(error = "Error")]
pub enum GetUserMediaError {
    /// The [`Error`] has been caused on getting audio.
    Audio(Error),

    /// The [`Error`] has been caused on getting video.
    Video(Error),

    /// Can not identify the cause of the [`Error`].
    Unknown(Error),
}

impl From<(i64, Error)> for GetUserMediaError {
    fn from((kind, error): (i64, Error)) -> Self {
        match kind {
            0 => Self::Audio(error),
            1 => Self::Video(error),
            _ => Self::Unknown(error),
        }
    }
}
