//! [MediaDeviceInfo][0] related representations.
//!
//! [0]: https://w3.org/TR/mediacapture-streams#device-info

use std::convert::{TryFrom, TryInto};

use medea_macro::dart_bridge;

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
        /// Returns an unique identifier of the provided device.
        pub fn device_id(info: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns a kind of the provided device.
        pub fn kind(info: Dart_Handle) -> i64;

        /// Returns a label describing the provided device (for example,
        /// "External USB Webcam").
        ///
        /// If the provided device has no associated label, then returns an
        /// empty string.
        pub fn label(info: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns a group identifier of the provided device.
        pub fn group_id(
            info: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
    }
}

/// Representation of a [MediaDeviceInfo][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
#[derive(Clone, Debug)]
pub struct InputDeviceInfo {
    /// Handle to the Dart side `InputDeviceInfo`.
    handle: DartHandle,

    /// [`MediaKind`] of this [`InputDeviceInfo`].
    kind: MediaKind,
}

impl InputDeviceInfo {
    /// Returns a unique identifier of the device represented by this
    /// [`InputDeviceInfo`].
    #[must_use]
    pub fn device_id(&self) -> String {
        unsafe {
            c_str_into_string(input_device_info::device_id(self.handle.get()))
        }
    }

    /// Returns a kind of the device represented by this [`InputDeviceInfo`].
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.kind
    }

    /// Returns a label describing the device represented by this
    /// [`InputDeviceInfo`] (for example, "External USB Webcam").
    ///
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            c_str_into_string(input_device_info::label(self.handle.get()))
        }
    }

    /// Returns a group identifier of the device represented by this
    /// [`InputDeviceInfo`]
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
    #[must_use]
    pub fn group_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            input_device_info::group_id(self.handle.get()).unbox()
        })
        .unwrap()
    }
}

impl TryFrom<DartHandle> for InputDeviceInfo {
    type Error = NotInput;

    fn try_from(value: DartHandle) -> Result<Self, Self::Error> {
        let kind = unsafe { input_device_info::kind(value.get()) }
            .try_into()
            .map_err(|_| NotInput)?;

        Ok(Self {
            handle: value,
            kind,
        })
    }
}

/// Error of a [MediaDeviceInfo][0] representing not an input device.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
pub struct NotInput;
