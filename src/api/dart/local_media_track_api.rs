use flutter_rust_bridge::{Opaque, SyncReturn};

#[cfg(feature = "mockable")]
pub use self::mock::LocalMediaTrack;
#[cfg(not(feature = "mockable"))]
pub use crate::media::track::local::LocalMediaTrack;

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`LocalMediaTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
pub fn local_media_track_get_track(
    track: Opaque<LocalMediaTrack>,
) -> SyncReturn<usize> {
    SyncReturn(track.get_track().handle() as _)
}

/// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
/// audio track, or a [`MediaKind::Video`] if it represents a video track.
///
/// [`MediaKind::Audio`]: crate::media::MediaKind::Audio
/// [`MediaKind::Video`]: crate::media::MediaKind::Video
pub fn local_media_track_kind(
    track: Opaque<LocalMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.kind() as u8)
}

/// Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
/// sourced from some device (webcam/microphone), or a
/// [`MediaSourceKind::Display`] if it's captured via
/// [MediaDevices.getDisplayMedia()][1].
///
/// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
/// [`MediaSourceKind::Device`]: crate::media::MediaSourceKind::Device
/// [`MediaSourceKind::Display`]: crate::media::MediaSourceKind::Display
pub fn local_media_track_media_source_kind(
    track: Opaque<LocalMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_source_kind() as u8)
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(clippy::unused_self, missing_copy_implementations)]

    use crate::{
        media::{
            track::local::LocalMediaTrack as CoreLocalMediaTrack, MediaKind,
            MediaSourceKind,
        },
        platform,
    };

    #[derive(Debug)]
    pub struct LocalMediaTrack(pub u8);

    impl From<CoreLocalMediaTrack> for LocalMediaTrack {
        fn from(_: CoreLocalMediaTrack) -> Self {
            Self(0)
        }
    }

    impl LocalMediaTrack {
        #[must_use]
        pub fn kind(&self) -> MediaKind {
            MediaKind::Video
        }

        #[must_use]
        pub fn media_source_kind(&self) -> MediaSourceKind {
            MediaSourceKind::Display
        }

        #[must_use]
        pub fn get_track(&self) -> platform::MediaStreamTrack {
            unreachable!()
        }
    }
}
