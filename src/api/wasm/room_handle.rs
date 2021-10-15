//! JS side handle to a [`Room`].
//!
//! [`Room`]: room::Room

use derive_more::{From, Into};
use js_sys::Promise;
use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::future_to_promise;

use crate::{
    api::{MediaSourceKind, MediaStreamSettings},
    room,
};

use super::Error;

/// JS side handle to a [`Room`] where all the media happens.
///
/// Like all handles it contains a weak reference to the object that is managed
/// by Rust, so its methods will fail if a weak reference could not be upgraded.
///
/// [`Room`]: room::Room
#[wasm_bindgen]
#[derive(From, Into)]
pub struct RoomHandle(room::RoomHandle);

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
    /// Throws [`StateError`] if the underlying pointer has been freed or if
    /// some mandatory callback is not set. These callbacks are:
    /// [`RoomHandle::on_connection_loss`] and
    /// [`RoomHandle::on_failed_local_media`].
    ///
    /// Throws [`FormatException`] if the provided `token` string has bad
    /// format.
    ///
    /// Throws [`RpcClientException`] if could not connect to media server.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`FormatException`]: crate::api::err::FormatException
    /// [`RpcClientException`]: crate::api::err::RpcClientException
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
    /// Throws [`StateError`] if the underlying pointer has been freed.
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
    /// Throws [`StateError`] if the underlying pointer has been freed.
    ///
    /// [`Room`]: room::Room
    /// [`RoomCloseReason`]: room::RoomCloseReason
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_close(&self, cb: js_sys::Function) -> Result<(), JsValue> {
        self.0
            .on_close(cb.into())
            .map_err(Error::from)
            .map_err(Into::into)
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
    /// Throws [`StateError`] if the underlying pointer has been freed.
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
    /// Throws [`StateError`] if the underlying pointer has been freed.
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
    /// Throws [`StateError`] if the underlying pointer has been freed.
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
    /// Throws [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaSettingsUpdateException`] if settings could not be
    /// updated.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [`Room`]: room::Room
    /// [`PeerConnection`]: crate::peer::PeerConnection
    /// [`LocalMediaTrack`]: crate::api::LocalMediaTrack
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaSettingsUpdateException`]:
    /// crate::api::err::MediaSettingsUpdateException
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
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::unmute_audio`] was called while muting or a media server
    /// didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn mute_audio(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.mute_audio().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Unmutes outbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if [`RoomHandle::mute_audio`]
    /// was called while unmuting or a media server didn't approve this state
    /// transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn unmute_audio(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.unmute_audio().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Mutes outbound video in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::unmute_video`] was called while muting or a media server
    /// didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn mute_video(&self, source_kind: Option<MediaSourceKind>) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.mute_video(source_kind.map(Into::into))
                .await
                .map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Unmutes outbound video in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if [`RoomHandle::mute_video`]
    /// was called while unmuting or a media server didn't approve this state
    /// transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn unmute_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.unmute_video(source_kind.map(Into::into))
                .await
                .map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables outbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::enable_audio`] was called while disabling or a media
    /// server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn disable_audio(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.disable_audio().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables outbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::disable_audio`] was called while enabling or a media
    /// server didn't approve this state transition.
    ///
    /// Throws a [`LocalMediaInitException`] if a request of platform media
    /// devices access failed.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    /// [`LocalMediaInitException`]: crate::api::err::LocalMediaInitException
    pub fn enable_audio(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.enable_audio().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables outbound video.
    ///
    /// Affects only video with a specific [`MediaSourceKind`] if specified.
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::enable_video`] was called while disabling or a media
    /// server didn't approve this state transition.
    ///
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn disable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.disable_video(source_kind.map(Into::into))
                .await
                .map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables outbound video.
    ///
    /// Affects only video with a specific [`MediaSourceKind`] if specified.
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::disable_video`] was called while enabling or a media
    /// server didn't approve this state transition.
    ///
    /// Throws a [`LocalMediaInitException`] if a request of platform media
    /// devices access failed.
    ///
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    /// [`LocalMediaInitException`]: crate::api::err::LocalMediaInitException
    pub fn enable_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.enable_video(source_kind.map(Into::into))
                .await
                .map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables inbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::enable_remote_audio`] was called while disabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn disable_remote_audio(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.disable_remote_audio().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables inbound video in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::enable_remote_video`] was called while disabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn disable_remote_video(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.disable_remote_video().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables inbound audio in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::disable_remote_audio`] was called while enabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn enable_remote_audio(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.enable_remote_audio().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables inbound video in this [`Room`].
    ///
    /// # Errors
    ///
    /// Throws a [`StateError`] if the underlying pointer has been freed.
    ///
    /// Throws a [`MediaStateTransitionException`] if
    /// [`RoomHandle::disable_remote_video`] was called while enabling or a
    /// media server didn't approve this state transition.
    ///
    /// [`Room`]: room::Room
    /// [`StateError`]: crate::api::err::StateError
    /// [`MediaStateTransitionException`]:
    /// crate::api::err::MediaStateTransitionException
    pub fn enable_remote_video(&self) -> Promise {
        let this = self.0.clone();

        future_to_promise(async move {
            this.enable_remote_video().await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }
}
