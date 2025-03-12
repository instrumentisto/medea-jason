//! Wrapper around a local [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack

use derive_more::with_trait::From;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use super::Error;
use crate::{
    api::{MediaKind, MediaSourceKind},
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
pub struct LocalMediaTrack(local::LocalMediaTrack);

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

    /// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
    /// audio track, or a [`MediaKind::Video`] if it represents a video track.
    // TODO: Try remove on next Rust upgrade.
    #[expect(clippy::allow_attributes, reason = "`#[expect]` doesn't work")]
    // TODO: Needs refactoring.
    #[allow(clippy::as_conversions, reason = "needs refactoring")]
    pub fn state(&self) -> Promise {
        let this = self.0.clone();
        future_to_promise(
            async move { Ok(JsValue::from(this.state().await as u8)) },
        )
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

    /// Sets the provided `OnAudioLevelChangedCallback` for this
    /// [`LocalMediaTrack`].
    ///
    /// It's called for live [`LocalMediaTrack`]s once their audio level
    /// changes.
    pub fn on_audio_level_changed(
        &self,
        cb: js_sys::Function,
    ) -> Result<(), JsValue> {
        self.0
            .on_audio_level_changed(cb.into())
            .map_err(Error::from)
            .map_err(Into::into)
    }
}
