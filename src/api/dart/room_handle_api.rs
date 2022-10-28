pub use dart_sys::Dart_Handle;
use flutter_rust_bridge::{Opaque, SyncReturn};
use tracerr::Traced;

use super::jason_api::JasonRH;
pub use super::{
    jason_api::{IntoDartFuture, MyDartFuture},
    MediaStreamSettings,
};

#[cfg(feature = "mockable")]
pub use self::mock::RoomHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::room::RoomHandle;
use crate::{
    media::MediaSourceKind,
    platform,
    room::{ChangeMediaStateError, ConstraintsUpdateError, RoomJoinError},
};

pub type RoomHandleDH = Dart_Handle;
pub type RoomHandleMS = MediaStreamSettings;


/// Connects to a media server and joins the [`Room`] with the provided
/// authorization `token`.
///
/// Authorization token has a fixed format:
/// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
/// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
///
/// [`Room`]: crate::room::Room
pub fn room_handle_join(
    room_handle: Opaque<RoomHandle>,
    token: String,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    SyncReturn(Opaque::new(
        async move {
            room_handle.join(token).await?;
            Ok::<_, Traced<RoomJoinError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Creates new [`MediaStreamSettings`] with none constraints configured.
pub fn room_handle_cast(
    room_handle: Opaque<RoomHandle>
) -> SyncReturn<Opaque<JasonRH>> {
    SyncReturn(room_handle)
}


/// Updates room_handle [`Room`]'s [`MediaStreamSettings`]. room_handle affects
/// all the [`PeerConnection`]s in room_handle [`Room`]. If
/// [`MediaStreamSettings`] are configured for some [`Room`], then room_handle
/// [`Room`] can only send media tracks that correspond to these settings.
/// [`MediaStreamSettings`] update will change media tracks in all sending
/// peers, so that might cause a new [getUserMedia()][1] request to happen.
///
/// Media obtaining/injection errors are additionally fired to
/// `on_failed_local_media` callback.
///
/// If `stop_first` set to `true` then affected local `Tracks` will be
/// dropped before new [`MediaStreamSettings`] are applied. room_handle is
/// usually required when changing video source device due to hardware
/// limitations, e.g. having an active track sourced from device `A` may hinder
/// [getUserMedia()][1] requests to device `B`.
///
/// `rollback_on_fail` option configures [`MediaStreamSettings`] update request
/// to automatically rollback to previous settings if new settings cannot be
/// applied.
///
/// If recovering from fail state isn't possible then affected media types will
/// be disabled.
///
/// [`Room`]: crate::room::Room
/// [`PeerConnection`]: crate::peer::PeerConnection
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia

pub fn room_handle_set_local_media_settings(
    room_handle: Opaque<RoomHandle>,
    settings: Opaque<RoomHandleMS>,
    stop_first: bool,
    rollback_on_fail: bool,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);
    let settings = MediaStreamSettings::clone(&settings);

    SyncReturn(Opaque::new(
        async move {
            room_handle
                .set_local_media_settings(
                    settings,
                    stop_first,
                    rollback_on_fail,
                )
                .await?;
            Ok::<_, ConstraintsUpdateError>(())
        }
        .into_my_dart_future(),
    ))
}

/// Mutes outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_mute_audio(
    room_handle: Opaque<RoomHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.mute_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Unmutes outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_unmute_audio(
    room_handle: Opaque<RoomHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.unmute_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Enables outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_enable_audio(
    room_handle: Opaque<RoomHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Disables outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_disable_audio(
    room_handle: Opaque<RoomHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Mutes outbound video in room_handle [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_mute_video(
    room_handle: Opaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.mute_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Unmutes outbound video in room_handle [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_unmute_video(
    room_handle: Opaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.unmute_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Enables outbound video.
///
/// Affects only video with specific [`MediaSourceKind`] if specified.

pub fn room_handle_enable_video(
    room_handle: Opaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Disables outbound video.
///
/// Affects only video with specific [`MediaSourceKind`] if specified.

pub fn room_handle_disable_video(
    room_handle: Opaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Enables inbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_enable_remote_audio(
    room_handle: Opaque<RoomHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_remote_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Disables inbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_disable_remote_audio(
    room_handle: Opaque<RoomHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_remote_audio();
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Enables inbound video in room_handle [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_enable_remote_video(
    room_handle: Opaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Disables inbound video in room_handle [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_disable_remote_video(
    room_handle: Opaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    SyncReturn(Opaque::new(
        async move {
            fut.await?;
            Ok::<_, Traced<ChangeMediaStateError>>(())
        }
        .into_my_dart_future(),
    ))
}

/// Sets callback, invoked when a new [`Connection`] with some remote `Peer`
/// is established.
///
/// [`Connection`]: crate::connection::Connection

pub fn room_handle_on_new_connection(
    room_handle: Opaque<RoomHandle>,
    cb: Opaque<RoomHandleDH>,
) -> anyhow::Result<SyncReturn<()>> {
    Ok(SyncReturn(
        room_handle
            .on_new_connection(unsafe {
                platform::Function::new(Dart_Handle::clone(&cb))
            })
            .map_err(|err| anyhow::anyhow!("{}", err))?,
    ))
}

/// Sets callback, invoked on room_handle [`Room`] close, providing a
/// [`RoomCloseReason`].
///
/// [`Room`]: crate::room::Room
/// [`RoomCloseReason`]: crate::room::RoomCloseReason

pub fn room_handle_on_close(
    room_handle: Opaque<RoomHandle>,
    cb: Opaque<RoomHandleDH>,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_close(unsafe { platform::Function::new(Dart_Handle::clone(&cb)) })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a new [`LocalMediaTrack`] is added to
/// room_handle [`Room`].
///
/// room_handle might happen in such cases:
/// 1. Media server initiates a media request.
/// 2. `enable_audio`/`enable_video` is called.
/// 3. [`MediaStreamSettings`] updated via `set_local_media_settings`.
///
/// [`Room`]: crate::room::Room
/// [`MediaStreamSettings`]: crate::media::MediaStreamSettings
/// [`LocalMediaTrack`]: crate::media::track::local::LocalMediaTrack

pub fn room_handle_on_local_track(
    room_handle: Opaque<RoomHandle>,
    cb: Opaque<RoomHandleDH>,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_local_track(unsafe {
            platform::Function::new(Dart_Handle::clone(&cb))
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a connection with server is lost.

pub fn room_handle_on_connection_loss(
    room_handle: Opaque<RoomHandle>,
    cb: Opaque<RoomHandleDH>,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_connection_loss(unsafe {
            platform::Function::new(Dart_Handle::clone(&cb))
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked on local media acquisition failures.
pub fn room_handle_on_failed_local_media(
    room_handle: Opaque<RoomHandle>,
    cb: Opaque<RoomHandleDH>,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_failed_local_media(unsafe {
            platform::Function::new(Dart_Handle::clone(&cb))
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

#[cfg(feature = "mockable")]
mod mock {
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
            cb.call1(RoomCloseReason::new(CloseReason::ByClient {
                is_err: true,
                reason: ClientDisconnect::RpcClientUnexpectedlyDropped,
            }));
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
}
