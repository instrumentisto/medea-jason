use std::panic::UnwindSafe;
use std::ptr;

use dart_sys::Dart_Handle;

use crate::{
    api::dart::utils::{DartError, DartResult},
    platform,
};

use super::panic_catcher;
use super::ForeignClass;

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
    panic_catcher(move || {
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
    panic_catcher(move || {
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
    panic_catcher(move || {
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
    panic_catcher(move || {
        this.as_ref()
            .get_remote_member_id()
            .map_err(DartError::from)
            .into()
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
    panic_catcher(move || {
        drop(ConnectionHandle::from_ptr(this));
    })
}

#[cfg(feature = "mockable")]
mod mock {
    use tracerr::Traced;

    use crate::{
        api::RemoteMediaTrack,
        connection::{
            ConnectionHandle as CoreConnectionHandle, HandleDetachedError,
        },
        platform,
    };

    #[derive(Clone, Copy, Debug)]
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
            Err(tracerr::new!(HandleDetachedError).into())
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
    }
}
