pub use dart_sys::Dart_Handle;
use flutter_rust_bridge::{Opaque, SyncReturn};

#[cfg(feature = "mockable")]
pub use self::mock::RemoteMediaTrack;
#[cfg(not(feature = "mockable"))]
pub use crate::media::track::remote::Track as RemoteMediaTrack;
use crate::platform;

pub type RemoteMediaTrackDH = Dart_Handle;

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of track
/// [`RemoteMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
pub fn remote_media_track_get_track(
    track: Opaque<RemoteMediaTrack>,
) -> SyncReturn<Opaque<RemoteMediaTrackDH>> {
    SyncReturn(Opaque::new(track.get_track().handle()))
}

/// Sets callback to invoke when track [`RemoteMediaTrack`] is muted.
pub fn remote_media_track_on_muted(
    track: Opaque<RemoteMediaTrack>,
    f: Opaque<RemoteMediaTrackDH>,
) -> SyncReturn<()> {
    track.on_muted(unsafe { platform::Function::new(Dart_Handle::clone(&f)) });
    SyncReturn(())
}

/// Sets callback to invoke when track [`RemoteMediaTrack`] is unmuted.
pub fn remote_media_track_on_unmuted(
    track: Opaque<RemoteMediaTrack>,
    f: Opaque<RemoteMediaTrackDH>,
) -> SyncReturn<()> {
    track
        .on_unmuted(unsafe { platform::Function::new(Dart_Handle::clone(&f)) });
    SyncReturn(())
}

/// Sets callback to invoke when track [`RemoteMediaTrack`] is stopped.
pub fn remote_media_track_on_stopped(
    track: Opaque<RemoteMediaTrack>,
    f: Opaque<RemoteMediaTrackDH>,
) -> SyncReturn<()> {
    track
        .on_stopped(unsafe { platform::Function::new(Dart_Handle::clone(&f)) });
    SyncReturn(())
}

/// Sets callback to invoke whenever track [`RemoteMediaTrack`]'s general
/// [`MediaDirection`] is changed.
pub fn remote_media_track_on_media_direction_changed(
    track: Opaque<RemoteMediaTrack>,
    f: Opaque<RemoteMediaTrackDH>,
) -> SyncReturn<()> {
    track.on_media_direction_changed(unsafe {
        platform::Function::new(Dart_Handle::clone(&f))
    });
    SyncReturn(())
}

/// Indicate whether track [`RemoteMediaTrack`] is muted.
pub fn remote_media_track_muted(
    track: Opaque<RemoteMediaTrack>,
) -> SyncReturn<bool> {
    SyncReturn(track.muted())
}

/// Returns track [`RemoteMediaTrack`]'s kind (audio/video).
pub fn remote_media_track_kind(
    track: Opaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.kind() as u8)
}

/// Returns track [`RemoteMediaTrack`]'s media source kind.
pub fn remote_media_track_media_source_kind(
    track: Opaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_source_kind() as u8)
}

/// Returns the current general [`MediaDirection`] of track
/// [`RemoteMediaTrack`].
pub fn remote_media_track_media_direction(
    track: Opaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_direction() as u8)
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::unused_self,
        clippy::needless_pass_by_value,
        missing_copy_implementations
    )]

    use crate::{
        api,
        media::{
            track::remote::Track as CoreRemoteMediaTrack, MediaDirection,
            MediaKind, MediaSourceKind,
        },
        platform,
    };

    #[derive(Clone, Debug)]
    pub struct RemoteMediaTrack(pub u8);

    impl From<CoreRemoteMediaTrack> for RemoteMediaTrack {
        fn from(_: CoreRemoteMediaTrack) -> Self {
            Self(0)
        }
    }

    impl RemoteMediaTrack {
        #[must_use]
        pub fn enabled(&self) -> bool {
            true
        }

        #[must_use]
        pub fn kind(&self) -> MediaKind {
            MediaKind::Video
        }

        #[must_use]
        pub fn media_source_kind(&self) -> MediaSourceKind {
            MediaSourceKind::Device
        }

        #[must_use]
        pub fn muted(&self) -> bool {
            false
        }

        pub fn on_enabled(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_disabled(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_muted(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_unmuted(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_stopped(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        #[allow(unused_qualifications)]
        pub fn on_media_direction_changed(
            &self,
            cb: platform::Function<api::MediaDirection>,
        ) {
            cb.call1(api::MediaDirection::SendRecv);
        }

        #[must_use]
        pub fn media_direction(&self) -> MediaDirection {
            MediaDirection::SendRecv
        }

        #[must_use]
        pub fn get_track(&self) -> platform::MediaStreamTrack {
            unreachable!()
        }
    }
}
