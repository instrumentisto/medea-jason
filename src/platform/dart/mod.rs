//! Multiplatform Dart runtime specific functionality.

// TODO: Remove allows when implementing platform code.
#![allow(
    unused_variables,
    clippy::missing_panics_doc,
    clippy::unused_self,
    clippy::needless_pass_by_value
)]

pub mod constraints;
pub mod error;
pub mod executor;
pub mod ice_candidate;
pub mod ice_server;
pub mod input_device_info;
pub mod media_devices;
pub mod media_track;
pub mod peer_connection;
pub mod rtc_stats;
pub mod transceiver;
pub mod transport;
pub mod utils;

use std::{os::raw::c_char, panic, ptr, time::Duration};

use dart_sys::Dart_Handle;

use crate::{
    api::string_into_c_str,
    platform::dart::utils::{
        dart_api::Dart_PropagateError_DL_Trampolined,
        dart_future::DartFutureResolver,
    },
};

pub use self::{
    constraints::{DisplayMediaStreamConstraints, MediaStreamConstraints},
    error::Error,
    executor::spawn,
    input_device_info::InputDeviceInfo,
    media_devices::{enumerate_devices, get_display_media, get_user_media},
    media_track::MediaStreamTrack,
    peer_connection::RtcPeerConnection,
    rtc_stats::RtcStats,
    transceiver::Transceiver,
    transport::WebSocketRpcTransport,
    utils::Function,
};

/// Pointer to an extern function that returns [`Dart_Handle`] to the newly
/// created Dart exception
type NewExceptionFunction = extern "C" fn(ptr::NonNull<c_char>) -> Dart_Handle;

/// Stores pointer to the [`NewExceptionFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_EXCEPTION_FUNCTION: Option<NewExceptionFunction> = None;

/// Registers the provided [`NewExceptionFunction`] as
/// [`NEW_EXCEPTION_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_new_exception_function(
    f: NewExceptionFunction,
) {
    NEW_EXCEPTION_FUNCTION = Some(f);
}

/// Sets hook for all Jason panics, which will convert this panic to the Dart
/// exception and throw it to the Dart side.
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|s| {
        let exception = unsafe {
            NEW_EXCEPTION_FUNCTION.unwrap()(string_into_c_str(s.to_string()))
        };
        unsafe { Dart_PropagateError_DL_Trampolined(exception) };
    }));
}

/// Pointer to an extern function that returns [`Dart_Handle`] to the Dart
/// `Future` which will wait for the provided amount of time.
type DelayedFutureFunction = extern "C" fn(i32) -> Dart_Handle;

/// Stores pointer to the [`DelayedFutureFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut DELAYED_FUTURE_FUNCTION: Option<DelayedFutureFunction> = None;

/// Registers the provided [`DelayedFutureFunction`] as
/// [`DELAYED_FUTURE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_delayed_future_function(
    f: DelayedFutureFunction,
) {
    DELAYED_FUTURE_FUNCTION = Some(f);
}

/// [`Future`] which resolves after the provided [`Duration`].
///
/// # Panics
///
/// Panics if [`DELAYED_FUTURE_FUNCTION`] isn't set by Dart side. This is should
/// be impossible case.
pub async fn delay_for(delay: Duration) {
    #[allow(clippy::cast_possible_truncation)]
    let delay = delay.as_millis() as i32;
    let dart_fut = unsafe { DELAYED_FUTURE_FUNCTION.unwrap()(delay) };
    DartFutureResolver::execute::<()>(dart_fut).await;
}
