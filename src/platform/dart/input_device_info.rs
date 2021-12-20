//! [MediaDeviceInfo][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams/#device-info

use derive_more::From;
use medea_macro::dart_bridge;
use std::convert::{TryFrom, TryInto};

use crate::{
    api::c_str_into_string,
    media::MediaKind,
    platform::dart::utils::{handle::DartHandle, NonNullDartValueArgExt},
};

#[dart_bridge("flutter/lib/src/native/platform/input_device_info.g.dart")]
mod input_device_info {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    extern "C" {
        /// Returns unique identifier for the provided device.
        pub fn device_id(info: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns kind of the provided device.
        pub fn kind(
            info: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i64>>>;

        /// Returns label describing the provided device (for example
        /// "External USB Webcam").
        ///
        /// If the device has no associated label, then returns an empty string.
        pub fn label(info: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns group identifier of the provided device.
        pub fn group_id(
            info: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
    }
}

/// Representation of [MediaDeviceInfo][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
#[derive(Clone, Debug, From)]
pub struct InputDeviceInfo(DartHandle);

impl InputDeviceInfo {
    /// Returns unique identifier for the represented device.
    #[must_use]
    pub fn device_id(&self) -> String {
        unsafe { c_str_into_string(input_device_info::device_id(self.0.get())) }
    }

    /// Returns kind of the represented device.
    ///
    /// This representation of [MediaDeviceInfo][1] ONLY for input device.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#device-info
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        // Kind should be always Some
        Option::<i64>::try_from(unsafe {
            input_device_info::kind(self.0.get()).unbox()
        })
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap()
    }

    /// Returns label describing the represented device (for example
    /// "External USB Webcam").
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe { c_str_into_string(input_device_info::label(self.0.get())) }
    }

    /// Returns group identifier of the represented device.
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
    #[must_use]
    pub fn group_id(&self) -> String {
        // Group ID should be always Some TODO: can be None
        Option::try_from(unsafe {
            input_device_info::group_id(self.0.get()).unbox()
        })
        .unwrap()
        .unwrap()
    }
}
