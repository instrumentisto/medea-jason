//! Wrapper around a local [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack

use derive_more::with_trait::From;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{
    api::{self, MediaKind, MediaSourceKind, MediaStreamTrackState},
    media::track::local,
};

/// Wrapper around a local [MediaStreamTrack][1].
///
/// Backed by a strong reference to the actual track implementing auto stop on
/// dropping. Can be manually dropped with a `free()` call.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
#[wasm_bindgen]
#[derive(Debug, From)]
pub struct LocalMediaTrack(local::LocalMediaTrackImpl);

#[wasm_bindgen]
impl LocalMediaTrack {
    /// Returns the underlying [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
    #[must_use]
    pub fn get_track(&self) -> web_sys::MediaStreamTrack {
        Clone::clone(self.0.get_track().as_ref())
    }

    /// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
    /// audio track, or a [`MediaKind::Video`] if it represents a video track.
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.0.kind().into()
    }

    /// Returns a [`MediaStreamTrackState::Live`] if this [`LocalMediaTrack`] is
    /// active, or a [`MediaStreamTrackState::Ended`] if it has ended.
    pub fn state(&self) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            Ok(JsValue::from(MediaStreamTrackState::from(this.state().await)))
        })
    }

    /// Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
    /// sourced from some device (webcam/microphone), or a
    /// [`MediaSourceKind::Display`] if it's captured via
    /// [MediaDevices.getDisplayMedia()][1].
    ///
    /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    #[must_use]
    pub fn media_source_kind(&self) -> MediaSourceKind {
        self.0.media_source_kind().into()
    }

    /// Indicates whether an `OnAudioLevelChangedCallback` is supported for this
    /// [`LocalMediaTrack`].
    #[must_use]
    pub fn is_on_audio_level_available(&self) -> bool {
        self.0.is_on_audio_level_available()
    }

    /// Sets the provided function as the callback for the audio level changes
    /// in this [`LocalMediaTrack`].
    ///
    /// # Errors
    ///
    /// If platform call errors.
    pub fn on_audio_level_changed(
        &self,
        cb: js_sys::Function,
    ) -> Result<(), JsValue> {
        self.0
            .on_audio_level_changed(cb.into())
            .map_err(api::Error::from)
            .map_err(Into::into)
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
    #[must_use]
    pub fn is_audio_processing_available(&self) -> bool {
        self.0.is_audio_processing_available()
    }

    /// Toggles noise suppression for this [`LocalMediaTrack`].
    pub fn set_noise_suppression_enabled(&self, enabled: bool) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            this.set_noise_suppression_enabled(enabled)
                .await
                .map_err(api::Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Toggles acoustic echo cancellation for this [`LocalMediaTrack`].
    pub fn set_echo_cancellation_enabled(&self, enabled: bool) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            this.set_echo_cancellation_enabled(enabled)
                .await
                .map_err(api::Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Toggles auto gain control for this [`LocalMediaTrack`].
    pub fn set_auto_gain_control_enabled(&self, enabled: bool) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            this.set_auto_gain_control_enabled(enabled)
                .await
                .map_err(api::Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Indicates whether noise suppression is enabled for this
    /// [`LocalMediaTrack`].
    pub fn is_noise_suppression_enabled(&self) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            let enabled = this
                .is_noise_suppression_enabled()
                .await
                .map_err(api::Error::from)?;
            Ok(JsValue::from(enabled))
        })
    }

    /// Indicates whether auto gain control is enabled for this
    /// [`LocalMediaTrack`].
    pub fn is_auto_gain_control_enabled(&self) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            let enabled = this
                .is_auto_gain_control_enabled()
                .await
                .map_err(api::Error::from)?;
            Ok(JsValue::from(enabled))
        })
    }

    /// Indicates whether echo cancellation is enabled for this
    /// [`LocalMediaTrack`].
    pub fn is_echo_cancellation_enabled(&self) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            let enabled = this
                .is_echo_cancellation_enabled()
                .await
                .map_err(api::Error::from)?;
            Ok(JsValue::from(enabled))
        })
    }
}
