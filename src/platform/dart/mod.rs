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
pub mod media_display_info;
pub mod media_track;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
pub mod transport;
pub mod utils;

use std::panic;

pub use self::{
    constraints::{DisplayMediaStreamConstraints, MediaStreamConstraints},
    error::Error,
    executor::spawn,
    media_device_info::MediaDeviceInfo,
    media_devices::MediaDevices,
    media_display_info::MediaDisplayInfo,
    media_track::MediaStreamTrack,
    peer_connection::RtcPeerConnection,
    rtc_stats::RtcStats,
    transceiver::Transceiver,
    transport::WebSocketRpcTransport,
    utils::{completer::delay_for, Function},
};

/// Sets Rust's [`panic!`] hook providing backtrace of the occurred panic to
/// Dart's functions.
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|bt| {
        if let Some(f) = unsafe { PANIC_FN.as_ref() } {
            f.call1(format!("{bt}"));
        }
    }));
}

/// [`Function`] being called whenever Rust code [`panic`]s.
static mut PANIC_FN: Option<Function<String>> = None;

/// Sets the provided [`Function`] as a callback to be called whenever Rust code
/// [`panic`]s.
///
/// [`panic`]: panic!
pub fn set_panic_callback(cb: Function<String>) {
    unsafe {
        PANIC_FN = Some(cb);
    }
}

#[cfg(target_os = "android")]
/// Initializes [`android_logger`] as the default application logger with
/// minimal log level set to [`log::Level::Debug`].
pub fn init_logger() {
    // TODO: `android_logger::init_once()` should be called only once.
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Debug),
    );
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos", target_os = "ios"))]
/// Initializes [`simple_logger`] as the default application logger with filter
/// level set to [`log::LevelFilter::Debug`].
pub fn init_logger() {
    // TODO: Should be called only once.
    let _ = simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init();
}
