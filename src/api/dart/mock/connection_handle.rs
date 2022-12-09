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
