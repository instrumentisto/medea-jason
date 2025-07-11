use flutter_rust_bridge::frb;
use send_wrapper::SendWrapper;

use crate::{api, jason};
#[cfg(doc)]
use crate::{
    api::{MediaManagerHandle, RoomHandle},
    room::Room,
};

/// General library interface.
///
/// Responsible for managing shared transports, local media and room
/// initialization.
#[derive(Debug)]
#[frb(opaque)]
pub struct Jason(SendWrapper<jason::JasonImpl>);

impl From<jason::JasonImpl> for Jason {
    fn from(value: jason::JasonImpl) -> Self {
        Self(SendWrapper::new(value))
    }
}

impl Jason {
    /// Instantiates a new [`Jason`] interface to interact with this library.
    #[frb(sync)]
    #[must_use]
    pub fn new() -> Self {
        Self(SendWrapper::new(jason::JasonImpl::new(None)))
    }

    /// Creates a new [`Room`] and returns its [`RoomHandle`].
    #[frb(sync)]
    #[must_use]
    pub fn jason_init_room(&self) -> api::RoomHandle {
        self.0.init_room().into()
    }

    /// Returns a [`MediaManagerHandle`].
    #[frb(sync)]
    #[must_use]
    pub fn jason_media_manager(&self) -> api::MediaManagerHandle {
        self.0.media_manager().into()
    }

    /// Closes the provided [`RoomHandle`].
    #[frb(sync)]
    #[must_use]
    pub fn jason_close_room(&self, room_to_delete: api::RoomHandle) {
        self.0.close_room(&room_to_delete.0);
    }

    /// Closes this [`Jason`].
    #[frb(sync)]
    #[must_use]
    pub fn jason_dispose(self) {
        self.0.take().dispose();
    }
}
