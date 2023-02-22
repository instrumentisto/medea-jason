//! General JS side library interface.

use derive_more::From;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{
    api::{MediaManagerHandle, RoomHandle},
    jason,
};

/// General JS side library interface.
///
/// Responsible for managing shared transports, local media and room
/// initialization.
#[wasm_bindgen]
#[derive(Debug, Default, From)]
pub struct Jason(jason::Jason);

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Jason {
    /// Instantiates a new [`Jason`] interface to interact with this library.
    #[must_use]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(jason::Jason::new())
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
    #[allow(unused_must_use)]
    pub fn close_room(&self, room_to_delete: RoomHandle) -> Promise {
        let this = self.0.clone();
        future_to_promise(async move {
            this.close_room(room_to_delete.into()).await;
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
