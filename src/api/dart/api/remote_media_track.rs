//! Wrapper around a received remote [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack

use flutter_rust_bridge::{frb, DartOpaque};
use send_wrapper::SendWrapper;

use crate::{
    api::{api::DART_HANDLER_PORT, dart::api::ForeignClass, MediaDirection},
    media::{track::remote as core, MediaKind, MediaSourceKind},
    platform,
};

/// Wrapper around a received remote [MediaStreamTrack][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
#[derive(Debug)]
#[frb(opaque)]
pub struct RemoteMediaTrack(SendWrapper<core::Track>);

impl From<core::Track> for RemoteMediaTrack {
    fn from(value: core::Track) -> Self {
        Self(SendWrapper::new(value))
    }
}

impl ForeignClass for RemoteMediaTrack {}

impl RemoteMediaTrack {
    /// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
    /// [`RemoteMediaTrack`].
    ///
    /// [`MediaStreamTrack`]: platform::MediaStreamTrack
    #[frb(sync)]
    #[must_use]
    pub fn get_track(&self) -> DartOpaque {
        DartOpaque::new(
            self.0.get_track().handle() as _,
            DART_HANDLER_PORT.get().unwrap(),
        )
    }

    /// Sets callback to invoke once this [`RemoteMediaTrack`] is muted.
    #[frb(sync)]
    #[must_use]
    pub fn on_muted(&self, f: DartOpaque) {
        self.0.on_muted(platform::Function::new(f));
    }

    /// Sets callback to invoke once this [`RemoteMediaTrack`] is unmuted.
    #[frb(sync)]
    #[must_use]
    pub fn on_unmuted(&self, f: DartOpaque) {
        self.0.on_unmuted(platform::Function::new(f));
    }

    /// Sets callback to invoke once this [`RemoteMediaTrack`] is stopped.
    #[frb(sync)]
    #[must_use]
    pub fn on_stopped(&self, f: DartOpaque) {
        self.0.on_stopped(platform::Function::new(f));
    }

    /// Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
    /// [`MediaDirection`] is changed.
    #[frb(sync)]
    #[must_use]
    pub fn on_media_direction_changed(&self, f: DartOpaque) {
        self.0.on_media_direction_changed(
            platform::Function::<MediaDirection>::new(f),
        );
    }

    /// Indicate whether this [`RemoteMediaTrack`] is muted.
    #[frb(sync)]
    #[must_use]
    pub fn muted(&self) -> bool {
        self.0.muted()
    }

    /// Returns this [`RemoteMediaTrack`]'s kind (audio/video).
    #[frb(sync)]
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.0.kind()
    }

    /// Returns this [`RemoteMediaTrack`]'s media source kind.
    #[frb(sync)]
    #[must_use]
    pub fn media_source_kind(&self) -> MediaSourceKind {
        self.0.media_source_kind()
    }

    /// Returns the current general [`MediaDirection`] of this
    /// [`RemoteMediaTrack`].
    #[frb(sync)]
    #[must_use]
    pub fn media_direction(&self) -> MediaDirection {
        self.0.media_direction()
    }
}
