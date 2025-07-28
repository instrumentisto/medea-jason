//! External handler to a [`Connection`] with a remote `Member`.

use flutter_rust_bridge::{DartOpaque, frb};
use send_wrapper::SendWrapper;
use tracerr::Traced;

use crate::{
    api::{Error as DartError, MemberConnectionState, dart::api::ForeignClass},
    connection as core,
    media::MediaSourceKind,
    platform::{self, utils::dart_future::IntoDartFuture as _},
};
#[cfg(doc)]
use crate::{connection::Connection, media::track::remote};

/// External handler to a [`Connection`] with a remote `Member`.
#[derive(Debug)]
#[frb(opaque)]
pub struct ConnectionHandle(SendWrapper<core::ConnectionHandle>);

impl From<core::ConnectionHandle> for ConnectionHandle {
    fn from(value: core::ConnectionHandle) -> Self {
        Self(SendWrapper::new(value))
    }
}

impl ForeignClass for ConnectionHandle {}

impl ConnectionHandle {
    /// Sets a callback to be invoked once the associated [`Connection`] is
    /// closed.
    ///
    /// # Errors
    ///
    /// If the [`core::ConnectionHandle::on_close()`] method errors.
    #[frb(sync)]
    pub fn on_close(&self, f: DartOpaque) -> Result<(), DartOpaque> {
        self.0
            .on_close(platform::Function::new(f))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked once a new [`remote::Track`] is added to
    /// the associated [`Connection`].
    ///
    /// # Errors
    ///
    /// If the [`core::ConnectionHandle::on_remote_track_added()`] method
    /// errors.
    ///
    /// [`remote::Track`]: media::track::remote::Track
    #[frb(sync)]
    pub fn on_remote_track_added(
        &self,
        f: DartOpaque,
    ) -> Result<(), DartOpaque> {
        self.0
            .on_remote_track_added(platform::Function::new(f))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked once a quality score of the associated
    /// [`Connection`] is updated by a media server.
    ///
    /// # Errors
    ///
    /// If the [`core::ConnectionHandle::on_quality_score_update()`] method
    /// errors.
    #[frb(sync)]
    pub fn on_quality_score_update(
        &self,
        f: DartOpaque,
    ) -> Result<(), DartOpaque> {
        self.0
            .on_quality_score_update(platform::Function::new(f))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Returns ID of remote `Member` ID of the associated [`Connection`].
    ///
    /// # Errors
    ///
    /// If the [`core::ConnectionHandle::get_remote_member_id()`] method errors.
    #[frb(sync)]
    pub fn get_remote_member_id(&self) -> Result<String, DartOpaque> {
        self.0
            .get_remote_member_id()
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Returns `MemberConnectionState` of the [`Connection`].
    ///
    /// <div class="warning">
    /// NOTE: this method only works in `P2P` mode and is subject to change.
    /// </div>
    ///
    /// # Errors
    ///
    /// If the [`core::ConnectionHandle::get_state()`] method errors.
    #[frb(sync)]
    pub fn get_state(
        &self,
    ) -> Result<Option<MemberConnectionState>, DartOpaque> {
        self.0
            .get_state()
            .map(|state| state.map(Into::into))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked once a state of associated [`Connection`]
    /// is changed.
    ///
    /// <div class="warning">
    /// NOTE: this method only works in `P2P` mode and is subject to change.
    /// </div>
    ///
    /// # Errors
    ///
    /// If the [`core::ConnectionHandle::on_state_change()`] method errors.
    #[frb(sync)]
    pub fn on_state_change(&self, f: DartOpaque) -> Result<(), DartOpaque> {
        self.0
            .on_state_change(platform::Function::new(f))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Enables inbound audio in the associated [`Connection`].
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

    /// Disables inbound audio in the associated [`Connection`].
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

    /// Enables inbound video in the associated [`Connection`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
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

    /// Disables inbound video in the associated [`Connection`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
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
