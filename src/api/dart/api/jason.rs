use derive_more::From;
use flutter_rust_bridge::frb;

use crate::{
    api::{self, api::DART_HANDLER_PORT},
    jason,
};

/// General library interface.
///
/// Responsible for managing shared transports, local media and room
/// initialization.
#[derive(Debug, From)]
#[frb(opaque)]
pub struct Jason(jason::Jason);

// Only used on single thread
unsafe impl Send for Jason {}
unsafe impl Sync for Jason {}

impl Jason {
    #[frb(sync)]
    pub fn new(dart_handler_port: i64) -> Self {
        unsafe {
            DART_HANDLER_PORT.replace(dart_handler_port);
        }

        Self(jason::Jason::new(None))
    }

    /// Creates a new [`Room`] and returns its [`RoomHandle`].
    ///
    /// [`Room`]: room::Room
    #[frb(sync)]
    #[must_use]
    pub fn jason_init_room(&self) -> api::RoomHandle {
        api::RoomHandle(self.0.init_room())
    }

    /// Returns a [`MediaManagerHandle`].
    #[frb(sync)]
    #[must_use]
    pub fn jason_media_manager(&self) -> api::MediaManagerHandle {
        api::MediaManagerHandle(self.0.media_manager())
    }

    /// Closes the provided [`RoomHandle`].
    #[frb(sync)]
    #[must_use]
    pub fn jason_close_room(&self, room_to_delete: api::RoomHandle) {
        self.0.close_room(&room_to_delete.0);
    }

    /// Closes the provided [`RoomHandle`].
    #[frb(sync)]
    #[must_use]
    pub fn jason_dispose(self) {
        self.0.dispose();
    }
}
