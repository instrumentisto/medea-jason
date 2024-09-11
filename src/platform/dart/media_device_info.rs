//! [MediaDeviceInfo][0] related representations.
//!
//! [0]: https://w3.org/TR/mediacapture-streams#device-info

use medea_macro::dart_bridge;

use crate::{
    media::MediaDeviceKind,
    platform::dart::utils::{
        dart_string_into_rust, handle::DartHandle, NonNullDartValueArgExt,
    },
};

#[dart_bridge("flutter/lib/src/native/platform/media_device_info.g.dart")]
mod media_device_info {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

    extern "C" {
        /// Returns an unique identifier of the provided device.
        pub fn device_id(
            info: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns a kind of the provided device.
        pub fn kind(info: Dart_Handle) -> Result<i64, Error>;

        /// Returns a label describing the provided device (for example,
        /// "External USB Webcam").
        ///
        /// If the provided device has no associated label, then returns an
        /// empty string.
        pub fn label(info: Dart_Handle) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns a group identifier of the provided device.
        pub fn group_id(
            info: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<String>>>, Error>;

        /// Indicates whether the last attempt to use the provided device
        /// failed.
        pub fn is_failed(info: Dart_Handle) -> Result<bool, Error>;
    }
}

/// Representation of a [MediaDeviceInfo][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
#[derive(Clone, Debug)]
pub struct MediaDeviceInfo {
    /// Handle to the Dart side `MediaDeviceInfo`.
    handle: DartHandle,

    /// [`MediaDeviceKind`] of this [`MediaDeviceInfo`].
    kind: MediaDeviceKind,
}

impl MediaDeviceInfo {
    /// Returns a unique identifier of the device represented by this
    /// [`MediaDeviceInfo`].
    #[must_use]
    pub fn device_id(&self) -> String {
        let device_id =
            unsafe { media_device_info::device_id(self.handle.get()) }.unwrap();
        unsafe { dart_string_into_rust(device_id) }
    }

    /// Returns a kind of the device represented by this [`MediaDeviceInfo`].
    #[must_use]
    pub const fn kind(&self) -> MediaDeviceKind {
        self.kind
    }

    /// Returns a label describing the device represented by this
    /// [`MediaDeviceInfo`] (for example, "External USB Webcam").
    ///
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn label(&self) -> String {
        let label =
            unsafe { media_device_info::label(self.handle.get()) }.unwrap();
        unsafe { dart_string_into_rust(label) }
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
    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    #[must_use]
    pub fn group_id(&self) -> Option<String> {
        let group_id =
            unsafe { media_device_info::group_id(self.handle.get()) }.unwrap();
        Option::try_from(unsafe { group_id.unbox() }).unwrap()
    }

    /// Indicates whether the last attempt to use this device failed.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        unsafe { media_device_info::is_failed(self.handle.get()) }.unwrap()
    }
}

impl TryFrom<DartHandle> for MediaDeviceInfo {
    type Error = NotInput;

    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    fn try_from(value: DartHandle) -> Result<Self, Self::Error> {
        #[expect(clippy::map_err_ignore, reason = "not useful")]
        let kind = unsafe { media_device_info::kind(value.get()) }
            .unwrap()
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
