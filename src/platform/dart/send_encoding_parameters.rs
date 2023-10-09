//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use dart_sys::_Dart_Handle;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::handle::DartHandle;

use super::utils::string_into_c_str;

#[dart_bridge(
    "flutter/lib/src/native/platform/send_encoding_parameters.g.dart"
)]
mod send_encoding_parameters {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    extern "C" {
        pub fn new_send_encoding_parameters(
            rid: ptr::NonNull<c_char>,
            active: bool,
        ) -> Dart_Handle;

        pub fn set_max_bitrate(encoding: Dart_Handle, max_bitrate: i64);

        pub fn set_scale_resolution_down_by(
            encoding: Dart_Handle,
            scale_resolution_down_by: i64,
        );
    }
}

/// Wrapper around [RTCRtpTransceiver] which provides handy methods for
/// direction changes.
///
/// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
#[derive(Clone, Debug)]
pub struct SendEncodingParameters(DartHandle);

impl SendEncodingParameters {
    #[must_use]
    pub fn new(rid: String, active: bool) -> Self {
        unsafe {
            let handle = send_encoding_parameters::new_send_encoding_parameters(
                string_into_c_str(rid),
                active,
            );
            Self(DartHandle::new(handle))
        }
    }

    #[must_use]
    pub fn handle(&self) -> *mut _Dart_Handle {
        self.0.get()
    }

    pub fn set_max_bitrate(&self, max_bitrate: i64) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_max_bitrate(handle, max_bitrate)
        };
    }

    pub fn set_scale_resolution_down_by(&self, scale_resolution_down_by: i64) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_scale_resolution_down_by(
                handle,
                scale_resolution_down_by,
            )
        };
    }
}
