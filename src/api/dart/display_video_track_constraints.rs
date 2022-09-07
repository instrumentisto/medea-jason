use std::ptr;

use super::{propagate_panic, utils::DartResult, ArgumentError, ForeignClass};

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
        match u32::try_from(height) {
            Ok(h) => this.as_mut().exact_height(h),
            Err(_) => {
                return ArgumentError::new(height, "height", "Expected u32")
                    .into();
            }
        };
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
        match u32::try_from(height) {
            Ok(h) => this.as_mut().ideal_height(h),
            Err(_) => {
                return ArgumentError::new(height, "height", "Expected u32")
                    .into();
            }
        };
        Ok(()).into()
    })
}

/// Sets an ideal [frameRate][1] constraint.
///
/// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-framerate
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__ideal_frame_rate(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    frame_rate: i64,
) -> DartResult {
    propagate_panic(move || {
        match u32::try_from(frame_rate) {
            Ok(h) => this.as_mut().ideal_frame_rate(h),
            Err(_) => {
                return ArgumentError::new(
                    frame_rate,
                    "frame_rate",
                    "Expected u32",
                )
                .into();
            }
        };
        Ok(()).into()
    })
}

/// Sets an exact [frameRate][1] constraint.
///
/// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-framerate
#[no_mangle]
pub unsafe extern "C" fn DisplayVideoTrackConstraints__exact_frame_rate(
    mut this: ptr::NonNull<DisplayVideoTrackConstraints>,
    frame_rate: i64,
) -> DartResult {
    propagate_panic(move || {
        match u32::try_from(frame_rate) {
            Ok(h) => this.as_mut().exact_frame_rate(h),
            Err(_) => {
                return ArgumentError::new(
                    frame_rate,
                    "frame_rate",
                    "Expected u32",
                )
                .into();
            }
        };
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
        match u32::try_from(width) {
            Ok(w) => this.as_mut().exact_width(w),
            Err(_) => {
                return ArgumentError::new(width, "width", "Expected u32")
                    .into();
            }
        };
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
        match u32::try_from(width) {
            Ok(w) => this.as_mut().ideal_width(w),
            Err(_) => {
                return ArgumentError::new(width, "width", "Expected u32")
                    .into();
            }
        };
        Ok(()).into()
    })
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
        let _ = DisplayVideoTrackConstraints::from_ptr(this);
    });
}
