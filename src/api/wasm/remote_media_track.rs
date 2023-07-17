//! Wrapper around a received remote [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack

use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

use crate::{
    api::{MediaDirection, MediaKind, MediaSourceKind},
    media::track::remote,
};

/// Wrapper around a received remote [MediaStreamTrack][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
#[wasm_bindgen]
#[derive(Clone, Debug, From, Into)]
pub struct RemoteMediaTrack(remote::Track);

#[wasm_bindgen]
impl RemoteMediaTrack {
    /// Returns the underlying [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
    #[must_use]
    pub fn get_track(&self) -> web_sys::MediaStreamTrack {
        Clone::clone(self.0.get_track().as_ref())
    }

    /// Indicates whether this [`RemoteMediaTrack`] is muted.
    #[must_use]
    pub fn muted(&self) -> bool {
        self.0.muted()
    }

    /// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
    pub fn on_muted(&self, cb: js_sys::Function) {
        self.0.on_muted(cb.into());
    }

    /// Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
    pub fn on_unmuted(&self, cb: js_sys::Function) {
        self.0.on_unmuted(cb.into());
    }

    /// Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
    pub fn on_stopped(&self, cb: js_sys::Function) {
        self.0.on_stopped(cb.into());
    }

    /// Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
    /// [`MediaDirection`] changes.
    pub fn on_media_direction_changed(&self, cb: js_sys::Function) {
        self.0.on_media_direction_changed(cb.into());
    }

    /// Returns a [`MediaKind::Audio`] if this [`RemoteMediaTrack`] represents
    /// an audio track, or a [`MediaKind::Video`] if it represents a video
    /// track.
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.0.kind().into()
    }

    /// Returns a [`MediaSourceKind::Device`] if this [`RemoteMediaTrack`] is
    /// sourced from some device (webcam/microphone), or a
    /// [`MediaSourceKind::Display`] if it's captured via
    /// [MediaDevices.getDisplayMedia()][1].
    ///
    /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    #[must_use]
    pub fn media_source_kind(&self) -> MediaSourceKind {
        self.0.media_source_kind().into()
    }

    /// Returns the current general [`MediaDirection`] of this
    /// [`RemoteMediaTrack`].
    #[must_use]
    pub fn media_direction(&self) -> MediaDirection {
        self.0.media_direction().into()
    }
}
