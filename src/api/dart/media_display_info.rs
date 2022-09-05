use std::{os::raw::c_char, ptr};

use super::{propagate_panic, utils::string_into_c_str, ForeignClass};

use crate::api::DartValueArg;

pub use crate::platform::MediaDisplayInfo;

impl ForeignClass for MediaDisplayInfo {}

/// Returns unique identifier of the represented display.
#[no_mangle]
pub unsafe extern "C" fn MediaDisplayInfo__device_id(
    this: ptr::NonNull<MediaDisplayInfo>,
) -> ptr::NonNull<c_char> {
    propagate_panic(move || string_into_c_str(this.as_ref().device_id()))
}

//todo
#[no_mangle]
pub unsafe extern "C" fn MediaDisplayInfo__title(
    this: ptr::NonNull<MediaDisplayInfo>,
) -> DartValueArg<Option<String>> {
    // propagate_panic(move || DartValueArg::from(this.as_ref().group_id()))

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