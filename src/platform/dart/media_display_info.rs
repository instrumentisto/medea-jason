//! WebRTC display source related representations.

use derive_more::From;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::handle::DartHandle;

use super::utils::{dart_string_into_rust, NonNullDartValueArgExt as _};

#[dart_bridge("flutter/lib/src/native/platform/media_display_info.g.dart")]
mod media_display_info {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    extern "C" {
        /// Returns a unique identifier of the provided display.
        pub fn device_id(info: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns a title describing the represented display.
        pub fn title(
            info: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
    }
}

/// Representation of a display source.
#[derive(Clone, Debug, From)]
pub struct MediaDisplayInfo(DartHandle);

impl MediaDisplayInfo {
    /// Returns a unique identifier of the display represented by this
    /// [`MediaDisplayInfo`].
    #[must_use]
    pub fn device_id(&self) -> String {
        let device_id = unsafe { media_display_info::device_id(self.0.get()) };
        unsafe { dart_string_into_rust(device_id) }
    }

    /// Returns a title describing the represented display.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn title(&self) -> Option<String> {
        let title = unsafe { media_display_info::title(self.0.get()) };
        Option::try_from(unsafe { title.unbox() }).unwrap()
    }
}
