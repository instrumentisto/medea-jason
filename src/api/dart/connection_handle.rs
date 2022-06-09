use std::ptr;

use dart_sys::Dart_Handle;
use tracerr::Traced;

use crate::{
    api::{
        dart::utils::{DartError, DartResult},
        utils::{dart_arg_try_into, DartFuture, IntoDartFuture as _},
        ArgumentError, DartValueCastError,
    },
    connection::ChangeMediaStateError,
    media::MediaSourceKind,
    platform,
};

use super::{propagate_panic, DartValueArg, ForeignClass};

#[cfg(feature = "mockable")]
pub use self::mock::ConnectionHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::connection::ConnectionHandle;

impl ForeignClass for ConnectionHandle {}

/// Sets callback, invoked when this `Connection` will close.
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__on_close(
    this: ptr::NonNull<ConnectionHandle>,
    f: Dart_Handle,
) -> DartResult {
    propagate_panic(move || {
        this.as_ref()
            .on_close(platform::Function::new(f))
            .map_err(DartError::from)
            .into()
    })
}

/// Sets callback, invoked when a new [`remote::Track`] is added to this
/// [`Connection`].
///
/// [`remote::Track`]: crate::media::track::remote::Track
/// [`Connection`]: crate::connection::Connection
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__on_remote_track_added(
    this: ptr::NonNull<ConnectionHandle>,
    f: Dart_Handle,
) -> DartResult {
    propagate_panic(move || {
        this.as_ref()
            .on_remote_track_added(platform::Function::new(f))
            .map_err(DartError::from)
            .into()
    })
}

/// Sets callback, invoked when a connection quality score is updated by
/// a server.
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__on_quality_score_update(
    this: ptr::NonNull<ConnectionHandle>,
    f: Dart_Handle,
) -> DartResult {
    propagate_panic(move || {
        this.as_ref()
            .on_quality_score_update(platform::Function::new(f))
            .map_err(DartError::from)
            .into()
    })
}

/// Returns remote `Member` ID.
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__get_remote_member_id(
    this: ptr::NonNull<ConnectionHandle>,
) -> DartResult {
    propagate_panic(move || {
        this.as_ref()
            .get_remote_member_id()
            .map_err(DartError::from)
            .into()
    })
}

/// Enables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__enable_remote_audio(
    this: ptr::NonNull<ConnectionHandle>,
) -> DartFuture<Result<(), Traced<ChangeMediaStateError>>> {
    propagate_panic(move || {
        let this = this.as_ref();

        let fut = this.enable_remote_audio();
        async move {
            fut.await?;
            Ok(())
        }
        .into_dart_future()
    })
}

/// Disables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__disable_remote_audio(
    this: ptr::NonNull<ConnectionHandle>,
) -> DartFuture<Result<(), Traced<ChangeMediaStateError>>> {
    propagate_panic(move || {
        let this = this.as_ref();

        let fut = this.disable_remote_audio();
        async move {
            fut.await?;
            Ok(())
        }
        .into_dart_future()
    })
}

/// Enables inbound video in this [`ConnectionHandle`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__enable_remote_video(
    this: ptr::NonNull<ConnectionHandle>,
    source_kind: DartValueArg<Option<MediaSourceKind>>,
) -> DartFuture<Result<(), DartError>> {
    propagate_panic(move || {
        let this = this.as_ref();

        let fut = this.enable_remote_video(dart_arg_try_into!(source_kind));
        async move {
            fut.await?;
            Ok(())
        }
        .into_dart_future()
    })
}

/// Disables inbound video in this [`ConnectionHandle`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__disable_remote_video(
    this: ptr::NonNull<ConnectionHandle>,
    source_kind: DartValueArg<Option<MediaSourceKind>>,
) -> DartFuture<Result<(), DartError>> {
    propagate_panic(move || {
        let this = this.as_ref();

        let fut = this.disable_remote_video(dart_arg_try_into!(source_kind));
        async move {
            fut.await?;
            Ok(())
        }
        .into_dart_future()
    })
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn ConnectionHandle__free(
    this: ptr::NonNull<ConnectionHandle>,
) {
    propagate_panic(move || {
        drop(ConnectionHandle::from_ptr(this));
    });
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::unused_self,
        clippy::missing_errors_doc,
        clippy::needless_pass_by_value,
        missing_copy_implementations
    )]

    use std::future::Future;

    use futures::future;
    use tracerr::Traced;

    use crate::{
        api::RemoteMediaTrack,
        connection::{
            ChangeMediaStateError, ConnectionHandle as CoreConnectionHandle,
            HandleDetachedError,
        },
        media::MediaSourceKind,
        platform,
    };

    /// Alias for a [`Result`] related to [`MediaState`] update functions.
    type ChangeMediaStateResult = Result<(), Traced<ChangeMediaStateError>>;

    #[derive(Debug)]
    pub struct ConnectionHandle(pub u8);

    impl From<CoreConnectionHandle> for ConnectionHandle {
        fn from(_: CoreConnectionHandle) -> Self {
            Self(0)
        }
    }

    impl ConnectionHandle {
        pub fn get_remote_member_id(
            &self,
        ) -> Result<String, Traced<HandleDetachedError>> {
            Err(tracerr::new!(HandleDetachedError))
        }

        pub fn on_close(
            &self,
            f: platform::Function<()>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            f.call0();
            Ok(())
        }

        pub fn on_remote_track_added(
            &self,
            f: platform::Function<RemoteMediaTrack>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            f.call1(RemoteMediaTrack(0));
            Ok(())
        }

        pub fn on_quality_score_update(
            &self,
            f: platform::Function<u8>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            f.call1(4);
            Ok(())
        }

        pub fn enable_remote_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn disable_remote_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn enable_remote_video(
            &self,
            _: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::err(tracerr::new!(ChangeMediaStateError::Detached))
        }

        pub fn disable_remote_video(
            &self,
            _: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }
    }
}
