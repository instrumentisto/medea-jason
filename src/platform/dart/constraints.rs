//! Media tracks and streams constraints functionality.

use dart_sys::Dart_Handle;
use derive_more::From;
use medea_macro::dart_bridge;

use crate::{
    media::{
        constraints::{ConstrainString, ConstrainU32},
        AudioTrackConstraints, DeviceVideoTrackConstraints,
        DisplayVideoTrackConstraints,
    },
    platform::dart::utils::{handle::DartHandle, map::DartMap},
};

#[dart_bridge("flutter/lib/src/native/platform/constraints.g.dart")]
mod constraints {
    use dart_sys::Dart_Handle;

    extern "C" {
        /// Creates new [`MediaStreamConstraints`] with none constraints
        /// configured.
        pub fn init() -> Dart_Handle;

        /// Specifies the nature and settings of the `audio`
        /// [MediaStreamTrack][1].
        ///
        /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
        pub fn audio(constraints: Dart_Handle, audio_cons: Dart_Handle);

        /// Specifies the nature and settings of the `video`
        /// [MediaStreamTrack][1].
        ///
        /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
        pub fn video(constraints: Dart_Handle, video_cons: Dart_Handle);
    }
}

/// [MediaTrackConstraints][1] wrapper.
///
/// [1]: https://www.w3.org/TR/mediacapture-streams/#media-track-constraints
pub struct MediaTrackConstraints(DartMap);

impl From<MediaTrackConstraints> for Dart_Handle {
    fn from(from: MediaTrackConstraints) -> Self {
        from.0.into()
    }
}

/// [MediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints
#[derive(Clone, Debug, From)]
pub struct MediaStreamConstraints(DartHandle);

impl From<MediaStreamConstraints> for Dart_Handle {
    fn from(from: MediaStreamConstraints) -> Self {
        from.0.get()
    }
}

impl MediaStreamConstraints {
    /// Creates new [`MediaStreamConstraints`] with none constraints configured.
    #[must_use]
    pub fn new() -> Self {
        unsafe { Self(DartHandle::new(constraints::init())) }
    }

    /// Specifies the nature and settings of the `audio` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn audio(&mut self, audio: AudioTrackConstraints) {
        unsafe {
            constraints::audio(
                self.0.get(),
                MediaTrackConstraints::from(audio).into(),
            );
        }
    }

    /// Specifies the nature and settings of the `video` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn video(&mut self, video: DeviceVideoTrackConstraints) {
        unsafe {
            constraints::video(
                self.0.get(),
                MediaTrackConstraints::from(video).into(),
            );
        }
    }
}

impl Default for MediaStreamConstraints {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// [DisplayMediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/screen-capture/#dom-displaymediastreamconstraints
#[derive(Clone, Debug, From)]
pub struct DisplayMediaStreamConstraints(DartHandle);

impl From<DisplayMediaStreamConstraints> for Dart_Handle {
    fn from(from: DisplayMediaStreamConstraints) -> Self {
        from.0.get()
    }
}

impl Default for DisplayMediaStreamConstraints {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayMediaStreamConstraints {
    /// Creates a new [`DisplayMediaStreamConstraints`] with none constraints
    /// configured.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unsafe { Self(DartHandle::new(constraints::init())) }
    }

    /// Specifies the nature and settings of the `video` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    #[inline]
    pub fn video(&mut self, video: DisplayVideoTrackConstraints) {
        unsafe {
            constraints::video(
                self.0.get(),
                MediaTrackConstraints::from(video).into(),
            );
        }
    }
}

impl From<DisplayVideoTrackConstraints> for MediaTrackConstraints {
    fn from(_: DisplayVideoTrackConstraints) -> Self {
        MediaTrackConstraints(DartMap::new())
    }
}

impl From<AudioTrackConstraints> for MediaTrackConstraints {
    fn from(from: AudioTrackConstraints) -> Self {
        let cons = DartMap::new();
        let audio_cons = DartMap::new();
        let ideal_cons = DartMap::new();
        let exact_cons = DartMap::new();
        if let Some(device_id) = from.device_id {
            match device_id {
                ConstrainString::Exact(device_id) => {
                    exact_cons.set("device_id".to_string(), device_id.into());
                }
                ConstrainString::Ideal(device_id) => {
                    ideal_cons.set("device_id".to_string(), device_id.into());
                }
            }
        }
        audio_cons.set("mandatory".to_string(), exact_cons.as_handle().into());
        audio_cons.set("optional".to_string(), ideal_cons.as_handle().into());
        cons.set("audio".to_string(), audio_cons.as_handle().into());
        MediaTrackConstraints(cons)
    }
}

impl From<DeviceVideoTrackConstraints> for MediaTrackConstraints {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    fn from(from: DeviceVideoTrackConstraints) -> Self {
        let video_cons = DartMap::new();
        let ideal_cons = DartMap::new();
        let exact_cons = DartMap::new();
        if let Some(device_id) = from.device_id {
            match device_id {
                ConstrainString::Exact(device_id) => {
                    ideal_cons.set("sourceId".to_string(), device_id.into());
                }
                ConstrainString::Ideal(device_id) => {
                    exact_cons.set("sourceId".to_string(), device_id.into());
                }
            }
        }
        if let Some(height) = from.height {
            match height {
                ConstrainU32::Ideal(height) => {
                    ideal_cons
                        .set("height".to_string(), (height as i32).into());
                }
                ConstrainU32::Exact(height) => {
                    exact_cons
                        .set("height".to_string(), (height as i32).into());
                }
                ConstrainU32::Range(min, max) => {
                    exact_cons
                        .set("minHeight".to_string(), (min as i32).into());
                    exact_cons
                        .set("maxHeight".to_string(), (max as i32).into());
                }
            }
        }
        if let Some(width) = from.width {
            match width {
                ConstrainU32::Ideal(width) => {
                    ideal_cons.set("width".to_string(), (width as i32).into());
                }
                ConstrainU32::Exact(width) => {
                    exact_cons.set("width".to_string(), (width as i32).into());
                }
                ConstrainU32::Range(min, max) => {
                    exact_cons.set("minWidth".to_string(), (min as i32).into());
                    exact_cons.set("maxWidth".to_string(), (max as i32).into());
                }
            }
        }
        if let Some(facing_mode) = from.facing_mode {
            match facing_mode {
                ConstrainString::Exact(facing_mode) => {
                    exact_cons.set(
                        "facing_mode".to_string(),
                        facing_mode.to_string().into(),
                    );
                }
                ConstrainString::Ideal(facing_mode) => {
                    ideal_cons.set(
                        "facing_mode".to_string(),
                        facing_mode.to_string().into(),
                    );
                }
            }
        }
        video_cons.set("mandatory".to_string(), exact_cons.as_handle().into());
        video_cons.set("optional".to_string(), ideal_cons.as_handle().into());
        let cons = DartMap::new();
        cons.set("video".to_string(), video_cons.as_handle().into());

        MediaTrackConstraints(cons)
    }
}
