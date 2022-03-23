//! [MediaDeviceInfo][0] related representations.
//!
//! [0]: https://w3.org/TR/mediacapture-streams#device-info

use medea_macro::dart_bridge;

use crate::{
    api::c_str_into_string,
    media::MediaDeviceKind,
    platform::dart::utils::{handle::DartHandle, NonNullDartValueArgExt},
};

#[dart_bridge("flutter/lib/src/native/platform/media_device_info.g.dart")]
mod media_device_info {
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
pub struct MediaDeviceInfo {
    /// Handle to the Dart side `MediaDeviceInfo`.
    handle: DartHandle,

    /// `MediaKind` of this [`MediaDeviceInfo`].
    kind: MediaDeviceKind,
}

impl MediaDeviceInfo {
    /// Returns a unique identifier of the device represented by this
    /// [`MediaDeviceInfo`].
    #[must_use]
    pub fn device_id(&self) -> String {
        unsafe {
            c_str_into_string(media_device_info::device_id(self.handle.get()))
        }
    }

    /// Returns a kind of the device represented by this [`MediaDeviceInfo`].
    #[must_use]
    pub fn kind(&self) -> MediaDeviceKind {
        self.kind
    }

    /// Returns a label describing the device represented by this
    /// [`MediaDeviceInfo`] (for example, "External USB Webcam").
    ///
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            c_str_into_string(media_device_info::label(self.handle.get()))
        }
    }

    /// Returns a group identifier of the device represented by this
    /// [`MediaDeviceInfo`]
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn group_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            media_device_info::group_id(self.handle.get()).unbox()
        })
        .unwrap()
    }
}

impl TryFrom<DartHandle> for MediaDeviceInfo {
    type Error = NotInput;

    fn try_from(value: DartHandle) -> Result<Self, Self::Error> {
        #[allow(clippy::map_err_ignore)]
        let kind = unsafe { media_device_info::kind(value.get()) }
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
#[derive(Clone, Copy, Debug)]
pub struct NotInput;
