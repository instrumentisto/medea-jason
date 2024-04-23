//! Platform-specific functionality.

pub mod callback;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
pub mod transport;
pub mod codec_capability;

#[cfg(not(target_family = "wasm"))]
mod dart;
#[cfg(not(target_family = "wasm"))]
pub use self::dart::*;

#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use self::wasm::*;

use derive_more::Display;

use crate::utils::Caused;

pub use self::{
    callback::Callback,
    peer_connection::{
        IceCandidate, IceCandidateError, RtcPeerConnectionError, SdpType,
    },
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
#[cause(error = Error)]
pub enum GetUserMediaError {
    /// [`Error`] has been caused by getting audio.
    Audio(Error),

    /// [`Error`] has been caused by getting video.
    Video(Error),

    /// Cause cannot be identified.
    Unknown(Error),
}
