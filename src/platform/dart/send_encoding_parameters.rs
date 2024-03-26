//! [SendEncodingParameters] wrapper.
//!
//! [SendEncodingParameters]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use dart_sys::_Dart_Handle;
use medea_client_api_proto::ScalabilityMode;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::handle::DartHandle;

use super::utils::{c_str_into_string, string_into_c_str};

#[dart_bridge(
    "flutter/lib/src/native/platform/send_encoding_parameters.g.dart"
)]
mod send_encoding_parameters {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    extern "C" {
        /// Creates a new [`SendEncodingParameters`].
        pub fn new_send_encoding_parameters(
            rid: ptr::NonNull<c_char>,
            active: bool,
        ) -> Dart_Handle;

        /// Gets `rid` of this [`SendEncodingParameters`].
        pub fn get_rid(encoding: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Sets `active` for the provided [`SendEncodingParameters`].
        pub fn set_active(encoding: Dart_Handle, active: bool);

        /// Sets `max_bitrate` for the provided [`SendEncodingParameters`].
        pub fn set_max_bitrate(encoding: Dart_Handle, max_bitrate: i64);

        /// Sets `scale_resolution_down_by` for the provided
        /// [`SendEncodingParameters`].
        pub fn set_scale_resolution_down_by(
            encoding: Dart_Handle,
            scale_resolution_down_by: i64,
        );

        pub fn set_scalability_mode(
            encoding: Dart_Handle,
            scalability_mode: ptr::NonNull<c_char>,
        );
    }
}

/// Wrapper around [ RTCRtpEncodingParameters] which provides handy methods for
/// direction changes.
///
/// [ RTCRtpEncodingParameters]: https://tinyurl.com/mr3dt9ch
#[derive(Clone, Debug)]
pub struct SendEncodingParameters(DartHandle);

impl From<DartHandle> for SendEncodingParameters {
    fn from(value: DartHandle) -> Self {
        Self(value)
    }
}

impl SendEncodingParameters {
    /// Creates a new [`SendEncodingParameters`].
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

    /// Returns underlying [`_Dart_Handle`].
    #[must_use]
    pub fn handle(&self) -> *mut _Dart_Handle {
        self.0.get()
    }

    /// Gets `rid`.
    #[must_use]
    pub fn rid(&self) -> String {
        let handle = self.0.get();
        unsafe { c_str_into_string(send_encoding_parameters::get_rid(handle)) }
    }

    /// Sets `active`.
    pub fn set_active(&self, active: bool) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_active(handle, active);
        };
    }

    /// Sets `max_bitrate`.
    pub fn set_max_bitrate(&self, max_bitrate: i64) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_max_bitrate(handle, max_bitrate);
        };
    }

    /// Sets `scale_resolution_down_by`.
    pub fn set_scale_resolution_down_by(&self, scale_resolution_down_by: i64) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_scale_resolution_down_by(
                handle,
                scale_resolution_down_by,
            );
        };
    }

    /// Asd
    pub fn set_scalability_mode(&self, scalability_mode: ScalabilityMode) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_scalability_mode(
                handle,
                string_into_c_str(scalability_mode.to_string()),
            );
        };
    }
}
