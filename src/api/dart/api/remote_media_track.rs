use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    ptr,
};

use derive_more::From;
use flutter_rust_bridge::{frb, DartOpaque};

use crate::{
    api::{utils::new_dart_opaque, ForeignClass, MediaDirection},
    media::{track::remote as core, MediaKind, MediaSourceKind},
    platform,
};

#[derive(Debug, From)]
#[frb(opaque)]
pub struct RemoteMediaTrack(core::Track);

impl RemoteMediaTrack {
    /// Returns the [`RemoteMediaTrack`] from the [`ForeignClass`] address.
    #[frb(sync, type_64bit_int)]
    #[must_use]
    pub fn from_raw(ptr: usize) -> RemoteMediaTrack {
        unsafe {
            RemoteMediaTrack::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
        }
    }

    /// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
    /// [`RemoteMediaTrack`].
    ///
    /// [`MediaStreamTrack`]: platform::MediaStreamTrack
    #[frb(sync)]
    #[must_use]
    pub fn get_track(&self) -> DartOpaque {
        unsafe { new_dart_opaque(self.0.get_track().handle()) }
    }

    /// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
    #[frb(sync)]
    #[must_use]
    pub fn on_muted(&self, f: DartOpaque) {
        self.0.on_muted(platform::Function::new(f));
    }

    /// Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
    #[frb(sync)]
    #[must_use]
    pub fn on_unmuted(&self, f: DartOpaque) {
        self.0.on_unmuted(platform::Function::new(f));
    }

    /// Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
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

impl ForeignClass for RemoteMediaTrack {}
impl RefUnwindSafe for RemoteMediaTrack {}
impl UnwindSafe for RemoteMediaTrack {}
unsafe impl Send for RemoteMediaTrack {}
unsafe impl Sync for RemoteMediaTrack {}
