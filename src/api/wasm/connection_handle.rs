//! Connection with a specific remote `Member` used on JS side.

use derive_more::with_trait::From;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{api, connection};

/// Connection with a specific remote `Member`, that is used on JS side.
///
/// Like all the handles it contains a weak reference to the object that is
/// managed by Rust, so its methods will fail if a weak reference could not be
/// upgraded.
#[wasm_bindgen]
#[derive(Debug, From)]
pub struct ConnectionHandle(connection::ConnectionHandle);

#[wasm_bindgen]
impl ConnectionHandle {
    /// Sets callback, invoked when this [`Connection`] is closed.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if an underlying object has been disposed, e.g.
    /// `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    /// a [`RoomHandle`] that implicitly owns native object behind this
    /// [`ConnectionHandle`].
    ///
    /// [`Connection`]: connection::Connection
    /// [`Jason`]: api::Jason
    /// [`RoomHandle`]: api::RoomHandle
    /// [`StateError`]: api::err::StateError
    pub fn on_close(&self, cb: js_sys::Function) -> Result<(), JsValue> {
        self.0.on_close(cb.into()).map_err(api::Error::from).map_err(Into::into)
    }

    /// Returns ID of the remote `Member`.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if an underlying object has been disposed, e.g.
    /// `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    /// a [`RoomHandle`] that implicitly owns native object behind this
    /// [`ConnectionHandle`].
    ///
    /// [`Jason`]: api::Jason
    /// [`RoomHandle`]: api::RoomHandle
    /// [`StateError`]: crate::api::err::StateError
    pub fn get_remote_member_id(&self) -> Result<String, JsValue> {
        self.0
            .get_remote_member_id()
            .map_err(api::Error::from)
            .map_err(Into::into)
    }

    /// Returns `MemberConnectionState` of the [`Connection`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if an underlying object has been disposed, e.g.
    /// `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    /// a [`RoomHandle`] that implicitly owns native object behind this
    /// [`ConnectionHandle`].
    ///
    /// [`Connection`]: connection::Connection
    /// [`Jason`]: api::Jason
    /// [`RoomHandle`]: api::RoomHandle
    /// [`StateError`]: crate::api::err::StateError
    pub fn get_state(
        &self,
    ) -> Result<Option<api::MemberConnectionState>, JsValue> {
        self.0
            .get_state()
            .map(|state| state.map(Into::into))
            .map_err(api::Error::from)
            .map_err(Into::into)
    }

    /// Sets a callback to be invoked once a state of associated [`Connection`]
    /// is changed.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if an underlying object has been disposed, e.g.
    /// `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    /// a [`RoomHandle`] that implicitly owns native object behind this
    /// [`ConnectionHandle`].
    ///
    /// [`Connection`]: connection::Connection
    /// [`Jason`]: api::Jason
    /// [`RoomHandle`]: api::RoomHandle
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_state_change(&self, cb: js_sys::Function) -> Result<(), JsValue> {
        self.0
            .on_state_change(cb.into())
            .map_err(api::Error::from)
            .map_err(Into::into)
    }

    /// Sets callback, invoked when a new [`RemoteMediaTrack`] is added to this
    /// [`Connection`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if an underlying object has been disposed, e.g.
    /// `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    /// a [`RoomHandle`] that implicitly owns native object behind this
    /// [`ConnectionHandle`].
    ///
    /// [`Connection`]: connection::Connection
    /// [`Jason`]: api::Jason
    /// [`RemoteMediaTrack`]: crate::api::RemoteMediaTrack
    /// [`RoomHandle`]: api::RoomHandle
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_remote_track_added(
        &self,
        cb: js_sys::Function,
    ) -> Result<(), JsValue> {
        self.0
            .on_remote_track_added(cb.into())
            .map_err(api::Error::from)
            .map_err(Into::into)
    }

    /// Sets callback, invoked when connection quality score is updated by a
    /// server.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if an underlying object has been disposed, e.g.
    /// `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    /// a [`RoomHandle`] that implicitly owns native object behind this
    /// [`ConnectionHandle`].
    ///
    /// [`Jason`]: api::Jason
    /// [`RoomHandle`]: api::RoomHandle
    /// [`StateError`]: crate::api::err::StateError
    pub fn on_quality_score_update(
        &self,
        cb: js_sys::Function,
    ) -> Result<(), JsValue> {
        self.0
            .on_quality_score_update(cb.into())
            .map_err(api::Error::from)
            .map_err(Into::into)
    }

    /// Enables inbound audio in this [`ConnectionHandle`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`ConnectionHandle::disable_remote_video()`] was called while enabling
    /// or a media server didn't approve this state transition.
    ///
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn enable_remote_audio(&self) -> Promise {
        let fut = self.0.enable_remote_audio();
        future_to_promise(async move {
            fut.await.map_err(api::Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables inbound audio in this [`ConnectionHandle`].
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`ConnectionHandle::enable_remote_video()`] was called while disabling
    /// or a media server didn't approve this state transition.
    ///
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn disable_remote_audio(&self) -> Promise {
        let fut = self.0.disable_remote_audio();
        future_to_promise(async move {
            fut.await.map_err(api::Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Enables inbound video in this [`ConnectionHandle`].
    ///
    /// Affects only video with the specific [`MediaSourceKind`], if specified.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`ConnectionHandle::disable_remote_audio()`] was called while enabling
    /// or a media server didn't approve this state transition.
    ///
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn enable_remote_video(
        &self,
        source_kind: Option<api::MediaSourceKind>,
    ) -> Promise {
        let fut = self.0.enable_remote_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(api::Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Disables inbound video in this [`ConnectionHandle`].
    ///
    /// Affects only video with the specific [`MediaSourceKind`], if specified.
    ///
    /// # Errors
    ///
    /// With a [`StateError`] if the underlying pointer has been freed.
    ///
    /// With a [`MediaStateTransitionException`][0] if
    /// [`ConnectionHandle::enable_remote_audio()`] was called while disabling
    /// or a media server didn't approve this state transition.
    ///
    /// [`StateError`]: crate::api::err::StateError
    /// [0]: crate::api::err::MediaStateTransitionException
    pub fn disable_remote_video(
        &self,
        source_kind: Option<api::MediaSourceKind>,
    ) -> Promise {
        let fut = self.0.disable_remote_video(source_kind.map(Into::into));
        future_to_promise(async move {
            fut.await.map_err(api::Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }
}
