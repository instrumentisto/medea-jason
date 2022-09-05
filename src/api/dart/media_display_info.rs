use std::{os::raw::c_char, ptr};

use super::{propagate_panic, utils::string_into_c_str, ForeignClass};

use crate::api::DartValueArg;

#[cfg(feature = "mockable")]
pub use self::mock::MediaDisplayInfo;
#[cfg(not(feature = "mockable"))]
pub use crate::platform::MediaDisplayInfo;

impl ForeignClass for MediaDisplayInfo {}

/// Returns unique identifier of the represented display.
#[no_mangle]
pub unsafe extern "C" fn MediaDisplayInfo__device_id(
    this: ptr::NonNull<MediaDisplayInfo>,
) -> ptr::NonNull<c_char> {
    propagate_panic(move || string_into_c_str(this.as_ref().device_id()))
}

/// Returns describing the represented display.
#[no_mangle]
pub unsafe extern "C" fn MediaDisplayInfo__title(
    this: ptr::NonNull<MediaDisplayInfo>,
) -> DartValueArg<Option<String>> {
    propagate_panic(move || DartValueArg::from(this.as_ref().title()))
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn MediaDisplayInfo__free(
    this: ptr::NonNull<MediaDisplayInfo>,
) {
    propagate_panic(move || {
        drop(MediaDisplayInfo::from_ptr(this));
    });
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(missing_copy_implementations, clippy::unused_self)]

    #[derive(Debug)]
    pub struct MediaDisplayInfo(pub u8);

    impl MediaDisplayInfo {
        #[must_use]
        pub fn device_id(&self) -> String {
            String::from("MediaDisplayInfo.device_id")
        }

        #[must_use]
        pub fn title(&self) -> Option<String> {
            Some(String::from("MediaDisplayInfo.title"))
        }
    }
}
