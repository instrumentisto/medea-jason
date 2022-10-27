use crate::{
    connection::ChangeMediaStateError, media::MediaSourceKind,
    platform,
};
pub use dart_sys::Dart_Handle;
use flutter_rust_bridge::{Opaque, SyncReturn};
use tracerr::Traced;

use super::utils::{IntoDartFuture, MyDartFuture};

#[cfg(feature = "mockable")]
pub use self::mock::ConnectionHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::connection::ConnectionHandle;

/// Sets callback, invoked when this `Connection` will close.
pub fn connection_handle_on_close(
    connection: Opaque<ConnectionHandle>,
    f: Opaque<Dart_Handle>,
) -> anyhow::Result<SyncReturn<()>> {
    connection
        .on_close(unsafe { platform::Function::new(Dart_Handle::clone(&f)) })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a new [`remote::Track`] is added to this
/// [`Connection`].
///
/// [`remote::Track`]: crate::media::track::remote::Track
/// [`Connection`]: crate::connection::Connection
pub fn connection_handle_on_remote_track_added(
    connection: Opaque<ConnectionHandle>,
    f: Opaque<Dart_Handle>,
) -> anyhow::Result<SyncReturn<()>> {
    connection
        .on_remote_track_added(unsafe {
            platform::Function::new(Dart_Handle::clone(&f))
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a connection quality score is updated by
/// a server.
pub fn connection_handle_on_quality_score_update(
    connection: Opaque<ConnectionHandle>,
    f: Opaque<Dart_Handle>,
) -> anyhow::Result<SyncReturn<()>> {
    connection
        .on_quality_score_update(unsafe {
            platform::Function::new(Dart_Handle::clone(&f))
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Returns remote `Member` ID.
pub fn connection_handle_get_remote_member_id(
    connection: Opaque<ConnectionHandle>,
) -> anyhow::Result<SyncReturn<String>> {
    Ok(SyncReturn(
        connection.get_remote_member_id().map_err(|err| anyhow::anyhow!("{}", err))?
    ))
}

/// Enables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_enable_remote_audio(
    connection: Opaque<ConnectionHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let fut = connection.enable_remote_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<(), Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Disables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_disable_remote_audio(
    connection: Opaque<ConnectionHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let fut = connection.disable_remote_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<(), Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Enables inbound video in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_enable_remote_video(
    connection: Opaque<ConnectionHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let fut = connection.enable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<(), Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Disables inbound video in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_disable_remote_video(
    connection: Opaque<ConnectionHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let fut = connection.disable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<(), Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
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
