//! Platform-specific functionality.

pub mod callback;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
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
    transceiver::Direction as TransceiverDirection,
    transport::{RpcTransport, TransportError, TransportState},
};

#[cfg(feature = "mockable")]
pub use self::transport::MockRpcTransport;

/// [`Error`] appeared on [getUserMedia()][1] request, differentiated by its
/// cause.
///
/// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
#[derive(Caused, Clone, Debug, Display)]
#[cause(error = "Error")]
pub enum GetUserMediaError {
    /// [`Error`] has been caused by getting audio.
    Audio(Error),

    /// [`Error`] has been caused by getting video.
    Video(Error),

    /// Cause cannot be identified.
    Unknown(Error),
}
