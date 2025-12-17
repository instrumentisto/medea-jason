//! Platform-specific functionality.

pub mod callback;
pub mod codec_capability;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
pub mod transport;

#[cfg(not(target_family = "wasm"))]
mod dart;
#[cfg(not(target_family = "wasm"))]
pub use self::dart::*;

#[cfg(target_family = "wasm")]
mod wasm;
use derive_more::with_trait::Display;

#[cfg(feature = "mockable")]
pub use self::transport::MockRpcTransport;
#[cfg(target_family = "wasm")]
pub use self::wasm::*;
pub use self::{
    callback::Callback,
    codec_capability::get_capabilities,
    peer_connection::{
        IceCandidate, IceCandidateError, IceGatheringState,
        RtcPeerConnectionError, SdpType,
    },
    rtc_stats::RtcStatsError,
    send_encoding_parameters::SendEncodingParameters,
    transceiver::Direction as TransceiverDirection,
    transport::{RpcTransport, TransportError, TransportState},
};
use crate::utils::Caused;

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
