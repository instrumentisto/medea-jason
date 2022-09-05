//! WebRTC display source related representations.
use medea_macro::dart_bridge;

use crate::{
    api::dart_string_into_rust, platform::dart::utils::handle::DartHandle,
};

use super::utils::NonNullDartValueArgExt;

#[dart_bridge("flutter/lib/src/native/platform/media_display_info.g.dart")]
mod media_display_info {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    extern "C" {
        /// Returns an unique identifier of the provided display.
        pub fn device_id(info: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns describing the represented display.
        pub fn title(
            info: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
    }
}

#[derive(Clone, Debug)]
/// Representation of a display source.
pub struct MediaDisplayInfo(DartHandle);

impl MediaDisplayInfo {
    /// Returns a unique identifier of the display represented by this
    /// [`MediaDisplayInfo`].
    #[must_use]
    pub fn device_id(&self) -> String {
        unsafe {
            dart_string_into_rust(media_display_info::device_id(self.0.get()))
        }
    }

    /// Returns describing the represented display.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn title(&self) -> Option<String> {
        Option::try_from(unsafe {
            media_display_info::title(self.0.get()).unbox()
        })
        .unwrap()
    }
}

impl TryFrom<DartHandle> for MediaDisplayInfo {
    type Error = NotInput;

    fn try_from(value: DartHandle) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

/// Error of a [MediaDeviceInfo][0] representing not an input device.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
#[derive(Clone, Copy, Debug)]
pub struct NotInput;
