//! Multiplatform Dart runtime specific functionality.

// TODO: Remove allows when implementing platform code.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::unused_self,
    clippy::needless_pass_by_value,
    unused_variables
)]

pub mod constraints;
pub mod error;
pub mod executor;
pub mod ice_candidate;
pub mod ice_server;
pub mod media_device_info;
pub mod media_devices;
pub mod media_track;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
pub mod transport;
pub mod utils;

pub use self::{
    constraints::{DisplayMediaStreamConstraints, MediaStreamConstraints},
    error::Error,
    executor::spawn,
    media_device_info::MediaDeviceInfo,
    media_devices::{
        enumerate_devices, MediaDevices,
    },
    media_track::MediaStreamTrack,
    peer_connection::RtcPeerConnection,
    rtc_stats::RtcStats,
    transceiver::Transceiver,
    transport::WebSocketRpcTransport,
    utils::{completer::delay_for, Function},
};

/// TODO: Implement panic hook.
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|bt| {
        log::error!("Rust code panicked {bt:?}");
    }));
}

/// Initialize [`android_logger`] as default application logger with min log
/// level set to [`log::Level::Debug`].
///
/// [`android_logger`]: https://docs.rs/android_logger
pub fn init_logger() {
    // TODO: `android_logger::init_once()` should be called only once.
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Debug),
    );
}
