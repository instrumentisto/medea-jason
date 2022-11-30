use std::{os::raw::c_char, ptr};

use super::{propagate_panic, utils::string_into_c_str, ForeignClass};

use crate::{api::DartValueArg, media::MediaDeviceKind};

#[cfg(feature = "mockable")]
pub use self::mock::MediaDeviceInfo;
#[cfg(not(feature = "mockable"))]
pub use crate::platform::MediaDeviceInfo;

impl ForeignClass for MediaDeviceInfo {}

/// Returns unique identifier of the represented device.
#[no_mangle]
pub unsafe extern "C" fn MediaDeviceInfo__device_id(
    this: ptr::NonNull<MediaDeviceInfo>,
) -> ptr::NonNull<c_char> {
    propagate_panic(move || string_into_c_str(this.as_ref().device_id()))
}

/// Returns kind of the represented device.
///
/// This representation of [MediaDeviceInfo][1] ONLY for input device.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
#[no_mangle]
pub unsafe extern "C" fn MediaDeviceInfo__kind(
    this: ptr::NonNull<MediaDeviceInfo>,
) -> MediaDeviceKind {
    propagate_panic(move || this.as_ref().kind())
}

/// Returns label describing the represented device (for example "External USB
/// Webcam").
///
/// If the device has no associated label, then returns an empty string.
#[no_mangle]
pub unsafe extern "C" fn MediaDeviceInfo__label(
    this: ptr::NonNull<MediaDeviceInfo>,
) -> ptr::NonNull<c_char> {
    propagate_panic(move || string_into_c_str(this.as_ref().label()))
}

/// Returns group identifier of the represented device.
///
/// Two devices have the same group identifier if they belong to the same
/// physical device. For example, the audio input and output devices
/// representing the speaker and microphone of the same headset have the
/// same [groupId][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
#[no_mangle]
pub unsafe extern "C" fn MediaDeviceInfo__group_id(
    this: ptr::NonNull<MediaDeviceInfo>,
) -> DartValueArg<Option<String>> {
    propagate_panic(move || DartValueArg::from(this.as_ref().group_id()))
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn MediaDeviceInfo__free(
    this: ptr::NonNull<MediaDeviceInfo>,
) {
    propagate_panic(move || {
        drop(MediaDeviceInfo::from_ptr(this));
    });
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(missing_copy_implementations, clippy::unused_self)]

    use crate::media::MediaDeviceKind;

    #[derive(Debug)]
    pub struct MediaDeviceInfo(pub u8);

    impl MediaDeviceInfo {
        #[must_use]
        pub fn device_id(&self) -> String {
            String::from("MediaDeviceInfo.device_id")
        }

        #[must_use]
        pub const fn kind(&self) -> MediaDeviceKind {
            MediaDeviceKind::AudioInput
        }

        #[must_use]
        pub fn label(&self) -> String {
            String::from("MediaDeviceInfo.label")
        }

        #[must_use]
        pub fn group_id(&self) -> Option<String> {
            Some(String::from("MediaDeviceInfo.group_id"))
        }
    }
}
