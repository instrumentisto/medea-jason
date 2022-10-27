pub use std::cell::RefCell;

use flutter_rust_bridge::{Opaque, SyncReturn};

pub use crate::api::{
    AudioTrackConstraints, DeviceVideoTrackConstraints,
    DisplayVideoTrackConstraints,
};
pub use crate::media::MediaStreamSettings;

/// Creates new [`MediaStreamSettings`] with none constraints configured.
pub fn media_stream_settings_new(
) -> SyncReturn<Opaque<RefCell<MediaStreamSettings>>> {
    SyncReturn(Opaque::new(RefCell::new(MediaStreamSettings::new())))
}

/// Specifies a nature and settings of an audio [`MediaStreamTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
pub fn media_stream_settings_audio(
    media_stream_settings: Opaque<RefCell<MediaStreamSettings>>,
    constraints: Opaque<AudioTrackConstraints>,
) -> SyncReturn<()> {
    media_stream_settings
        .borrow_mut()
        .audio(AudioTrackConstraints::clone(&constraints));
    SyncReturn(())
}

/// Set constraints for obtaining a local video sourced from a media device.
pub fn media_stream_settings_device_video(
    media_stream_settings: Opaque<RefCell<MediaStreamSettings>>,
    constraints: Opaque<DeviceVideoTrackConstraints>,
) -> SyncReturn<()> {
    media_stream_settings
        .borrow_mut()
        .device_video(DeviceVideoTrackConstraints::clone(&constraints));
    SyncReturn(())
}

/// Set constraints for capturing a local video from user's display.
pub fn media_stream_settings_display_video(
    media_stream_settings: Opaque<RefCell<MediaStreamSettings>>,
    constraints: Opaque<DisplayVideoTrackConstraints>,
) -> SyncReturn<()> {
    media_stream_settings
        .borrow_mut()
        .display_video(DisplayVideoTrackConstraints::clone(&constraints));
    SyncReturn(())
}
