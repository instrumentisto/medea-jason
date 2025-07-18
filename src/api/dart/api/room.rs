//! External handle to a [`Room`].

use flutter_rust_bridge::{DartOpaque, frb};
use send_wrapper::SendWrapper;
use tracerr::Traced;

use crate::{
    api::{Error as DartError, api::ApiMediaStreamSettings},
    media::MediaSourceKind,
    platform::{self, utils::dart_future::IntoDartFuture as _},
    room as core,
};
#[cfg(doc)]
use crate::{media::track::local::LocalMediaTrack, room::Room};

/// External handle to a [`Room`].
#[derive(Debug)]
#[frb(opaque)]
pub struct RoomHandle(pub(crate) SendWrapper<core::RoomHandle>);

impl From<core::RoomHandle> for RoomHandle {
    fn from(value: core::RoomHandle) -> Self {
        Self(SendWrapper::new(value))
    }
}

impl RoomHandle {
    /// Connects to a media server and joins the [`Room`] with the provided
    /// authorization `token`.
    ///
    /// Authorization token has a fixed format:
    /// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
    /// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
    #[frb(sync)]
    #[must_use]
    pub fn join(&self, token: String) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.join(token).await?;
            Ok::<_, Traced<core::RoomJoinError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Updates this [`Room`]'s [`ApiMediaStreamSettings`].
    ///
    /// This affects all the [`PeerConnection`]s in this [`Room`]. If
    /// [`ApiMediaStreamSettings`] are configured for some [`Room`], then this
    /// [`Room`] can only send media tracks that correspond to these settings.
    /// [`ApiMediaStreamSettings`] update will change media tracks in all
    /// sending peers, so that might cause a new [getUserMedia()][1] request to
    /// happen.
    ///
    /// Media obtaining/injection errors are additionally fired to a
    /// [`on_failed_local_media`] callback.
    ///
    /// If the `stop_first` argument is [`true`], then affected
    /// [`LocalMediaTrack`]s will be dropped before new
    /// [`ApiMediaStreamSettings`] are applied. This is usually required when
    /// changing video source device due to hardware limitations, e.g. having an
    /// active track sourced from device `A` may hinder [getUserMedia()][1]
    /// requests to device `B`.
    ///
    /// The `rollback_on_fail` argument configures [`ApiMediaStreamSettings`]
    /// update request to automatically roll back to previous settings if new
    /// settings cannot be applied.
    ///
    /// If recovering from fail state isn't possible then affected media types
    /// will be disabled.
    ///
    /// [`on_failed_local_media`]: RoomHandle::on_failed_local_media
    /// [`PeerConnection`]: crate::peer::PeerConnection
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    #[frb(sync)]
    #[must_use]
    pub fn set_local_media_settings(
        &self,
        settings: ApiMediaStreamSettings,
        stop_first: bool,
        rollback_on_fail: bool,
    ) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle
                .set_local_media_settings(
                    settings.into(),
                    stop_first,
                    rollback_on_fail,
                )
                .await?;
            Ok::<_, core::ConstraintsUpdateError>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Mutes outbound audio in the provided [`Room`].
    #[frb(sync)]
    #[must_use]
    pub fn mute_audio(&self) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.mute_audio(None).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Unmutes outbound audio in the provided [`Room`].
    #[frb(sync)]
    #[must_use]
    pub fn unmute_audio(&self) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.unmute_audio(None).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Enables outbound audio in the provided [`Room`].
    #[frb(sync)]
    #[must_use]
    pub fn enable_audio(&self) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.enable_audio(None).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Disables outbound audio in the provided [`Room`].
    #[frb(sync)]
    #[must_use]
    pub fn disable_audio(&self) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.disable_audio(None).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Mutes outbound video in the provided [`Room`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
    ///
    /// # Errors
    ///
    /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
    #[frb(sync)]
    #[must_use]
    pub fn mute_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.mute_video(source_kind).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Unmutes outbound video in the provided [`Room`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
    ///
    /// # Errors
    ///
    /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
    #[frb(sync)]
    #[must_use]
    pub fn unmute_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.unmute_video(source_kind).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Enables outbound video in the provided [`Room`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
    ///
    /// # Errors
    ///
    /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
    #[frb(sync)]
    #[must_use]
    pub fn enable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.enable_video(source_kind).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Disables outbound video in the provided [`Room`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
    ///
    /// # Errors
    ///
    /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
    #[frb(sync)]
    #[must_use]
    pub fn disable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.disable_video(source_kind).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Enables inbound audio in the provided [`Room`].
    #[frb(sync)]
    #[must_use]
    pub fn enable_remote_audio(&self) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.enable_remote_audio(None).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Disables inbound audio in the provided [`Room`].
    #[frb(sync)]
    #[must_use]
    pub fn disable_remote_audio(&self) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.disable_remote_audio(None).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Enables inbound video in the provided [`Room`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
    ///
    /// # Errors
    ///
    /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
    #[frb(sync)]
    #[must_use]
    pub fn enable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.enable_remote_video(source_kind).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Disables inbound video in the provided [`Room`].
    ///
    /// Affects only video with the provided [`MediaSourceKind`], if any.
    ///
    /// # Errors
    ///
    /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
    #[frb(sync)]
    #[must_use]
    pub fn disable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> DartOpaque {
        let room_handle = self.0.clone();

        async move {
            room_handle.disable_remote_video(source_kind).await?;

            Ok::<_, Traced<core::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Sets a callback to be invoked once a new [`Connection`] with some remote
    /// `Peer` is established.
    ///
    /// # Errors
    ///
    /// If the [`core::RoomHandle::on_new_connection()`] method errors.
    ///
    /// [`Connection`]: connection::Connection
    #[frb(sync)]
    pub fn on_new_connection(&self, cb: DartOpaque) -> Result<(), DartOpaque> {
        self.0
            .on_new_connection(platform::Function::new(cb))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked once the provided [`Room`] is closed,
    /// providing a [`RoomCloseReason`].
    ///
    /// # Errors
    ///
    /// If the [`core::RoomHandle::on_close()`] method errors.
    #[frb(sync)]
    pub fn on_close(&self, cb: DartOpaque) -> Result<(), DartOpaque> {
        self.0
            .on_close(platform::Function::new(cb))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked once a new [`LocalMediaTrack`] is added
    /// to the provided [`Room`].
    ///
    /// This might happen in such cases:
    /// 1. Media server initiates a media request.
    /// 2. [`enable_audio()`]/[`enable_video()`] is called.
    /// 3. [`MediaStreamSettings`] are updated via
    ///    [`set_local_media_settings()`].
    ///
    /// # Errors
    ///
    /// If the [`core::RoomHandle::on_local_track()`] method errors.
    ///
    /// [`enable_audio()`]: RoomHandle::enable_audio
    /// [`enable_video()`]: RoomHandle::enable_video
    /// [`MediaStreamSettings`]: media::MediaStreamSettings
    /// [`set_local_media_settings()`]: RoomHandle::set_local_media_settings
    #[frb(sync)]
    pub fn on_local_track(&self, cb: DartOpaque) -> Result<(), DartOpaque> {
        self.0
            .on_local_track(platform::Function::new(cb))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked once a connection with a media server is
    /// lost.
    ///
    /// # Errors
    ///
    /// If the [`core::RoomHandle::on_connection_loss()`] method errors.
    #[frb(sync)]
    pub fn on_connection_loss(&self, cb: DartOpaque) -> Result<(), DartOpaque> {
        self.0
            .on_connection_loss(platform::Function::new(cb))
            .map_err(DartError::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked on local media acquisition failures.
    ///
    /// # Errors
    ///
    /// If the [`core::RoomHandle::on_failed_local_media()`] method errors.
    #[frb(sync)]
    pub fn on_failed_local_media(
        &self,
        cb: DartOpaque,
    ) -> Result<(), DartOpaque> {
        self.0
            .on_failed_local_media(platform::Function::new(cb))
            .map_err(DartError::from)
            .map_err(Into::into)
    }
}
