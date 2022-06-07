use std::ptr;

use dart_sys::Dart_Handle;

use super::ForeignClass;

use crate::{
    api::dart::propagate_panic,
    media::{MediaKind, MediaSourceKind},
};

#[cfg(feature = "mockable")]
pub use self::mock::LocalMediaTrack;
#[cfg(not(feature = "mockable"))]
pub use crate::media::track::local::LocalMediaTrack;

impl ForeignClass for LocalMediaTrack {}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`LocalMediaTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
#[no_mangle]
pub unsafe extern "C" fn LocalMediaTrack__get_track(
    this: ptr::NonNull<LocalMediaTrack>,
) -> Dart_Handle {
    propagate_panic(move || this.as_ref().get_track().handle())
}

/// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
/// audio track, or a [`MediaKind::Video`] if it represents a video track.
///
/// [`MediaKind::Audio`]: crate::media::MediaKind::Audio
/// [`MediaKind::Video`]: crate::media::MediaKind::Video
#[no_mangle]
pub unsafe extern "C" fn LocalMediaTrack__kind(
    this: ptr::NonNull<LocalMediaTrack>,
) -> MediaKind {
    propagate_panic(move || this.as_ref().kind())
}

/// Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
/// sourced from some device (webcam/microphone), or a
/// [`MediaSourceKind::Display`] if it's captured via
/// [MediaDevices.getDisplayMedia()][1].
///
/// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
/// [`MediaSourceKind::Device`]: crate::media::MediaSourceKind::Device
/// [`MediaSourceKind::Display`]: crate::media::MediaSourceKind::Display
#[no_mangle]
pub unsafe extern "C" fn LocalMediaTrack__media_source_kind(
    this: ptr::NonNull<LocalMediaTrack>,
) -> MediaSourceKind {
    propagate_panic(move || this.as_ref().media_source_kind())
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn LocalMediaTrack__free(
    this: ptr::NonNull<LocalMediaTrack>,
) {
    propagate_panic(move || {
        drop(LocalMediaTrack::from_ptr(this));
    });
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
