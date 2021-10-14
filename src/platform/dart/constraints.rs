//! Media tracks and streams constraints functionality.

use derive_more::AsRef;

use crate::media::{
    AudioTrackConstraints, DeviceVideoTrackConstraints,
    DisplayVideoTrackConstraints,
};

/// [MediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints
#[derive(AsRef, Debug)]
pub struct MediaStreamConstraints;

impl MediaStreamConstraints {
    /// Creates a new [`MediaStreamConstraints`] with none constraints
    /// configured.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Specifies the nature and settings of the `audio` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    #[inline]
    pub fn audio(&mut self, _audio: AudioTrackConstraints) {
        unimplemented!()
    }

    /// Specifies the nature and settings of the `video` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    #[inline]
    pub fn video(&mut self, _video: DeviceVideoTrackConstraints) {
        unimplemented!()
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
#[derive(AsRef, Debug)]
pub struct DisplayMediaStreamConstraints();

impl Default for DisplayMediaStreamConstraints {
    #[inline]
    fn default() -> Self {
        unimplemented!()
    }
}

impl DisplayMediaStreamConstraints {
    /// Creates a new [`DisplayMediaStreamConstraints`] with none constraints
    /// configured.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Specifies the nature and settings of the `video` [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    #[inline]
    pub fn video(&mut self, _video: DisplayVideoTrackConstraints) {
        unimplemented!()
    }
}
