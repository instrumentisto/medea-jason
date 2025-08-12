//! JS side handle to a [`Room`].
//!
//! [`Room`]: room::Room

use derive_more::with_trait::{From, Into};
use js_sys::Promise;
use wasm_bindgen::{JsValue, prelude::*};
use wasm_bindgen_futures::future_to_promise;

use super::Error;
use crate::{
    api::{MediaSourceKind, MediaStreamSettings},
    room,
};

/// JS side handle to a [`Room`] where all the media happens.
///
/// Like all handles it contains a weak reference to the object that is managed
/// by Rust, so its methods will fail if a weak reference could not be upgraded.
///
/// [`Room`]: room::Room
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct RoomHandle(room::RoomHandleImpl);

#[wasm_bindgen]
impl RoomHandle {
    /// Connects to a media server and joins a [`Room`] with the provided
    /// authorization `token`.
    ///
    /// Authorization token has a fixed format:
    /// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
    /// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
    ///
    /// Establishes connection with media server (if it doesn't exist already).
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed, or if
    /// some mandatory callback is not set. These callbacks are:
    /// [`RoomHandle::on_connection_loss`] and
    /// [`RoomHandle::on_failed_local_media`].
    ///
    /// With a [`FormatException`] if the provided `token` string has bad
    /// format.
    ///
    /// With a [`RpcClientException`] if could not connect to a media server.
    ///
    /// [`FormatException`]: crate::api::err::FormatException
    /// [`Room`]: room::Room
    /// [`RpcClientException`]: crate::api::err::RpcClientException
    /// [`StateError`]: crate::api::err::StateError
    pub fn join(&self, token: String) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.join(token).await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Sets callback, invoked when a new [`Connection`] with some remote
    /// `Member` is established.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// [`Connection`]: crate::connection::Connection
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_new_connection(
        &self,
        cb: js_sys::Function,
    ) -> Result<(), JsValue> {
        self.0
            .on_new_connection(cb.into())
            .map_err(Error::from)
            .map_err(Into::into)
    }

    /// Sets `on_close` callback, invoked when this [`Room`] is closed,
    /// providing a [`RoomCloseReason`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// [`Room`]: room::Room
    /// [`RoomCloseReason`]: room::RoomCloseReasonImpl
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_close(&self, cb: js_sys::Function) -> Result<(), JsValue> {
        self.0.on_close(cb.into()).map_err(Error::from).map_err(Into::into)
    }

    /// Sets callback, invoked when a new [`LocalMediaTrack`] is added to this
    /// [`Room`].
    ///
    /// This might happen in such cases:
    /// 1. Media server initiates a media request.
    /// 2. `enable_audio`/`enable_video` is called.
    /// 3. [`MediaStreamSettings`] is updated via `set_local_media_settings`.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// [`Room`]: room::Room
    /// [`LocalMediaTrack`]: crate::api::LocalMediaTrack
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_local_track(&self, cb: js_sys::Function) -> Result<(), JsValue> {
        self.0
            .on_local_track(cb.into())
            .map_err(Error::from)
            .map_err(Into::into)
    }

    /// Sets `on_failed_local_media` callback, invoked on local media
    /// acquisition failures.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_failed_local_media(
        &self,
        cb: js_sys::Function,
    ) -> Result<(), JsValue> {
        self.0
            .on_failed_local_media(cb.into())
            .map_err(Error::from)
            .map_err(Into::into)
    }

    /// Sets `on_connection_loss` callback, invoked when a connection with a
    /// server is lost.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_connection_loss(
        &self,
        cb: js_sys::Function,
    ) -> Result<(), JsValue> {
        self.0
            .on_connection_loss(cb.into())
            .map_err(Error::from)
            .map_err(Into::into)
    }

    /// Updates this [`Room`]s [`MediaStreamSettings`]. This affects all
    /// [`PeerConnection`]s in this [`Room`]. If [`MediaStreamSettings`] is
    /// configured for some [`Room`], then this [`Room`] can only send media
    /// tracks that correspond to this settings. [`MediaStreamSettings`]
    /// update will change media tracks in all sending peers, so that might
    /// cause new [getUserMedia()][1] request.
    ///
    /// Media obtaining/injection errors are additionally fired to
    /// `on_failed_local_media` callback.
    ///
    /// If `stop_first` set to `true` then affected [`LocalMediaTrack`]s will be
    /// dropped before new [`MediaStreamSettings`] is applied. This is usually
    /// required when changing video source device due to hardware limitations,
    /// e.g. having an active track sourced from device `A` may hinder
    /// [getUserMedia()][1] requests to device `B`.
    ///
    /// `rollback_on_fail` option configures [`MediaStreamSettings`] update
    /// request to automatically rollback to previous settings if new settings
    /// cannot be applied.
    ///
    /// If recovering from fail state isn't possible then affected media types
    /// will be disabled.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaSettingsUpdateException`][0] if media settings could not
    /// be updated.
    ///
    /// [`LocalMediaTrack`]: crate::api::LocalMediaTrack
    /// [`PeerConnection`]: crate::peer::PeerConnection
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaSettingsUpdateException
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    pub fn set_local_media_settings(
        &self,
        settings: &MediaStreamSettings,
        stop_first: bool,
        rollback_on_fail: bool,
    ) -> Promise {
        let this = self.0.clone();
        let settings = settings.clone();

        future_to_promise(async move {
            this.set_local_media_settings(
                settings.into(),
                stop_first,
                rollback_on_fail,
            )
            .await
            .map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Mutes outbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::unmute_audio()`] was called while muting or a media server
    /// didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn mute_audio(&self, source_kind: Option<MediaSourceKind>) -> Promise {
        let this = self.0.clone();

        let fut = this.mute_audio(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Unmutes outbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::mute_audio()`] was called while unmuting or a media server
    /// didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn unmute_audio(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.unmute_audio(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Mutes outbound video in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::unmute_video()`] was called while muting or a media server
    /// didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn mute_video(&self, source_kind: Option<MediaSourceKind>) -> Promise {
        let this = self.0.clone();

        let fut = this.mute_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Unmutes outbound video in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::mute_video()`] was called while unmuting or a media server
    /// didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn unmute_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.unmute_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables outbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::enable_audio()`] was called while disabling or a media
    /// server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn disable_audio(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.disable_audio(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables outbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::disable_audio()`] was called while enabling or a media
    /// server didn't approve this state transition.
    ///
    /// With a [`LocalMediaInitException`] if a request of platform media
    /// devices access failed.
    ///
    /// [`LocalMediaInitException`]: crate::api::err::LocalMediaInitException
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn enable_audio(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.enable_audio(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables outbound video.
    ///
    /// Affects only video with a specific [`MediaSourceKind`] if specified.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::enable_video()`] was called while disabling or a media
    /// server didn't approve this state transition.
    ///
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn disable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.disable_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables outbound video.
    ///
    /// Affects only video with a specific [`MediaSourceKind`] if specified.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::disable_video()`] was called while enabling or a media
    /// server didn't approve this state transition.
    ///
    /// With a [`LocalMediaInitException`] if a request of platform media
    /// devices access failed.
    ///
    /// [`LocalMediaInitException`]: crate::api::err::LocalMediaInitException
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn enable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.enable_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables inbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::enable_remote_audio()`] was called while disabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn disable_remote_audio(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.disable_remote_audio(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables inbound video in this [`Room`].
    ///
    /// Affects only video with the specific [`MediaSourceKind`], if specified.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::enable_remote_video()`] was called while disabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn disable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.disable_remote_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables inbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::disable_remote_audio()`] was called while enabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn enable_remote_audio(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.enable_remote_audio(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables inbound video in this [`Room`].
    ///
    /// Affects only video with the specific [`MediaSourceKind`], if specified.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`RoomHandle::disable_remote_video()`] was called while enabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn enable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        let fut = this.enable_remote_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }
}
