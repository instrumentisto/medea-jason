use std::panic::{RefUnwindSafe, UnwindSafe};

use derive_more::From;
use flutter_rust_bridge::frb;

use crate::api::{self};

#[derive(Debug, From)]
#[frb(opaque)]
pub struct JasonHandle(crate::jason::Jason);

impl JasonHandle {
    #[frb(sync)]
    pub fn new() -> JasonHandle {
        Self(crate::jason::Jason::new(None))
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

impl RefUnwindSafe for JasonHandle {}
impl UnwindSafe for JasonHandle {}
unsafe impl Send for JasonHandle {}
unsafe impl Sync for JasonHandle {}
