//! Wrapper around a [`platform::MediaStreamTrack`] received from a
//! [getUserMedia()][1]/[getDisplayMedia()][2] request.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
//! [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia

use std::rc::Rc;

use derive_more::with_trait::AsRef;
use medea_client_api_proto as proto;
use tracerr::Traced;

use crate::{
    media::{
        AudioLevelError, AudioProcessingError, MediaKind, MediaSourceKind,
        MediaStreamTrackState, NoiseSuppressionLevel,
    },
    platform,
};

/// Wrapper around a [`platform::MediaStreamTrack`] received from a
/// [getUserMedia()][1]/[getDisplayMedia()][2] request.
///
/// Underlying [`platform::MediaStreamTrack`] is stopped on this [`Track`]'s
/// [`Drop`].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
#[derive(AsRef, Debug)]
pub struct Track {
    /// Actual [`platform::MediaStreamTrack`].
    #[as_ref]
    inner: platform::MediaStreamTrack,

    /// Underlying [`platform::MediaStreamTrack`] source kind.
    source_kind: proto::MediaSourceKind,

    /// Reference to the parent [`Track`].
    ///
    /// Parent will be [`None`] if this [`Track`] wasn't forked from another
    /// [`Track`].
    ///
    /// This field is used only for holding strong reference to the parent.
    _parent: Option<Rc<Self>>,
}

impl Track {
    /// Builds a new [`Track`] from the provided [`platform::MediaStreamTrack`]
    /// and [`proto::MediaSourceKind`].
    #[must_use]
    pub const fn new(
        track: platform::MediaStreamTrack,
        source_kind: proto::MediaSourceKind,
    ) -> Self {
        Self { inner: track, source_kind, _parent: None }
    }

    /// Returns the underlying [`platform::MediaStreamTrack`] of this [`Track`].
    #[must_use]
    pub const fn platform_track(&self) -> &platform::MediaStreamTrack {
        &self.inner
    }

    /// Changes [`enabled`][1] attribute on the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    /// Returns [`id`] of underlying [MediaStreamTrack][2].
    ///
    /// [`id`]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn id(&self) -> String {
        self.inner.id()
    }

    /// Returns this [`Track`]'s media source kind.
    #[must_use]
    pub const fn media_source_kind(&self) -> proto::MediaSourceKind {
        self.source_kind
    }

    /// Returns this [`Track`]'s kind (audio/video).
    #[cfg_attr(
        target_family = "wasm",
        expect(clippy::missing_const_for_fn, reason = "not on all platforms")
    )]
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.inner.kind()
    }

    /// Sets a callback to invoke when this [`Track`] is ended.
    pub fn on_ended(&self, callback: platform::Function<()>) {
        self.inner.on_ended(Some(move || callback.call0()));
    }

    /// Returns a [`MediaStreamTrackState::Live`] if this [`Track`] is active,
    /// or a [`MediaStreamTrackState::Ended`] if it has ended.
    pub async fn state(&self) -> MediaStreamTrackState {
        self.inner.ready_state().await
    }

    /// Forks this [`Track`].
    ///
    /// Creates a new [`Track`] from this [`Track`]'s
    /// [`platform::MediaStreamTrack`] using a [`clone()`][1] method.
    ///
    /// Forked [`Track`] will hold a strong reference to this [`Track`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-clone
    pub async fn fork(self: &Rc<Self>) -> Self {
        let parent = Rc::clone(self);
        let track = self.inner.fork().await;
        Self {
            inner: track,
            source_kind: self.source_kind,
            _parent: Some(parent),
        }
    }

    /// [Stops][1] this [`Track`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-stop
    pub async fn stop(&self) {
        self.inner.stop().await;
    }
}

impl Drop for Track {
    fn drop(&mut self) {
        platform::spawn(Box::pin(self.inner.stop()));
    }
}

/// Strongly referenced [`Track`] received from a
/// [getUserMedia()][1]/[getDisplayMedia()][2] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
#[derive(Debug, Clone)]
pub struct LocalMediaTrackImpl(Rc<Track>);

impl LocalMediaTrackImpl {
    /// Creates a new [`LocalMediaTrackImpl`] from the provided [`Track`].
    #[must_use]
    pub const fn new(track: Rc<Track>) -> Self {
        Self(track)
    }

    /// Returns the underlying [`platform::MediaStreamTrack`] of this
    /// [`LocalMediaTrackImpl`].
    #[must_use]
    pub fn get_track(&self) -> &platform::MediaStreamTrack {
        &self.0.inner
    }

    /// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrackImpl`]
    /// represents an audio track, or a [`MediaKind::Video`] if it represents
    /// a video track.
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.0.kind()
    }

    /// Sets a callback to invoke when this [`LocalMediaTrackImpl`] is ended.
    pub fn on_ended(&self, callback: platform::Function<()>) {
        self.0.on_ended(callback);
    }

    /// Returns a [`MediaStreamTrackState::Live`] if
    /// this [`LocalMediaTrackImpl`] is active, or
    /// a [`MediaStreamTrackState::Ended`] if it has ended.
    pub async fn state(&self) -> MediaStreamTrackState {
        self.0.state().await
    }

    /// Indicates whether an `OnAudioLevelChangedCallback` is supported for this
    /// [`LocalMediaTrackImpl`].
    #[must_use]
    pub fn is_on_audio_level_available(&self) -> bool {
        self.0.inner.is_on_audio_level_available()
    }

    /// Sets the provided `OnAudioLevelChangedCallback` for this
    /// [`LocalMediaTrackImpl`].
    ///
    /// It's called for live [`LocalMediaTrackImpl`]s when their audio level
    /// changes.
    ///
    /// # Errors
    ///
    /// With an [`AudioLevelError`] if platform call errors.
    pub fn on_audio_level_changed(
        &self,
        callback: platform::Function<i32>,
    ) -> Result<(), Traced<AudioLevelError>> {
        self.0
            .inner
            .on_audio_level_changed(move |v| callback.call1(v))
            .map_err(AudioLevelError::from)
            .map_err(tracerr::wrap!())
    }

    /// Indicates whether this [`LocalMediaTrackImpl`] supports audio processing
    /// functions:
    /// - [`LocalMediaTrackImpl::is_noise_suppression_enabled()`]
    /// - [`LocalMediaTrackImpl::set_noise_suppression_enabled()`]
    /// - [`LocalMediaTrackImpl::get_noise_suppression_level()`]
    /// - [`LocalMediaTrackImpl::set_noise_suppression_level()`]
    /// - [`LocalMediaTrackImpl::is_echo_cancellation_enabled()`]
    /// - [`LocalMediaTrackImpl::set_echo_cancellation_enabled()`]
    /// - [`LocalMediaTrackImpl::is_auto_gain_control_enabled()`]
    /// - [`LocalMediaTrackImpl::set_auto_gain_control_enabled()`]
    /// - [`LocalMediaTrackImpl::is_high_pass_filter_enabled()`]
    /// - [`LocalMediaTrackImpl::set_high_pass_filter_enabled()`]
    #[must_use]
    pub fn is_audio_processing_available(&self) -> bool {
        self.0.inner.is_audio_processing_available()
    }

    /// Toggles noise suppression for this [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn set_noise_suppression_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), Traced<AudioProcessingError>> {
        self.0
            .inner
            .set_noise_suppression_enabled(enabled)
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Configures a [`NoiseSuppressionLevel`] for this [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn set_noise_suppression_level(
        &self,
        level: NoiseSuppressionLevel,
    ) -> Result<(), Traced<AudioProcessingError>> {
        self.0
            .inner
            .set_noise_suppression_level(level)
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Toggles acoustic echo cancellation for this [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn set_echo_cancellation_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), Traced<AudioProcessingError>> {
        self.0
            .inner
            .set_echo_cancellation_enabled(enabled)
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Toggles auto gain control for this [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn set_auto_gain_control_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), Traced<AudioProcessingError>> {
        self.0
            .inner
            .set_auto_gain_control_enabled(enabled)
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Toggles high-pass filter for this [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn set_high_pass_filter_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), Traced<AudioProcessingError>> {
        self.0
            .inner
            .set_high_pass_filter_enabled(enabled)
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Indicates whether noise suppression is enabled for this
    /// [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn is_noise_suppression_enabled(
        &self,
    ) -> Result<bool, Traced<AudioProcessingError>> {
        self.0
            .inner
            .is_noise_suppression_enabled()
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Returns the current configured [`NoiseSuppressionLevel`] of this
    /// [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn get_noise_suppression_level(
        &self,
    ) -> Result<NoiseSuppressionLevel, Traced<AudioProcessingError>> {
        self.0
            .inner
            .get_noise_suppression_level()
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Indicates whether auto gain control is enabled for this
    /// [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn is_auto_gain_control_enabled(
        &self,
    ) -> Result<bool, Traced<AudioProcessingError>> {
        self.0
            .inner
            .is_auto_gain_control_enabled()
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Indicates whether echo cancellation is enabled for this
    /// [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn is_echo_cancellation_enabled(
        &self,
    ) -> Result<bool, Traced<AudioProcessingError>> {
        self.0
            .inner
            .is_echo_cancellation_enabled()
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Indicates whether high-pass filter is enabled for this
    /// [`LocalMediaTrackImpl`].
    ///
    /// # Errors
    ///
    /// With an [`AudioProcessingError`] if platform call errors.
    pub async fn is_high_pass_filter_enabled(
        &self,
    ) -> Result<bool, Traced<AudioProcessingError>> {
        self.0
            .inner
            .is_high_pass_filter_enabled()
            .await
            .map_err(AudioProcessingError::from)
            .map_err(tracerr::wrap!())
    }

    /// Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrackImpl`] is
    /// sourced from some device (webcam/microphone), or
    /// a [`MediaSourceKind::Display`] if it's captured via
    /// [MediaDevices.getDisplayMedia()][1].
    ///
    /// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    #[must_use]
    pub fn media_source_kind(&self) -> MediaSourceKind {
        self.0.media_source_kind().into()
    }

    /// [Stops][1] this [`LocalMediaTrackImpl`] if this is the last wrapper for
    /// the underlying [`Track`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-stop
    pub async fn maybe_stop(mut self) {
        if let Some(track) = Rc::get_mut(&mut self.0) {
            track.stop().await;
        }
    }
}
