use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    ptr,
};

use derive_more::From;
use flutter_rust_bridge::{frb, DartOpaque};
use tracerr::Traced;

use crate::{
    api::{Error as DartError, ForeignClass},
    connection as core,
    media::MediaSourceKind,
    platform::{self, utils::dart_future::IntoDartFuture},
};

#[derive(Debug, From)]
#[frb(opaque)]
pub struct ConnectionHandle(core::ConnectionHandle);

impl ConnectionHandle {
    /// Returns the [`ConnectionHandle`] from the [`ForeignClass`] address.
    #[frb(sync, type_64bit_int)]
    #[must_use]
    pub fn from_raw(ptr: usize) -> ConnectionHandle {
        unsafe {
            ConnectionHandle::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
        }
    }

    /// Sets a callback to be invoked once the provided `connection` is closed.
    ///
    /// # Errors
    ///
    /// If [`ConnectionHandle::on_close()`] errors.
    #[frb(sync)]
    pub fn on_close(&self, f: DartOpaque) -> Result<(), DartOpaque> {
        self.0
            .on_close(platform::Function::new(f))
            .map_err(DartError::from)?;

        Ok(())
    }

    /// Sets a callback to be invoked once a new [`remote::Track`] is added to
    /// the provided `connection`.
    ///
    /// # Errors
    ///
    /// If [`ConnectionHandle::on_remote_track_added()`] errors.
    ///
    /// [`remote::Track`]: media::track::remote::Track
    #[frb(sync)]
    pub fn on_remote_track_added(
        &self,
        f: DartOpaque,
    ) -> Result<(), DartOpaque> {
        self.0
            .on_remote_track_added(platform::Function::new(f))
            .map_err(DartError::from)?;

        Ok(())
    }

    /// Sets a callback to be invoked when a quality score of the provided
    /// `connection` is updated by a server.
    ///
    /// # Errors
    ///
    /// If [`ConnectionHandle::on_quality_score_update()`] errors.
    #[frb(sync)]
    pub fn on_quality_score_update(
        &self,
        f: DartOpaque,
    ) -> Result<(), DartOpaque> {
        self.0
            .on_quality_score_update(platform::Function::new(f))
            .map_err(DartError::from)?;

        Ok(())
    }

    /// Returns remote `Member` ID of the provided `connection`.
    ///
    /// # Errors
    ///
    /// If [`ConnectionHandle::get_remote_member_id()`] errors.
    #[frb(sync)]
    pub fn get_remote_member_id(&self) -> Result<String, DartOpaque> {
        Ok(self.0.get_remote_member_id().map_err(DartError::from)?)
    }

    /// Enables inbound audio in the provided `connection`.
    #[frb(sync)]
    #[must_use]
    pub fn enable_remote_audio(&self) -> DartOpaque {
        let con = self.0.clone();

        async move {
            con.enable_remote_audio().await?;

            Ok::<(), Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Disables inbound audio in the provided `connection`.
    #[frb(sync)]
    #[must_use]
    pub fn disable_remote_audio(&self) -> DartOpaque {
        let con = self.0.clone();

        async move {
            con.disable_remote_audio().await?;

            Ok::<(), Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Enables inbound video in the provided `connection`.
    ///
    /// Affects only video with the specific [`MediaSourceKind`], if specified.
    #[frb(sync)]
    #[must_use]
    pub fn enable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let con = self.0.clone();

        let result = async move {
            con.enable_remote_video(source_kind).await?;

            Ok::<(), Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque();

        result
    }

    /// Disables inbound video in the provided `connection`.
    ///
    /// Affects only video with the specific [`MediaSourceKind`], if specified.
    #[frb(sync)]
    #[must_use]
    pub fn disable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let con = self.0.clone();

        async move {
            con.disable_remote_video(source_kind).await?;

            Ok::<(), Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }
}

impl ForeignClass for ConnectionHandle {}
impl RefUnwindSafe for ConnectionHandle {}
impl UnwindSafe for ConnectionHandle {}
unsafe impl Send for ConnectionHandle {}
unsafe impl Sync for ConnectionHandle {}
