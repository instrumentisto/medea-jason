//! Platform-specific functionality.

pub mod callback;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
pub mod transceiver_direction;
pub mod transport;

cfg_if::cfg_if! {
    if #[cfg(not(target_family = "wasm"))] {
        mod dart;
        pub use self::dart::*;
    } else {
        mod wasm;
        pub use self::wasm::*;
    }
}

pub use self::{
    callback::Callback,
    peer_connection::{IceCandidate, RtcPeerConnectionError, SdpType},
    rtc_stats::RtcStatsError,
    transceiver_direction::TransceiverDirection,
    transport::{RpcTransport, TransportError, TransportState},
};

#[cfg(feature = "mockable")]
pub use self::transport::MockRpcTransport;
