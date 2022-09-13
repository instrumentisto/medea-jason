//! Representations of [MediaTrackConstraints][0] and
//! [MediaStreamConstraints][1].
//!
//! [0]: https://w3.org/TR/mediacapture-streams#media-track-constraints
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints

use dart_sys::Dart_Handle;
use derive_more::From;
use medea_macro::dart_bridge;

use crate::{
    api::DartValue,
    media::{
        constraints::{ConstrainString, ConstrainU32},
        AudioTrackConstraints, DeviceVideoTrackConstraints,
        DisplayVideoTrackConstraints,
    },
    platform::dart::utils::handle::DartHandle,
};

#[dart_bridge("flutter/lib/src/native/platform/constraints.g.dart")]
mod constraints {
    use dart_sys::Dart_Handle;

    use crate::api::DartValue;

    extern "C" {
        /// Initializes new empty [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        pub fn init_device_constraints() -> Dart_Handle;

        /// Initializes new empty [MediaStreamConstraints][0] for display.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        pub fn init_display_constraints() -> Dart_Handle;

        /// Initializes a new empty [MediaStreamConstraints.video][0].
        ///
        /// [0]: https://tinyurl.com/3yvnbb9e
        pub fn new_video_constraints() -> Dart_Handle;

        /// Initializes a new empty [MediaStreamConstraints.audio][0].
        ///
        /// [0]: https://tinyurl.com/5bmrr4w5
        pub fn new_audio_constraints() -> Dart_Handle;

        /// Specifies the provided setting of a
        /// [MediaStreamConstraints.video][0] (for example `facingMode`).
        ///
        /// [0]: https://tinyurl.com/3yvnbb9e
        pub fn set_video_constraint_value(
            constraints: Dart_Handle,
            kind: i64,
            value: DartValue,
        );

        /// Specifies the provided setting of a
        /// [MediaStreamConstraints.audio][0] (for example `deviceId`).
        ///
        /// [0]: https://tinyurl.com/5bmrr4w5
        pub fn set_audio_constraint_value(
            constraints: Dart_Handle,
            kind: i64,
            value: DartValue,
        );

        /// Specifies the provided nature and settings of a `video`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_video_constraint(
            constraints: Dart_Handle,
            ty: i64,
            video: Dart_Handle,
        );

        /// Specifies the provided nature and settings of an `audio`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_audio_constraint(
            constraints: Dart_Handle,
            ty: i64,
            audio: Dart_Handle,
        );
    }
}

/// Kind of a [MediaStreamConstraints.video][0] setting.
///
/// [0]: https://tinyurl.com/3yvnbb9e
enum VideoConstraintKind {
    FacingMode = 0,
    DeviceId = 1,
    Width = 2,
    Height = 3,
    FrameRate = 4,
}

/// Kind of a [MediaStreamConstraints.audio][0] setting.
///
/// [0]: https://tinyurl.com/5bmrr4w5
enum AudioConstraintKind {
    DeviceId = 0,
}

/// Indicator of necessity of a [MediaStreamConstraints] setting.
///
/// [0]: https://www.w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints
enum ConstraintType {
    /// Not necessary, so if the device doesn't fit to the provided constraint,
    /// it still can be used.
    Optional = 0,

    /// Necessary, so if the device doesn't fit to the provided constraint, it
    /// can't be used.
    Mandatory = 1,
}

/// Dart side representation of [MediaTrackConstraints][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#media-track-constraints
#[derive(Debug)]
pub struct MediaTrackConstraints {
    /// Optional setting, so if the device doesn't fit to the provided
    /// constraint, it still can be used.
    optional: DartHandle,

    /// Necessary setting, so if the device doesn't fit to the provided
    /// constraint, it can't be used.
    mandatory: DartHandle,
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
        unsafe { Self(DartHandle::new(constraints::init_device_constraints())) }
    }

    /// Specifies the provided nature and settings of an `audio`
    /// [MediaStreamTrack][1] to these [`MediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn audio(&mut self, audio: AudioTrackConstraints) {
        unsafe {
            let audio = MediaTrackConstraints::from(audio);
            constraints::set_audio_constraint(
                self.0.get(),
                ConstraintType::Mandatory as i64,
                audio.mandatory.get(),
            );
            constraints::set_audio_constraint(
                self.0.get(),
                ConstraintType::Optional as i64,
                audio.optional.get(),
            );
        }
    }

    /// Specifies the provided nature and settings of a `video`
    /// [MediaStreamTrack][1] to these [`MediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn video(&mut self, video: DeviceVideoTrackConstraints) {
        unsafe {
            let video = MediaTrackConstraints::from(video);
            constraints::set_video_constraint(
                self.0.get(),
                ConstraintType::Mandatory as i64,
                video.mandatory.get(),
            );
            constraints::set_video_constraint(
                self.0.get(),
                ConstraintType::Optional as i64,
                video.optional.get(),
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
        unsafe {
            Self(DartHandle::new(constraints::init_display_constraints()))
        }
    }

    /// Specifies the provided nature and settings of a `video`
    /// [MediaStreamTrack][1] to these [`DisplayMediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn video(&mut self, video: DisplayVideoTrackConstraints) {
        let video = MediaTrackConstraints::from(video);
        unsafe {
            constraints::set_video_constraint(
                self.0.get(),
                ConstraintType::Mandatory as i64,
                video.mandatory.get(),
            );
            constraints::set_video_constraint(
                self.0.get(),
                ConstraintType::Optional as i64,
                video.optional.get(),
            );
        }
    }
}

impl From<AudioTrackConstraints> for MediaTrackConstraints {
    fn from(from: AudioTrackConstraints) -> Self {
        unsafe {
            let optional =
                DartHandle::new(constraints::new_audio_constraints());
            let mandatory =
                DartHandle::new(constraints::new_audio_constraints());
            if let Some(device_id) = from.device_id {
                match device_id {
                    ConstrainString::Exact(device_id) => {
                        constraints::set_audio_constraint_value(
                            mandatory.get(),
                            AudioConstraintKind::DeviceId as i64,
                            DartValue::from(device_id),
                        );
                    }
                    ConstrainString::Ideal(device_id) => {
                        constraints::set_audio_constraint_value(
                            optional.get(),
                            AudioConstraintKind::DeviceId as i64,
                            DartValue::from(device_id),
                        );
                    }
                }
            }

            Self {
                optional,
                mandatory,
            }
        }
    }
}

impl From<DeviceVideoTrackConstraints> for MediaTrackConstraints {
    fn from(from: DeviceVideoTrackConstraints) -> Self {
        unsafe {
            let optional =
                DartHandle::new(constraints::new_video_constraints());
            let mandatory =
                DartHandle::new(constraints::new_video_constraints());

            if let Some(device_id) = from.device_id {
                match device_id {
                    ConstrainString::Exact(device_id) => {
                        constraints::set_video_constraint_value(
                            optional.get(),
                            VideoConstraintKind::DeviceId as i64,
                            DartValue::from(device_id),
                        );
                    }
                    ConstrainString::Ideal(device_id) => {
                        constraints::set_video_constraint_value(
                            mandatory.get(),
                            VideoConstraintKind::DeviceId as i64,
                            DartValue::from(device_id),
                        );
                    }
                }
            }

            if let Some(facing_mode) = from.facing_mode {
                match facing_mode {
                    ConstrainString::Exact(facing_mode) => {
                        constraints::set_video_constraint_value(
                            mandatory.get(),
                            VideoConstraintKind::FacingMode as i64,
                            DartValue::from(facing_mode as i64),
                        );
                    }
                    ConstrainString::Ideal(facing_mode) => {
                        constraints::set_video_constraint_value(
                            optional.get(),
                            VideoConstraintKind::FacingMode as i64,
                            DartValue::from(facing_mode as i64),
                        );
                    }
                }
            }

            if let Some(width) = from.width {
                set_video_constrain_u32(
                    width,
                    VideoConstraintKind::Width,
                    &optional,
                    &mandatory,
                );
            }

            if let Some(height) = from.height {
                set_video_constrain_u32(
                    height,
                    VideoConstraintKind::Height,
                    &optional,
                    &mandatory,
                );
            }

            Self {
                optional,
                mandatory,
            }
        }
    }
}

impl From<DisplayVideoTrackConstraints> for MediaTrackConstraints {
    fn from(from: DisplayVideoTrackConstraints) -> Self {
        unsafe {
            let optional =
                DartHandle::new(constraints::new_video_constraints());
            let mandatory =
                DartHandle::new(constraints::new_video_constraints());

            if let Some(width) = from.width {
                set_video_constrain_u32(
                    width,
                    VideoConstraintKind::Width,
                    &optional,
                    &mandatory,
                );
            }

            if let Some(height) = from.height {
                set_video_constrain_u32(
                    height,
                    VideoConstraintKind::Height,
                    &optional,
                    &mandatory,
                );
            }

            if let Some(frame_rate) = from.frame_rate {
                set_video_constrain_u32(
                    frame_rate,
                    VideoConstraintKind::FrameRate,
                    &optional,
                    &mandatory,
                );
            }

            Self {
                optional,
                mandatory,
            }
        }
    }
}

/// Applies the given [`ConstrainU32`] to the `optional` and `mandatory`
/// [`DartHandle`]s that represent the `Dart`-side constraint.
unsafe fn set_video_constrain_u32(
    constrain: ConstrainU32,
    kind: VideoConstraintKind,
    optional: &DartHandle,
    mandatory: &DartHandle,
) {
    match constrain {
        ConstrainU32::Ideal(val) => {
            constraints::set_video_constraint_value(
                optional.get(),
                kind as i64,
                DartValue::from(val),
            );
        }
        ConstrainU32::Exact(val) => {
            constraints::set_video_constraint_value(
                mandatory.get(),
                kind as i64,
                DartValue::from(val),
            );
        }
        ConstrainU32::Range(min, max) => {
            constraints::set_video_constraint_value(
                mandatory.get(),
                kind as i64,
                DartValue::from(min),
            );
        }
    }
}
