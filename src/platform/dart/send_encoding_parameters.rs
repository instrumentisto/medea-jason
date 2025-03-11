//! Wrapper around [RTCRtpEncodingParameters][0].
//!
//! [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::{
    handle::DartHandle, NonNullDartValueArgExt as _,
};

use super::utils::{c_str_into_string, string_into_c_str};
use crate::platform::dart::utils::handle::DartHandle;

#[dart_bridge(
    "flutter/lib/src/native/platform/send_encoding_parameters.g.dart"
)]
mod send_encoding_parameters {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

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

        /// Returns [activeness][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
        pub fn get_active(encoding: Dart_Handle) -> Result<bool, Error>;

        /// Sets [maxBitrate][1] of the provided [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]:https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
        pub fn set_max_bitrate(
            encoding: Dart_Handle,
            max_bitrate: u32,
        ) -> Result<(), Error>;

        /// Returns [maxBitrate][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]:https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
        pub fn get_max_bitrate(
            encoding: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<u32>>>, Error>;

        /// Sets [scaleResolutionDownBy][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://tinyurl.com/ypzzc75t
        pub fn set_scale_resolution_down_by(
            encoding: Dart_Handle,
            scale_resolution_down_by: f64,
        ) -> Result<(), Error>;

        /// Returns [scaleResolutionDownBy][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://tinyurl.com/ypzzc75t
        pub fn get_scale_resolution_down_by(
            encoding: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<f64>>>, Error>;

        /// Sets [scalabilityMode][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://tinyurl.com/3zuaee45
        pub fn set_scalability_mode(
            encoding: Dart_Handle,
            scalability_mode: ptr::NonNull<c_char>,
        ) -> Result<(), Error>;

        /// Returns [scalabilityMode][1] of the provided
        /// [RTCRtpEncodingParameters][0].
        ///
        /// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        /// [1]: https://tinyurl.com/3zuaee45
        pub fn get_scalability_mode(
            encoding: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<String>>>, Error>;
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
    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    #[must_use]
    pub fn rid(&self) -> Option<String> {
        let rid =
            unsafe { send_encoding_parameters::get_rid(self.0.get()).unwrap() };

        let rid = unsafe { c_str_into_string(rid) };

        if rid.is_empty() {
            None
        } else {
            Some(rid)
        }
    }

    /// Sets [activeness][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
    pub fn set_active(&self, active: bool) {
        unsafe { send_encoding_parameters::set_active(self.0.get(), active) }
            .unwrap();
    }

    /// Returns [activeness][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
    #[must_use]
    pub fn active(&self) -> bool {
        unsafe { send_encoding_parameters::get_active(self.0.get()) }.unwrap()
    }

    /// Sets [maxBitrate][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
    pub fn set_max_bitrate(&self, max_bitrate: u32) {
        unsafe {
            send_encoding_parameters::set_max_bitrate(self.0.get(), max_bitrate)
        }
        .unwrap();
    }

    /// Returns [maxBitrate][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    #[must_use]
    pub fn max_bitrate(&self) -> Option<u32> {
        let max_bitrate =
            unsafe { send_encoding_parameters::get_max_bitrate(self.0.get()) }
                .unwrap();

        Option::try_from(unsafe { max_bitrate.unbox() }).unwrap()
    }

    /// Sets [scaleResolutionDownBy][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/ypzzc75t
    pub fn set_scale_resolution_down_by(&self, scale_resolution_down_by: f64) {
        unsafe {
            send_encoding_parameters::set_scale_resolution_down_by(
                self.0.get(),
                scale_resolution_down_by,
            )
        }
        .unwrap();
    }

    /// Returns [scaleResolutionDownBy][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/ypzzc75t
    #[must_use]
    pub fn scale_resolution_down_by(&self) -> f64 {
        let scale_resolution_down_by = unsafe {
            send_encoding_parameters::get_scale_resolution_down_by(self.0.get())
        }
        .unwrap();

        Option::try_from(unsafe { scale_resolution_down_by.unbox() })
            .unwrap()
            .unwrap_or(1.0)
    }

    /// Sets [scalabilityMode][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/3zuaee45
    pub fn set_scalability_mode(&self, scalability_mode: String) {
        unsafe {
            send_encoding_parameters::set_scalability_mode(
                self.0.get(),
                string_into_c_str(scalability_mode),
            )
        }
        .unwrap();
    }

    /// Returns [scalabilityMode][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/3zuaee45
    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    #[must_use]
    pub fn scalability_mode(&self) -> Option<String> {
        let mode = unsafe {
            send_encoding_parameters::get_scalability_mode(self.0.get())
        }
        .unwrap();
        Option::try_from(unsafe { mode.unbox() }).unwrap()
    }
}
