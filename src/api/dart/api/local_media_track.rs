use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    ptr,
};

use derive_more::From;
use flutter_rust_bridge::{frb, DartOpaque};

use crate::{
    api::{utils::new_dart_opaque, Error, ForeignClass},
    media::{track::local as core, MediaKind, MediaSourceKind},
    platform::{self, utils::dart_future::IntoDartFuture},
};

#[derive(Debug, From)]
#[frb(opaque)]
pub struct LocalMediaTrack(core::LocalMediaTrack);

impl LocalMediaTrack {
    /// Returns the [`LocalMediaTrack`] from the [`ForeignClass`] address.
    #[frb(sync, type_64bit_int)]
    #[must_use]
    pub fn from_raw(ptr: usize) -> LocalMediaTrack {
        unsafe {
            LocalMediaTrack::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
        }
    }

    /// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of the
    /// provided [`LocalMediaTrack`].
    ///
    /// [`MediaStreamTrack`]: platform::MediaStreamTrack
    #[frb(sync)]
    #[must_use]
    pub fn get_track(&self) -> DartOpaque {
        unsafe { new_dart_opaque(self.0.get_track().handle()) }
    }

    /// Returns a [`MediaKind::Audio`] if the provided [`LocalMediaTrack`]
    /// represents an audio track, or a [`MediaKind::Video`] if it represents a
    /// video track.
    #[frb(sync)]
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.0.kind()
    }

    /// Sets callback to invoke when this [`LocalMediaTrack`] is ended.
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
    /// It's called for live [`LocalMediaTrack`]s when their audio level
    /// changes.
    #[frb(sync)]
    #[must_use]
    pub fn on_audio_level_changed(&self, f: DartOpaque) {
        self.0.on_audio_level_changed(platform::Function::new(f));
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

    /// Frees the data behind the provided opaque local track.
    #[frb(sync)]
    #[must_use]
    pub fn free(self) -> DartOpaque {
        let track = self.0.clone();

        async move {
            track.maybe_stop().await;
            Ok::<_, Error>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }
}

/// Returns the [`Vec<LocalMediaTrack>`] from the
/// [`ForeignClass`] address.
#[frb(sync, type_64bit_int)]
#[must_use]
pub fn vec_local_tracks_from_raw(ptr: usize) -> Vec<LocalMediaTrack> {
    unsafe {
        Vec::<LocalMediaTrack>::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
            .into_iter()
            .collect()
    }
}

impl ForeignClass for LocalMediaTrack {}
unsafe impl Send for LocalMediaTrack {}
unsafe impl Sync for LocalMediaTrack {}
