//! Multiplatform Dart runtime specific functionality.

// TODO: Needs refactoring.
#![expect(
    clippy::as_conversions,
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used,
    reason = "needs refactoring"
)]

pub mod codec_capability;
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
pub mod send_encoding_parameters;
pub mod send_parameters;
pub mod transceiver;
pub mod transport;
pub mod utils;

use std::{cell::RefCell, mem::ManuallyDrop, panic};

use libc::c_void;

pub use self::{
    codec_capability::CodecCapability,
    constraints::{DisplayMediaStreamConstraints, MediaStreamConstraints},
    error::Error,
    executor::spawn,
    media_device_info::MediaDeviceInfo,
    media_devices::MediaDevices,
    media_display_info::MediaDisplayInfo,
    media_track::MediaStreamTrack,
    peer_connection::RtcPeerConnection,
    rtc_stats::RtcStats,
    transceiver::{Transceiver, TransceiverInit},
    transport::WebSocketRpcTransport,
    utils::{Function, completer::delay_for},
};
use crate::platform::utils::dart_api;

/// Function to initialize `dart_api_dl` functions.
///
/// # Safety
///
/// This function should never be called manually.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_jason_dart_api_dl(data: *mut c_void) -> isize {
    unsafe { dart_api::initialize_api(data) }
}

/// Sets Rust's [`panic!`] hook providing backtrace of the occurred panic to
/// Dart's functions.
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|bt| {
        PANIC_FN.with_borrow(|f| {
            if let Some(f) = f {
                f.call1(format!("{bt}"));
            }
        });
    }));
}

thread_local! {
    /// [`Function`] being called whenever Rust code [`panic`]s.
    // NOTE: Wrapped with `ManuallyDrop` because `platform::Function` calls
    //       DartVM API on `Drop`, which is already inaccessible once thread
    //       local data is dropped.
    static PANIC_FN: RefCell<Option<ManuallyDrop<Function<String>>>> =
        RefCell::default();
}

/// Sets the provided [`Function`] as a callback to be called whenever Rust code
/// [`panic`]s.
///
/// [`panic`]: panic!
pub fn set_panic_callback(cb: Function<String>) {
    if let Some(old_cb) = PANIC_FN.replace(Some(ManuallyDrop::new(cb))) {
        drop(ManuallyDrop::into_inner(old_cb));
    }
}

#[cfg(target_os = "android")]
/// Initializes [`android_logger`] as the default application logger with filter
/// level set to [`log::LevelFilter::Debug`].
pub fn init_logger() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug),
    );
}

#[cfg(any(
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "windows"
))]
/// Initializes [`simple_logger`] as the default application logger with filter
/// level set to [`log::LevelFilter::Debug`].
pub fn init_logger() {
    use std::cell::LazyCell;

    thread_local! {
        /// [`LazyCell`] ensuring that a [`simple_logger`] is initialized only
        /// once.
        static INITIALIZED: LazyCell<()> = LazyCell::new(|| {
            _ = simple_logger::SimpleLogger::new()
                .with_level(log::LevelFilter::Debug)
                .init();
        });
    }
    INITIALIZED.with(|i| **i);
}
