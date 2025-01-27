//! Wrapper around [RTCRtpEncodingParameters][0].
//!
//! [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters

use dart_sys::Dart_Handle;
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

    use crate::platform::Error;

    extern "C" {
        /// Creates new [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        pub fn new_send_encoding_parameters(
            rid: ptr::NonNull<c_char>,
            active: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Returns [RID] from the provided [RTCRtpEncodingParameters][0].
        ///
        /// [RID]: https://w3.org/TR/webrtc#dom-rtcrtpcodingparameters-rid
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        pub fn get_rid(
            encoding: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;

        /// Sets [activeness][1] of the provided [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
        pub fn set_active(
            encoding: Dart_Handle,
            active: bool,
        ) -> Result<(), Error>;

        /// Sets [maxBitrate][1] of the provided [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]:https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
        pub fn set_max_bitrate(
            encoding: Dart_Handle,
            max_bitrate: i64,
        ) -> Result<(), Error>;

        /// Sets [scaleResolutionDownBy][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://tinyurl.com/ypzzc75t
        pub fn set_scale_resolution_down_by(
            encoding: Dart_Handle,
            scale_resolution_down_by: i64,
        ) -> Result<(), Error>;

        /// Sets [scalabilityMode][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://tinyurl.com/3zuaee45
        pub fn set_scalability_mode(
            encoding: Dart_Handle,
            scalability_mode: ptr::NonNull<c_char>,
        ) -> Result<(), Error>;
    }
}

/// Wrapper around [RTCRtpEncodingParameters][0] providing handy methods for its
/// direction changes.
///
/// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
#[derive(Clone, Debug)]
pub struct SendEncodingParameters(DartHandle);

impl From<DartHandle> for SendEncodingParameters {
    fn from(value: DartHandle) -> Self {
        Self(value)
    }
}

impl SendEncodingParameters {
    /// Creates new [`SendEncodingParameters`].
    #[must_use]
    pub fn new(rid: String, active: bool) -> Self {
        let handle = unsafe {
            send_encoding_parameters::new_send_encoding_parameters(
                string_into_c_str(rid),
                active,
            )
        }
        .unwrap();
        Self(unsafe { DartHandle::new(handle) })
    }

    /// Returns the underlying [`Dart_Handle`] of these
    /// [`SendEncodingParameters`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns [RID] of these [`SendEncodingParameters`].
    ///
    /// [RID]: https://w3.org/TR/webrtc#dom-rtcrtpcodingparameters-rid
    #[must_use]
    pub fn rid(&self) -> String {
        let handle = self.0.get();
        let rid = unsafe { send_encoding_parameters::get_rid(handle) }.unwrap();
        unsafe { c_str_into_string(rid) }
    }

    /// Sets [activeness][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
    pub fn set_active(&self, active: bool) {
        let handle = self.0.get();
        unsafe { send_encoding_parameters::set_active(handle, active) }
            .unwrap();
    }

    /// Sets [maxBitrate][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
    pub fn set_max_bitrate(&self, max_bitrate: u32) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_max_bitrate(
                handle,
                max_bitrate.into(),
            )
        }
        .unwrap();
    }

    /// Sets [scaleResolutionDownBy][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/ypzzc75t
    pub fn set_scale_resolution_down_by(&self, scale_resolution_down_by: i64) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_scale_resolution_down_by(
                handle,
                scale_resolution_down_by,
            )
        }
        .unwrap();
    }

    /// Sets [scalabilityMode][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/3zuaee45
    pub fn set_scalability_mode(&self, scalability_mode: ScalabilityMode) {
        let handle = self.0.get();
        unsafe {
            send_encoding_parameters::set_scalability_mode(
                handle,
                string_into_c_str(scalability_mode.to_string()),
            )
        }
        .unwrap();
    }
}
