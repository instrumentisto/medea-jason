use std::ptr;

use super::{
    audio_track_constraints::AudioTrackConstraints,
    device_video_track_constraints::DeviceVideoTrackConstraints,
    display_video_track_constraints::DisplayVideoTrackConstraints,
    propagate_panic, ForeignClass,
};

pub use crate::media::MediaStreamSettings;

impl ForeignClass for MediaStreamSettings {}

/// Creates new [`MediaStreamSettings`] with none constraints configured.
#[no_mangle]
#[rustfmt::skip]
pub extern "C" fn MediaStreamSettings__new()
    -> ptr::NonNull<MediaStreamSettings>
{
    propagate_panic(|| {
        MediaStreamSettings::new().into_ptr()
    })
}

/// Specifies a nature and settings of an audio [`MediaStreamTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
#[no_mangle]
pub unsafe extern "C" fn MediaStreamSettings__audio(
    mut this: ptr::NonNull<MediaStreamSettings>,
    constraints: ptr::NonNull<AudioTrackConstraints>,
) {
    propagate_panic(move || {
        this.as_mut()
            .audio(AudioTrackConstraints::from_ptr(constraints));
    });
}

/// Set constraints for obtaining a local video sourced from a media device.
#[no_mangle]
pub unsafe extern "C" fn MediaStreamSettings__device_video(
    mut this: ptr::NonNull<MediaStreamSettings>,
    constraints: ptr::NonNull<DeviceVideoTrackConstraints>,
) {
    propagate_panic(move || {
        this.as_mut()
            .device_video(DeviceVideoTrackConstraints::from_ptr(constraints));
    });
}

/// Set constraints for capturing a local video from user's display.
#[no_mangle]
pub unsafe extern "C" fn MediaStreamSettings__display_video(
    mut this: ptr::NonNull<MediaStreamSettings>,
    constraints: ptr::NonNull<DisplayVideoTrackConstraints>,
) {
    propagate_panic(move || {
        this.as_mut()
            .display_video(DisplayVideoTrackConstraints::from_ptr(constraints));
    });
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn MediaStreamSettings__free(
    this: ptr::NonNull<MediaStreamSettings>,
) {
    propagate_panic(move || {
        drop(MediaStreamSettings::from_ptr(this));
    });
}
