//! Representations of [MediaTrackConstraints][0] and
//! [MediaStreamConstraints][1].
//!
//! [0]: https://w3.org/TR/mediacapture-streams#media-track-constraints
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints

use dart_sys::Dart_Handle;
use derive_more::with_trait::From;
use medea_macro::dart_bridge;

use crate::{
    api::DartValue,
    media::{
        DeviceAudioTrackConstraints, DeviceVideoTrackConstraints,
        DisplayAudioTrackConstraints, DisplayVideoTrackConstraints,
        constraints::{ConstrainBoolean, ConstrainString, ConstrainU32},
    },
    platform::dart::utils::handle::DartHandle,
};

#[dart_bridge("flutter/lib/src/native/platform/constraints.g.dart")]
mod constraints {
    use dart_sys::Dart_Handle;

    use crate::{api::DartValue, platform::Error};

    extern "C" {
        /// Initializes new empty [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        pub fn init_device_constraints() -> Result<Dart_Handle, Error>;

        /// Initializes new empty [MediaStreamConstraints][0] for display.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        pub fn init_display_constraints() -> Result<Dart_Handle, Error>;

        /// Initializes a new empty [MediaStreamConstraints.video][0].
        ///
        /// [0]: https://tinyurl.com/3yvnbb9e
        pub fn new_video_constraints() -> Result<Dart_Handle, Error>;

        /// Initializes a new empty [MediaStreamConstraints.audio][0].
        ///
        /// [0]: https://tinyurl.com/5bmrr4w5
        pub fn new_audio_constraints() -> Result<Dart_Handle, Error>;

        /// Specifies the provided setting of a
        /// [MediaStreamConstraints.video][0] (for example `facingMode`).
        ///
        /// [0]: https://tinyurl.com/3yvnbb9e
        pub fn set_video_constraint_value(
            constraints: Dart_Handle,
            kind: i64,
            value: DartValue,
        ) -> Result<(), Error>;

        /// Specifies the provided setting of a
        /// [MediaStreamConstraints.audio][0] (for example `deviceId`).
        ///
        /// [0]: https://tinyurl.com/5bmrr4w5
        pub fn set_audio_constraint_value(
            constraints: Dart_Handle,
            kind: i64,
            value: DartValue,
        ) -> Result<(), Error>;

        /// Specifies the provided nature and settings of a `video`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_video_constraint(
            constraints: Dart_Handle,
            ty: i64,
            video: Dart_Handle,
        ) -> Result<(), Error>;

        /// Specifies the provided nature and settings of a display `video`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_display_video_constraint(
            constraints: Dart_Handle,
            ty: i64,
            video: Dart_Handle,
        ) -> Result<(), Error>;

        /// Specifies the provided nature and settings of a device `audio`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_device_audio_constraint(
            constraints: Dart_Handle,
            ty: i64,
            audio: Dart_Handle,
        ) -> Result<(), Error>;

        /// Specifies the provided nature and settings of a display `audio`
        /// [MediaStreamTrack][1] to the given [MediaStreamConstraints][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_display_audio_constraint(
            constraints: Dart_Handle,
            ty: i64,
            video: Dart_Handle,
        ) -> Result<(), Error>;
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
    AutoGainControl = 1,
    NoiseSuppression = 2,
    NoiseSuppressionLevel = 3,
    HighPassFilter = 4,
    EchoCancellation = 5,
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
        let constraints =
            unsafe { constraints::init_device_constraints() }.unwrap();
        unsafe { Self(DartHandle::new(constraints)) }
    }

    /// Specifies the provided nature and settings of an `audio`
    /// [MediaStreamTrack][1] to these [`MediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn audio(&mut self, audio: DeviceAudioTrackConstraints) {
        let audio = MediaTrackConstraints::from(audio);
        unsafe {
            constraints::set_device_audio_constraint(
                self.0.get(),
                ConstraintType::Mandatory as i64,
                audio.mandatory.get(),
            )
        }
        .unwrap();
        unsafe {
            constraints::set_device_audio_constraint(
                self.0.get(),
                ConstraintType::Optional as i64,
                audio.optional.get(),
            )
        }
        .unwrap();
    }

    /// Specifies the provided nature and settings of a `video`
    /// [MediaStreamTrack][1] to these [`MediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn video(&mut self, video: DeviceVideoTrackConstraints) {
        let video = MediaTrackConstraints::from(video);
        unsafe {
            constraints::set_video_constraint(
                self.0.get(),
                ConstraintType::Mandatory as i64,
                video.mandatory.get(),
            )
        }
        .unwrap();
        unsafe {
            constraints::set_video_constraint(
                self.0.get(),
                ConstraintType::Optional as i64,
                video.optional.get(),
            )
        }
        .unwrap();
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
        let constraints =
            unsafe { constraints::init_display_constraints() }.unwrap();
        unsafe { Self(DartHandle::new(constraints)) }
    }

    /// Specifies the provided nature and settings of a `video`
    /// [MediaStreamTrack][1] to these [`DisplayMediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn video(&mut self, video: DisplayVideoTrackConstraints) {
        let video = MediaTrackConstraints::from(video);
        unsafe {
            constraints::set_display_video_constraint(
                self.0.get(),
                ConstraintType::Mandatory as i64,
                video.mandatory.get(),
            )
        }
        .unwrap();
        unsafe {
            constraints::set_display_video_constraint(
                self.0.get(),
                ConstraintType::Optional as i64,
                video.optional.get(),
            )
        }
        .unwrap();
    }

    /// Specifies the provided nature and settings of a `audio`
    /// [MediaStreamTrack][1] to these [`DisplayMediaStreamConstraints`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn audio(&mut self, audio: DisplayAudioTrackConstraints) {
        let audio = MediaTrackConstraints::from(audio);
        unsafe {
            constraints::set_display_audio_constraint(
                self.0.get(),
                ConstraintType::Mandatory as i64,
                audio.mandatory.get(),
            )
        }
        .unwrap();
        unsafe {
            constraints::set_display_audio_constraint(
                self.0.get(),
                ConstraintType::Optional as i64,
                audio.optional.get(),
            )
        }
        .unwrap();
    }
}

#[expect(clippy::fallible_impl_from, reason = "FFI error is unexpected")]
impl From<DeviceAudioTrackConstraints> for MediaTrackConstraints {
    fn from(from: DeviceAudioTrackConstraints) -> Self {
        let optional = {
            let audio =
                unsafe { constraints::new_audio_constraints() }.unwrap();
            unsafe { DartHandle::new(audio) }
        };
        let mandatory = {
            let audio =
                unsafe { constraints::new_audio_constraints() }.unwrap();
            unsafe { DartHandle::new(audio) }
        };
        let parse_bool_const = |c: ConstrainBoolean| -> (&DartHandle, bool) {
            match c {
                ConstrainBoolean::Exact(val) => (&mandatory, val),
                ConstrainBoolean::Ideal(val) => (&optional, val),
            }
        };

        if let Some(device_id) = from.device_id {
            match device_id {
                ConstrainString::Exact(device_id) => unsafe {
                    constraints::set_audio_constraint_value(
                        mandatory.get(),
                        AudioConstraintKind::DeviceId as i64,
                        DartValue::from(device_id),
                    )
                }
                .unwrap(),
                ConstrainString::Ideal(device_id) => unsafe {
                    constraints::set_audio_constraint_value(
                        optional.get(),
                        AudioConstraintKind::DeviceId as i64,
                        DartValue::from(device_id),
                    )
                }
                .unwrap(),
            }
        }
        if let Some(agc) = from.auto_gain_control {
            let (kind, val) = parse_bool_const(agc);

            unsafe {
                constraints::set_audio_constraint_value(
                    kind.get(),
                    AudioConstraintKind::AutoGainControl as i64,
                    DartValue::from(val),
                )
                .unwrap();
            };
        }
        if let Some(aec) = from.echo_cancellation {
            let (kind, val) = parse_bool_const(aec);

            unsafe {
                constraints::set_audio_constraint_value(
                    kind.get(),
                    AudioConstraintKind::EchoCancellation as i64,
                    DartValue::from(val),
                )
                .unwrap();
            }
        }
        if let Some(hpf) = from.high_pass_filter {
            let (kind, val) = parse_bool_const(hpf);

            unsafe {
                constraints::set_audio_constraint_value(
                    kind.get(),
                    AudioConstraintKind::HighPassFilter as i64,
                    DartValue::from(val),
                )
                .unwrap();
            }
        }
        if let Some(ns) = from.noise_suppression {
            let (kind, val) = parse_bool_const(ns);

            unsafe {
                constraints::set_audio_constraint_value(
                    kind.get(),
                    AudioConstraintKind::NoiseSuppression as i64,
                    DartValue::from(val),
                )
                .unwrap();
            }
        }
        if let Some(nsl) = from.noise_suppression_level {
            unsafe {
                constraints::set_audio_constraint_value(
                    optional.get(),
                    AudioConstraintKind::NoiseSuppressionLevel as i64,
                    DartValue::from(nsl as i64),
                )
                .unwrap();
            }
        }

        Self { optional, mandatory }
    }
}

#[expect(clippy::fallible_impl_from, reason = "FFI error is unexpected")]
impl From<DisplayAudioTrackConstraints> for MediaTrackConstraints {
    fn from(_: DisplayAudioTrackConstraints) -> Self {
        let optional = {
            let audio =
                unsafe { constraints::new_audio_constraints() }.unwrap();
            unsafe { DartHandle::new(audio) }
        };
        let mandatory = {
            let audio =
                unsafe { constraints::new_audio_constraints() }.unwrap();
            unsafe { DartHandle::new(audio) }
        };

        Self { optional, mandatory }
    }
}

#[expect(clippy::fallible_impl_from, reason = "FFI error is unexpected")]
impl From<DeviceVideoTrackConstraints> for MediaTrackConstraints {
    fn from(from: DeviceVideoTrackConstraints) -> Self {
        let optional = {
            let video =
                unsafe { constraints::new_video_constraints() }.unwrap();
            unsafe { DartHandle::new(video) }
        };
        let mandatory = {
            let video =
                unsafe { constraints::new_video_constraints() }.unwrap();
            unsafe { DartHandle::new(video) }
        };

        if let Some(device_id) = from.device_id {
            unsafe {
                set_constrain_string(
                    device_id,
                    VideoConstraintKind::DeviceId,
                    &optional,
                    &mandatory,
                );
            }
        }
        if let Some(facing_mode) = from.facing_mode {
            match facing_mode {
                ConstrainString::Exact(facing_mode) => unsafe {
                    constraints::set_video_constraint_value(
                        mandatory.get(),
                        VideoConstraintKind::FacingMode as i64,
                        DartValue::from(facing_mode as i64),
                    )
                }
                .unwrap(),
                ConstrainString::Ideal(facing_mode) => unsafe {
                    constraints::set_video_constraint_value(
                        optional.get(),
                        VideoConstraintKind::FacingMode as i64,
                        DartValue::from(facing_mode as i64),
                    )
                }
                .unwrap(),
            }
        }
        if let Some(width) = from.width {
            unsafe {
                set_video_constrain_u32(
                    width,
                    VideoConstraintKind::Width,
                    &optional,
                    &mandatory,
                );
            }
        }
        if let Some(height) = from.height {
            unsafe {
                set_video_constrain_u32(
                    height,
                    VideoConstraintKind::Height,
                    &optional,
                    &mandatory,
                );
            }
        }

        Self { optional, mandatory }
    }
}

#[expect(clippy::fallible_impl_from, reason = "FFI error is unexpected")]
impl From<DisplayVideoTrackConstraints> for MediaTrackConstraints {
    fn from(from: DisplayVideoTrackConstraints) -> Self {
        let optional = {
            let video =
                unsafe { constraints::new_video_constraints() }.unwrap();
            unsafe { DartHandle::new(video) }
        };
        let mandatory = {
            let video =
                unsafe { constraints::new_video_constraints() }.unwrap();
            unsafe { DartHandle::new(video) }
        };

        if let Some(device_id) = from.device_id {
            unsafe {
                set_constrain_string(
                    device_id,
                    VideoConstraintKind::DeviceId,
                    &optional,
                    &mandatory,
                );
            }
        }
        if let Some(width) = from.width {
            unsafe {
                set_video_constrain_u32(
                    width,
                    VideoConstraintKind::Width,
                    &optional,
                    &mandatory,
                );
            }
        }
        if let Some(height) = from.height {
            unsafe {
                set_video_constrain_u32(
                    height,
                    VideoConstraintKind::Height,
                    &optional,
                    &mandatory,
                );
            }
        }
        if let Some(frame_rate) = from.frame_rate {
            unsafe {
                set_video_constrain_u32(
                    frame_rate,
                    VideoConstraintKind::FrameRate,
                    &optional,
                    &mandatory,
                );
            }
        }

        Self { optional, mandatory }
    }
}

/// Applies the specified [`ConstrainString`] to the provided  `optional` and
/// `mandatory` [`DartHandle`]s representing the Dart side constraints.
unsafe fn set_constrain_string<T>(
    constrain: ConstrainString<T>,
    kind: VideoConstraintKind,
    optional: &DartHandle,
    mandatory: &DartHandle,
) where
    DartValue: From<T>,
{
    match constrain {
        ConstrainString::Exact(val) => unsafe {
            constraints::set_video_constraint_value(
                mandatory.get(),
                kind as i64,
                DartValue::from(val),
            )
        }
        .unwrap(),
        ConstrainString::Ideal(val) => unsafe {
            constraints::set_video_constraint_value(
                optional.get(),
                kind as i64,
                DartValue::from(val),
            )
        }
        .unwrap(),
    }
}

/// Applies the specified [`ConstrainU32`] to the provided  `optional` and
/// `mandatory` [`DartHandle`]s representing the Dart side constraints.
unsafe fn set_video_constrain_u32(
    constrain: ConstrainU32,
    kind: VideoConstraintKind,
    optional: &DartHandle,
    mandatory: &DartHandle,
) {
    match constrain {
        ConstrainU32::Ideal(val) => unsafe {
            constraints::set_video_constraint_value(
                optional.get(),
                kind as i64,
                DartValue::from(val),
            )
        }
        .unwrap(),
        ConstrainU32::Exact(val) => unsafe {
            constraints::set_video_constraint_value(
                mandatory.get(),
                kind as i64,
                DartValue::from(val),
            )
        }
        .unwrap(),
        ConstrainU32::Range(min, _) => unsafe {
            // TODO: Implement range constraints in `medea_flutter_webrtc`.
            constraints::set_video_constraint_value(
                mandatory.get(),
                kind as i64,
                DartValue::from(min),
            )
        }
        .unwrap(),
    }
}
