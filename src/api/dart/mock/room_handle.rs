#![allow(
    clippy::needless_pass_by_value,
    clippy::unused_async,
    clippy::unused_self,
    missing_copy_implementations
)]

use std::future::Future;

use futures::future;
use tracerr::Traced;

use crate::{
    api::{
        dart::utils::DartError, ConnectionHandle, LocalMediaTrack,
        MediaStreamSettings, ReconnectHandle,
    },
    media::MediaSourceKind,
    peer::{LocalMediaError, TracksRequestError, UpdateLocalStreamError},
    platform,
    room::{
        ChangeMediaStateError, ConstraintsUpdateError, HandleDetachedError,
        RoomCloseReason, RoomJoinError,
    },
    rpc::{ClientDisconnect, CloseReason, ConnectionInfo},
};

/// Alias for a [`Result`] related to [`MediaState`] update functions.
type ChangeMediaStateResult = Result<(), Traced<ChangeMediaStateError>>;

#[derive(Clone, Debug)]
pub struct RoomHandle(pub u8);

#[allow(clippy::missing_errors_doc, clippy::unused_async)]
impl RoomHandle {
    pub fn on_new_connection(
        &self,
        cb: platform::Function<ConnectionHandle>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        cb.call1(ConnectionHandle(0));
        Ok(())
    }

    pub fn on_close(
        &self,
        cb: platform::Function<RoomCloseReason>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        println!("RAZ CALL5");
        let a = CloseReason::ByClient {
            is_err: true,
            reason: ClientDisconnect::RpcClientUnexpectedlyDropped,
        };
        println!("RAZ CALL6");
        let b = RoomCloseReason::new(a);
        println!("RAZ CALL7");
        cb.call1(b);
        println!("RAZ CALL8");
        Ok(())
    }

    pub fn on_local_track(
        &self,
        cb: platform::Function<LocalMediaTrack>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        cb.call1(LocalMediaTrack(0));
        Ok(())
    }

    pub fn on_connection_loss(
        &self,
        cb: platform::Function<ReconnectHandle>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        cb.call1(ReconnectHandle(0));
        Ok(())
    }

    pub async fn join(
        &self,
        token: String,
    ) -> Result<(), Traced<RoomJoinError>> {
        token
            .parse::<ConnectionInfo>()
            .map_err(tracerr::map_from_and_wrap!())
            .map(drop)
    }

    pub fn on_failed_local_media(
        &self,
        cb: platform::Function<DartError>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        cb.call1(
            tracerr::new!(LocalMediaError::UpdateLocalStreamError(
                UpdateLocalStreamError::InvalidLocalTracks(
                    TracksRequestError::NoTracks,
                ),
            ))
            .into(),
        );
        Ok(())
    }

    pub async fn set_local_media_settings(
        &self,
        _settings: MediaStreamSettings,
        _stop_first: bool,
        _rollback_on_fail: bool,
    ) -> Result<(), ConstraintsUpdateError> {
        Ok(())
    }

    pub fn mute_audio(
        &self,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        future::ok(())
    }

    pub fn unmute_audio(
        &self,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        future::ok(())
    }

    pub fn enable_audio(
        &self,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        future::ok(())
    }

    pub fn disable_audio(
        &self,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        future::ok(())
    }

    pub fn mute_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        assert_eq!(source_kind, None);
        future::ok(())
    }

    pub fn unmute_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        assert_eq!(source_kind, Some(MediaSourceKind::Display));
        future::ok(())
    }

    pub fn enable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        assert_eq!(source_kind, Some(MediaSourceKind::Device));
        future::ok(())
    }

    pub fn disable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
        assert_eq!(source_kind, Some(MediaSourceKind::Display));
        future::ok(())
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
