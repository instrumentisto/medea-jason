use std::{os::raw::c_char, ptr};

use super::{
    propagate_panic,
    utils::{c_str_into_string, DartResult},
    ArgumentError, ForeignClass,
};

pub use crate::media::DisplayVideoTrackConstraints;

impl ForeignClass for DisplayVideoTrackConstraints {}

/// Creates new [`DisplayVideoTrackConstraints`] with none constraints
/// configured.
#[no_mangle]
pub extern "C" fn DisplayVideoTrackConstraints__new(
) -> ptr::NonNull<DisplayVideoTrackConstraints> {
    propagate_panic(|| DisplayVideoTrackConstraints::new().into_ptr())
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__exact_height(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    height: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(height) = u32::try_from(height) else {
            return ArgumentError::new(height, "height", "Expected u32")
                .into();
        };
        this.as_mut().exact_height(height);
        Ok(()).into()
    })
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__ideal_height(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    height: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(height) = u32::try_from(height) else {
            return ArgumentError::new(height, "height", "Expected u32")
                .into();
        };
        this.as_mut().ideal_height(height);
        Ok(()).into()
    })
}

/// Sets an ideal [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__ideal_frame_rate(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    frame_rate: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(rate) = u32::try_from(frame_rate) else {
            return ArgumentError::new(frame_rate, "frame_rate", "Expected u32")
                .into();
        };
        this.as_mut().ideal_frame_rate(rate);
        Ok(()).into()
    })
}

/// Sets an exact [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__exact_frame_rate(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    frame_rate: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(rate) = u32::try_from(frame_rate) else {
            return ArgumentError::new(frame_rate, "frame_rate", "Expected u32")
                .into();
        };
        this.as_mut().exact_frame_rate(rate);
        Ok(()).into()
    })
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__exact_width(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    width: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(width) = u32::try_from(width) else {
            return ArgumentError::new(width, "width", "Expected u32")
                .into();
        };
        this.as_mut().exact_width(width);
        Ok(()).into()
    })
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__ideal_width(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    width: i64,
) -> DartResult {
    propagate_panic(|| {
        let Ok(width) = u32::try_from(width) else {
            return ArgumentError::new(width, "width", "Expected u32")
                .into();
        };
        this.as_mut().ideal_width(width);
        Ok(()).into()
    })
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__device_id(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    device_id: ptr::NonNull<c_char>,
) {
    propagate_panic(move || {
        this.as_mut().device_id(c_str_into_string(device_id));
    });
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__free(
    this: ptr::NonNull<DisplayVideoTrackConstraints>,
) {
    propagate_panic(move || {
        drop(DisplayVideoTrackConstraints::from_ptr(this));
    });
}
