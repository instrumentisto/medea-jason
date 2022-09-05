//! [MediaDeviceInfo][0] related representations.
//!
//! [0]: https://w3.org/TR/mediacapture-streams#device-info
//todo
use medea_macro::dart_bridge;

use crate::{
    api::dart_string_into_rust,
    platform::dart::utils::handle::DartHandle,
};

#[dart_bridge("flutter/lib/src/native/platform/media_display_info.g.dart")]
mod media_display_info {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    extern "C" {
        /// Returns an unique identifier of the provided device.
        pub fn device_id(info: Dart_Handle) -> ptr::NonNull<c_char>;

        pub fn title(info: Dart_Handle) -> ptr::NonNull<DartValueArg<Option<String>>>;
    }
}

#[derive(Clone, Debug)]
/// todo
pub struct MediaDisplayInfo(DartHandle);

impl MediaDisplayInfo {
    /// Returns a unique identifier of the device represented by this
    /// [`MediaDisplayInfo`].
    #[must_use]
    pub fn device_id(&self) -> String {
        unsafe {
            dart_string_into_rust(media_display_info::device_id(
                self.0.get(),
            ))
        }
    }

    /// Returns the title of this [`MediaDisplayInfo`].
    ///
    /// # Panics
    ///
    /// Panics if .
    #[must_use]
    pub fn title(&self) -> Option<String> {
        unsafe {
            let title = media_display_info::title(self.0.get());
            (*Box::from_raw(title.as_ptr())).try_into().unwrap()
        }
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
