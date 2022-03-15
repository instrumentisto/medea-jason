//! Constraints applicable to audio tracks.

use std::{os::raw::c_char, ptr};
use std::panic::{catch_unwind, UnwindSafe};

use super::{utils::c_str_into_string, ForeignClass, panic_catcher};

use crate::api::utils::new_panic_error;
use crate::platform;
use crate::platform::utils::dart_api::Dart_PropagateError_DL_Trampolined;

pub use crate::media::AudioTrackConstraints;

impl ForeignClass for AudioTrackConstraints {}

/// Creates new [`AudioTrackConstraints`] with none constraints configured.
#[no_mangle]
pub extern "C" fn AudioTrackConstraints__new(
) -> ptr::NonNull<AudioTrackConstraints> {
    panic_catcher(|| {
        AudioTrackConstraints::new().into_ptr()
    })
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
#[no_mangle]
pub unsafe extern "C" fn AudioTrackConstraints__device_id(
    mut this: ptr::NonNull<AudioTrackConstraints>,
    device_id: ptr::NonNull<c_char>,
) {
    panic_catcher(move || {
        this.as_mut().device_id(c_str_into_string(device_id));
    })
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn AudioTrackConstraints__free(
    this: ptr::NonNull<AudioTrackConstraints>,
) {
    panic_catcher(move || {
        drop(AudioTrackConstraints::from_ptr(this));
    })
}
