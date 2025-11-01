//! WebRTC display source related representations.

use derive_more::with_trait::From;
use medea_macro::dart_bridge;

use super::utils::{NonNullDartValueArgExt as _, dart_string_into_rust};
use crate::platform::dart::utils::handle::DartHandle;

#[dart_bridge("flutter/lib/src/native/platform/media_display_info.g.dart")]
mod media_display_info {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

    extern "C" {
        /// Returns a unique identifier of the provided display.
        pub fn device_id(
            info: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns a title describing the represented display.
        pub fn title(
            info: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<String>>>, Error>;
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
        let device_id =
            unsafe { media_display_info::device_id(self.0.get()) }.unwrap();
        unsafe { dart_string_into_rust(device_id) }
    }

    /// Returns a title describing the represented display.
    #[must_use]
    pub fn title(&self) -> Option<String> {
        let title = unsafe { media_display_info::title(self.0.get()) }.unwrap();
        Option::try_from(unsafe { title.unbox() }).unwrap()
    }
}
