//! General JS side library interface.

use derive_more::with_trait::From;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{
    api::{Error, MediaManagerHandle, RoomHandle},
    jason,
};

/// General JS side library interface.
///
/// Responsible for managing shared transports, local media and room
/// initialization.
#[wasm_bindgen]
#[derive(Debug, Default, From)]
pub struct Jason(jason::JasonImpl);

#[wasm_bindgen]
impl Jason {
    /// Instantiates a new [`Jason`] interface to interact with this library.
    #[must_use]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(jason::JasonImpl::new(None))
    }

    /// Creates a new `Room` and returns its [`RoomHandle`].
    #[must_use]
    pub fn init_room(&self) -> RoomHandle {
        self.0.init_room().into()
    }

    /// Returns a [`MediaManagerHandle`].
    #[must_use]
    pub fn media_manager(&self) -> MediaManagerHandle {
        self.0.media_manager().into()
    }

    /// Closes the provided [`RoomHandle`].
    pub fn close_room(&self, room_to_delete: RoomHandle) {
        self.0.close_room(&room_to_delete.into());
    }

    /// Notifies [`Jason`] about a network change event (interface switch or
    /// similar).
    ///
    /// Drops and recreates active connections and schedules [ICE] restart after
    /// reconnection.
    ///
    /// [ICE]: https://webrtcglossary.com/ice
    pub fn network_changed(&self) -> Promise {
        let fut = self.0.network_changed();

        future_to_promise(async move {
            fut.await.map_err(Error::from)?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// Drops [`Jason`] API object, so all the related objects (rooms,
    /// connections, streams etc.) respectively. All objects related to this
    /// [`Jason`] API object will be detached (you will still hold them, but
    /// unable to use).
    pub fn dispose(self) {
        self.0.dispose();
    }
}
