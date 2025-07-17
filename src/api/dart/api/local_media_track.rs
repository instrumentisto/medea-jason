//! Strongly referenced [`local::Track`] received from a
//! [getUserMedia()][1]/[getDisplayMedia()][2] request.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
//! [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia

use flutter_rust_bridge::{DartOpaque, frb};
use send_wrapper::SendWrapper;

#[cfg(doc)]
use crate::media::track::local;
use crate::{
    api::{DART_HANDLER_PORT, Error, dart::api::ForeignClass},
    media::{
        MediaKind, MediaSourceKind, NoiseSuppressionLevel, track::local as core,
    },
    platform::{self, utils::dart_future::IntoDartFuture as _},
};

/// Strongly referenced [`local::Track`] received from a
/// [getUserMedia()][1]/[getDisplayMedia()][2] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
#[derive(Debug)]
#[frb(opaque)]
pub struct LocalMediaTrack(SendWrapper<core::LocalMediaTrackImpl>);

impl From<core::LocalMediaTrackImpl> for LocalMediaTrack {
    fn from(value: core::LocalMediaTrackImpl) -> Self {
        Self(SendWrapper::new(value))
    }
}

impl ForeignClass for LocalMediaTrack {}

impl LocalMediaTrack {
    /// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of the
    /// provided [`LocalMediaTrack`].
    ///
    /// [`MediaStreamTrack`]: platform::MediaStreamTrack
    #[frb(sync)]
    #[must_use]
    pub fn get_track(&self) -> DartOpaque {
        DartOpaque::new(
            self.0.get_track().handle() as _,
            DART_HANDLER_PORT
                .get()
                .expect("`DART_HANDLER_PORT` must be initialized"),
        )
    }

    /// Returns a [`MediaKind::Audio`] if the provided [`LocalMediaTrack`]
    /// represents an audio track, or a [`MediaKind::Video`] if it represents a
    /// video track.
    #[frb(sync)]
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.0.kind()
    }

    /// Sets callback to be invoked once this [`LocalMediaTrack`] is ended.
    #[frb(sync)]
    #[must_use]
    pub fn on_ended(&self, f: DartOpaque) {
        self.0.on_ended(platform::Function::new(f));
    }

    /// Returns a [`media::MediaStreamTrackState::Live`] if this
    /// [`LocalMediaTrack`] is active, or a
    /// [`media::MediaStreamTrackState::Ended`] if it has ended.
    #[frb(sync)]
    #[must_use]
    pub fn state(&self) -> DartOpaque {
        let track = self.0.clone();

        async move { Ok::<_, Error>(track.state().await as i64) }
            .into_dart_future()
            .into_dart_opaque()
    }

    /// Indicates whether an `OnAudioLevelChangedCallback` is supported for this
    /// [`LocalMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn is_on_audio_level_available(&self) -> bool {
        self.0.is_on_audio_level_available()
    }

    /// Sets the provided `OnAudioLevelChangedCallback` for this
    /// [`LocalMediaTrack`].
    ///
    /// It's called for live [`LocalMediaTrack`]s once their audio level
    /// changes.
    #[frb(sync)]
    #[must_use]
    pub fn on_audio_level_changed(&self, f: DartOpaque) {
        self.0
            .on_audio_level_changed(platform::Function::new(f))
            .unwrap_or_else(|e| {
                unreachable!("cannot error on non-WASM platforms, but did: {e}")
            })
    }

    /// Returns a [`MediaSourceKind::Device`] if the provided
    /// [`LocalMediaTrack`] is sourced from some device
    /// (webcam/microphone), or a [`MediaSourceKind::Display`] if it's
    /// captured via [MediaDevices.getDisplayMedia()][1].
    ///
    /// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    #[frb(sync)]
    #[must_use]
    pub fn media_source_kind(&self) -> MediaSourceKind {
        self.0.media_source_kind()
    }

    /// Indicates whether this [`LocalMediaTrack`] supports audio processing
    /// functions:
    /// - [`LocalMediaTrack::is_noise_suppression_enabled()`]
    /// - [`LocalMediaTrack::set_noise_suppression_enabled()`]
    /// - [`LocalMediaTrack::get_noise_suppression_level()`]
    /// - [`LocalMediaTrack::set_noise_suppression_level()`]
    /// - [`LocalMediaTrack::is_echo_cancellation_enabled()`]
    /// - [`LocalMediaTrack::set_echo_cancellation_enabled()`]
    /// - [`LocalMediaTrack::is_auto_gain_control_enabled()`]
    /// - [`LocalMediaTrack::set_auto_gain_control_enabled()`]
    /// - [`LocalMediaTrack::is_high_pass_filter_enabled()`]
    /// - [`LocalMediaTrack::set_high_pass_filter_enabled()`]
    #[frb(sync)]
    #[must_use]
    pub fn is_audio_processing_available(&self) -> bool {
        self.0.is_audio_processing_available()
    }

    /// Toggles noise suppression for this [`LocalMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn set_noise_suppression_enabled(&self, enabled: bool) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.set_noise_suppression_enabled(enabled)
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Configures a [`NoiseSuppressionLevel`] for this [`LocalMediaTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    #[frb(sync)]
    #[must_use]
    pub fn set_noise_suppression_level(
        &self,
        level: NoiseSuppressionLevel,
    ) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.set_noise_suppression_level(level).await.map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Toggles acoustic echo cancellation for this [`LocalMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn set_echo_cancellation_enabled(&self, enabled: bool) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.set_echo_cancellation_enabled(enabled)
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Toggles auto gain control for this [`LocalMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn set_auto_gain_control_enabled(&self, enabled: bool) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.set_auto_gain_control_enabled(enabled)
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Toggles high-pass filter for this [`LocalMediaTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    #[frb(sync)]
    #[must_use]
    pub fn set_high_pass_filter_enabled(&self, enabled: bool) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.set_high_pass_filter_enabled(enabled)
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Indicates whether noise suppression is enabled for this
    /// [`LocalMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn is_noise_suppression_enabled(&self) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.is_noise_suppression_enabled()
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Returns the current configured [`NoiseSuppressionLevel`] of this
    /// [`LocalMediaTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    #[frb(sync)]
    #[must_use]
    pub fn get_noise_suppression_level(&self) -> DartOpaque {
        let this = self.0.clone();
        async move {
            let lvl = this
                .get_noise_suppression_level()
                .await
                .map_err(Error::from)?;

            Ok::<_, Error>(lvl as i64)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Indicates whether auto gain control is enabled for this
    /// [`LocalMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn is_auto_gain_control_enabled(&self) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.is_auto_gain_control_enabled()
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Indicates whether echo cancellation is enabled for this
    /// [`LocalMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn is_echo_cancellation_enabled(&self) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.is_echo_cancellation_enabled()
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Indicates whether high-pass filter is enabled for this
    /// [`LocalMediaTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    #[frb(sync)]
    #[must_use]
    pub fn is_high_pass_filter_enabled(&self) -> DartOpaque {
        let this = self.0.clone();
        async move {
            this.is_high_pass_filter_enabled()
                .await
                .map_err(Error::from)
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Frees the data behind the provided opaque local track.
    #[frb(sync)]
    #[must_use]
    pub fn free(self) -> DartOpaque {
        async move {
            self.0.take().maybe_stop().await;
            Ok::<_, Error>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }
}

/// Returns the [`Vec<LocalMediaTrack>`] from the provided [`ForeignClass`]
/// address.
#[frb(sync, type_64bit_int)]
#[must_use]
pub fn vec_local_tracks_from_raw(ptr: usize) -> Vec<LocalMediaTrack> {
    unsafe { Vec::<LocalMediaTrack>::from_ptr(ptr).into_iter().collect() }
}
