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
pub mod logging;
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

use std::{cell::RefCell, mem::ManuallyDrop, panic, sync::Once};

use libc::c_void;
use log::LevelFilter;

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
use crate::platform::{self, utils::dart_api};

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

/// Initializes global logger.
pub fn init_logger() {
    static INITIALIZED: Once = Once::new();
    INITIALIZED.call_once(|| {
        init_logger_inner(LevelFilter::max());
        log::set_max_level(platform::DEFAULT_LOG_LEVEL);
    });
}

#[cfg(target_os = "android")]
/// Initializes [`android_logger`] as the default application logger.
fn init_logger_inner(level: LevelFilter) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(level),
    );
}

#[cfg(any(
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "windows"
))]
/// Initializes [`simple_logger`] as the default application logger.
fn init_logger_inner(level: LevelFilter) {
    _ = simple_logger::SimpleLogger::new().with_level(level).init();
}

/// Sets the global maximum log level for both `medea-jason` and
/// [`medea-flutter-webrtc`][0] (including bundled [`libwebrtc`][1]).
///
/// [0]: https://github.com/instrumentisto/medea-flutter-webrtc
/// [1]: https://webrtc.googlesource.com/src/
pub async fn set_log_level<T>(level: T)
where
    T: Into<LevelFilter>,
{
    let level: LevelFilter = level.into();
    log::set_max_level(level);
    logging::set_log_level(level).await;
}
