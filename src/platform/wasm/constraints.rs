//! Media tracks and streams constraints functionality.

use derive_more::with_trait::{AsRef, Into};
use web_sys::{
    ConstrainBooleanParameters, ConstrainDomStringParameters,
    ConstrainDoubleRange, MediaTrackConstraints,
};

use crate::media::{
    AudioTrackConstraints, DeviceVideoTrackConstraints,
    DisplayVideoTrackConstraints,
    constraints::{ConstrainBoolean, ConstrainString, ConstrainU32},
};

/// [MediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints
#[derive(AsRef, Debug, Into)]
pub struct MediaStreamConstraints(web_sys::MediaStreamConstraints);

impl MediaStreamConstraints {
    /// Creates new [`MediaStreamConstraints`] with none constraints configured.
    #[must_use]
    pub fn new() -> Self {
        Self(web_sys::MediaStreamConstraints::new())
    }

    /// Specifies the nature and settings of the `audio` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn audio(&self, audio: AudioTrackConstraints) {
        self.0.set_audio(&MediaTrackConstraints::from(audio).into());
    }

    /// Specifies the nature and settings of the `video` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn video(&self, video: DeviceVideoTrackConstraints) {
        self.0.set_video(&MediaTrackConstraints::from(video).into());
    }
}

impl Default for MediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AudioTrackConstraints> for MediaTrackConstraints {
    fn from(track_constraints: AudioTrackConstraints) -> Self {
        // Noise suppression level cannot be set via web API, and
        // `googHighPassFilter` has been removed:
        // https://chromium.googlesource.com/chromium/src/+/4a7eeb8c

        let constraints = Self::new();

        if let Some(device_id) = track_constraints.device_id {
            constraints
                .set_device_id(&ConstrainDomStringParameters::from(&device_id));
        }
        if let Some(agc) = track_constraints.auto_gain_control {
            constraints
                .set_auto_gain_control(&ConstrainBooleanParameters::from(agc));
        }
        if let Some(aec) = track_constraints.echo_cancellation {
            constraints
                .set_echo_cancellation(&ConstrainBooleanParameters::from(aec));
        }
        if let Some(ns) = track_constraints.noise_suppression {
            constraints
                .set_noise_suppression(&ConstrainBooleanParameters::from(ns));
        }

        constraints
    }
}

impl From<DeviceVideoTrackConstraints> for MediaTrackConstraints {
    fn from(track_constraints: DeviceVideoTrackConstraints) -> Self {
        let constraints = Self::new();

        if let Some(device_id) = track_constraints.device_id {
            constraints
                .set_device_id(&ConstrainDomStringParameters::from(&device_id));
        }
        if let Some(facing_mode) = track_constraints.facing_mode {
            constraints.set_facing_mode(&ConstrainDomStringParameters::from(
                &facing_mode,
            ));
        }
        if let Some(width) = track_constraints.width {
            constraints.set_width(&ConstrainDoubleRange::from(width));
        }
        if let Some(height) = track_constraints.height {
            constraints.set_height(&ConstrainDoubleRange::from(height));
        }

        constraints
    }
}

impl From<ConstrainU32> for ConstrainDoubleRange {
    fn from(from: ConstrainU32) -> Self {
        let constraint = Self::new();
        match from {
            ConstrainU32::Exact(val) => constraint.set_exact(f64::from(val)),
            ConstrainU32::Ideal(val) => constraint.set_ideal(f64::from(val)),
            ConstrainU32::Range(min, max) => {
                constraint.set_min(f64::from(min));
                constraint.set_max(f64::from(max));
            }
        }
        constraint
    }
}

impl From<ConstrainBoolean> for ConstrainBooleanParameters {
    fn from(from: ConstrainBoolean) -> Self {
        let constraint = Self::new();
        match from {
            ConstrainBoolean::Exact(val) => constraint.set_exact(val),
            ConstrainBoolean::Ideal(val) => constraint.set_ideal(val),
        }
        constraint
    }
}

impl<T: AsRef<str>> From<&ConstrainString<T>> for ConstrainDomStringParameters {
    fn from(from: &ConstrainString<T>) -> Self {
        let constraint = Self::new();
        match from {
            ConstrainString::Exact(val) => constraint
                .set_exact(&wasm_bindgen::JsValue::from_str(val.as_ref())),
            ConstrainString::Ideal(val) => constraint
                .set_ideal(&wasm_bindgen::JsValue::from_str(val.as_ref())),
        }
        constraint
    }
}

/// [DisplayMediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/screen-capture/#dom-displaymediastreamconstraints
#[derive(AsRef, Debug, Into)]
pub struct DisplayMediaStreamConstraints(
    web_sys::DisplayMediaStreamConstraints,
);

impl Default for DisplayMediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayMediaStreamConstraints {
    /// Creates a new [`DisplayMediaStreamConstraints`] with none constraints
    /// configured.
    #[must_use]
    pub fn new() -> Self {
        Self(web_sys::DisplayMediaStreamConstraints::new())
    }

    /// Specifies the nature and settings of the `video` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn video(&self, video: DisplayVideoTrackConstraints) {
        self.0.set_video(&MediaTrackConstraints::from(video).into());
    }

    /// Specifies the nature and settings of the `audio` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub fn audio(&self, audio: AudioTrackConstraints) {
        self.0.set_audio(&MediaTrackConstraints::from(audio).into());
    }
}

impl From<DisplayVideoTrackConstraints> for MediaTrackConstraints {
    fn from(track_constraints: DisplayVideoTrackConstraints) -> Self {
        let constraints = Self::new();

        if let Some(width) = track_constraints.width {
            constraints.set_width(&ConstrainDoubleRange::from(width));
        }
        if let Some(height) = track_constraints.height {
            constraints.set_height(&ConstrainDoubleRange::from(height));
        }
        if let Some(frame_rate) = track_constraints.frame_rate {
            constraints.set_frame_rate(&ConstrainDoubleRange::from(frame_rate));
        }

        constraints
    }
}
