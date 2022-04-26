use std::ptr;

use dart_sys::Dart_Handle;

use crate::{
    media::{MediaDirection, MediaKind, MediaSourceKind},
    platform,
};

use super::{propagate_panic, ForeignClass};

#[cfg(feature = "mockable")]
pub use self::mock::RemoteMediaTrack;
#[cfg(not(feature = "mockable"))]
pub use crate::media::track::remote::Track as RemoteMediaTrack;

impl ForeignClass for RemoteMediaTrack {}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`RemoteMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__get_track(
    this: ptr::NonNull<RemoteMediaTrack>,
) -> Dart_Handle {
    propagate_panic(move || this.as_ref().get_track().handle())
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__on_muted(
    this: ptr::NonNull<RemoteMediaTrack>,
    f: Dart_Handle,
) {
    propagate_panic(move || {
        this.as_ref().on_muted(platform::Function::new(f));
    });
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__on_unmuted(
    this: ptr::NonNull<RemoteMediaTrack>,
    f: Dart_Handle,
) {
    propagate_panic(move || {
        this.as_ref().on_unmuted(platform::Function::new(f));
    });
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__on_stopped(
    this: ptr::NonNull<RemoteMediaTrack>,
    f: Dart_Handle,
) {
    propagate_panic(move || {
        this.as_ref().on_stopped(platform::Function::new(f));
    });
}

/// Sets callback to invoke when this [`RemoteMediaTrack`]'s general media
/// exchange direction is changed.
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__on_media_direction_changed(
    this: ptr::NonNull<RemoteMediaTrack>,
    f: Dart_Handle,
) {
    propagate_panic(move || {
        this.as_ref()
            .on_media_direction_changed(platform::Function::new(f));
    });
}

/// Indicate whether this [`RemoteMediaTrack`] is muted.
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__muted(
    this: ptr::NonNull<RemoteMediaTrack>,
) -> u8 {
    propagate_panic(move || this.as_ref().muted().into())
}

/// Returns this [`RemoteMediaTrack`]'s kind (audio/video).
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__kind(
    this: ptr::NonNull<RemoteMediaTrack>,
) -> MediaKind {
    propagate_panic(move || this.as_ref().kind())
}

/// Returns this [`RemoteMediaTrack`]'s media source kind.
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__media_source_kind(
    this: ptr::NonNull<RemoteMediaTrack>,
) -> MediaSourceKind {
    propagate_panic(move || this.as_ref().media_source_kind())
}

/// Returns current general media exchange direction of this
/// [`RemoteMediaTrack`].
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__media_direction(
    this: ptr::NonNull<RemoteMediaTrack>,
) -> MediaDirection {
    propagate_panic(move || this.as_ref().media_direction())
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn RemoteMediaTrack__free(
    this: ptr::NonNull<RemoteMediaTrack>,
) {
    propagate_panic(move || {
        drop(RemoteMediaTrack::from_ptr(this));
    });
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
