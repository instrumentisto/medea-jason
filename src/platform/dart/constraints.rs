//! Representations of [MediaTrackConstraints][0] and
//! [MediaStreamConstraints][1].
//!
//! [0]: https://w3.org/TR/mediacapture-streams#media-track-constraints
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints

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
        /// Initializes new empty [MediaStreamConstraints][1].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        pub fn init() -> Dart_Handle;

        /// Specifies the provided nature and settings of an `audio`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn audio(constraints: Dart_Handle, audio_cons: Dart_Handle);

        /// Specifies the provided nature and settings of a `video`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn video(constraints: Dart_Handle, video_cons: Dart_Handle);
    }
}

/// Dart side representation of [MediaTrackConstraints][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#media-track-constraints
pub struct MediaTrackConstraints(DartMap);

impl From<MediaTrackConstraints> for Dart_Handle {
    fn from(from: MediaTrackConstraints) -> Self {
        from.0.into()
    }
}

/// Dart side representation of [MediaStreamConstraints][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
#[derive(Clone, Debug, From)]
pub struct MediaStreamConstraints(DartHandle);

impl From<MediaStreamConstraints> for Dart_Handle {
    fn from(from: MediaStreamConstraints) -> Self {
        from.0.get()
    }
}

impl Default for MediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaStreamConstraints {
    /// Creates new empty [`MediaStreamConstraints`].
    #[must_use]
    pub fn new() -> Self {
        unsafe { Self(DartHandle::new(constraints::init())) }
    }

    /// Specifies the provided nature and settings of an `audio`
    /// [MediaStreamTrack][1] to these [`MediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    pub fn audio(&mut self, audio: AudioTrackConstraints) {
        unsafe {
            constraints::audio(
                self.0.get(),
                MediaTrackConstraints::from(audio).into(),
            );
        }
    }

    /// Specifies the provided nature and settings of a `video`
    /// [MediaStreamTrack][1] to these [`MediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    pub fn video(&mut self, video: DeviceVideoTrackConstraints) {
        unsafe {
            constraints::video(
                self.0.get(),
                MediaTrackConstraints::from(video).into(),
            );
        }
    }
}

/// Dart side representation of [DisplayMediaStreamConstraints][0].
///
/// [0]: https://w3.org/TR/screen-capture#dom-displaymediastreamconstraints
#[derive(Clone, Debug, From)]
pub struct DisplayMediaStreamConstraints(DartHandle);

impl From<DisplayMediaStreamConstraints> for Dart_Handle {
    fn from(from: DisplayMediaStreamConstraints) -> Self {
        from.0.get()
    }
}

impl Default for DisplayMediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayMediaStreamConstraints {
    /// Creates new empty [`DisplayMediaStreamConstraints`] .
    #[must_use]
    pub fn new() -> Self {
        unsafe { Self(DartHandle::new(constraints::init())) }
    }

    /// Specifies the provided nature and settings of a `video`
    /// [MediaStreamTrack][1] to these [`DisplayMediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
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
        Self(DartMap::new())
    }
}

impl From<AudioTrackConstraints> for MediaTrackConstraints {
    fn from(from: AudioTrackConstraints) -> Self {
        let mut audio_cons = DartMap::new();
        let mut ideal_cons = DartMap::new();
        let mut exact_cons = DartMap::new();
        if let Some(device_id) = from.device_id {
            match device_id {
                ConstrainString::Exact(device_id) => {
                    exact_cons.set("device_id".to_owned(), device_id.into());
                }
                ConstrainString::Ideal(device_id) => {
                    ideal_cons.set("device_id".to_owned(), device_id.into());
                }
            }
        }
        audio_cons.set("mandatory".to_owned(), exact_cons.as_handle().into());
        audio_cons.set("optional".to_owned(), ideal_cons.as_handle().into());

        let mut cons = DartMap::new();
        cons.set("audio".to_owned(), audio_cons.as_handle().into());

        Self(cons)
    }
}

impl From<DeviceVideoTrackConstraints> for MediaTrackConstraints {
    fn from(from: DeviceVideoTrackConstraints) -> Self {
        let mut video_cons = DartMap::new();
        let mut ideal_cons = DartMap::new();
        let mut exact_cons = DartMap::new();
        if let Some(device_id) = from.device_id {
            match device_id {
                ConstrainString::Exact(device_id) => {
                    ideal_cons.set("sourceId".to_owned(), device_id.into());
                }
                ConstrainString::Ideal(device_id) => {
                    exact_cons.set("sourceId".to_owned(), device_id.into());
                }
            }
        }
        if let Some(height) = from.height {
            match height {
                ConstrainU32::Ideal(height) => {
                    ideal_cons.set("height".to_owned(), height.into());
                }
                ConstrainU32::Exact(height) => {
                    exact_cons.set("height".to_owned(), height.into());
                }
                ConstrainU32::Range(min, max) => {
                    exact_cons.set("minHeight".to_owned(), min.into());
                    exact_cons.set("maxHeight".to_owned(), max.into());
                }
            }
        }
        if let Some(width) = from.width {
            match width {
                ConstrainU32::Ideal(width) => {
                    ideal_cons.set("width".to_owned(), width.into());
                }
                ConstrainU32::Exact(width) => {
                    exact_cons.set("width".to_owned(), width.into());
                }
                ConstrainU32::Range(min, max) => {
                    exact_cons.set("minWidth".to_owned(), min.into());
                    exact_cons.set("maxWidth".to_owned(), max.into());
                }
            }
        }
        if let Some(facing_mode) = from.facing_mode {
            match facing_mode {
                ConstrainString::Exact(facing_mode) => {
                    exact_cons.set(
                        "facing_mode".to_owned(),
                        facing_mode.to_string().into(),
                    );
                }
                ConstrainString::Ideal(facing_mode) => {
                    ideal_cons.set(
                        "facing_mode".to_owned(),
                        facing_mode.to_string().into(),
                    );
                }
            }
        }
        video_cons.set("mandatory".to_owned(), exact_cons.as_handle().into());
        video_cons.set("optional".to_owned(), ideal_cons.as_handle().into());

        let mut cons = DartMap::new();
        cons.set("video".to_owned(), video_cons.as_handle().into());

        Self(cons)
    }
}
