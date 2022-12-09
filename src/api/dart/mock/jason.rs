#![allow(
    clippy::new_without_default,
    clippy::unused_self,
    unused_tuple_struct_fields
)]

use crate::api::{MediaManagerHandle, RoomHandle};

#[derive(Clone, Copy, Debug)]
pub struct Jason(u8);

impl Jason {
    #[must_use]
    pub fn new() -> Self {
        crate::platform::init_logger();
        Self(0)
    }

    #[must_use]
    pub const fn init_room(&self) -> RoomHandle {
        RoomHandle(0)
    }

    #[must_use]
    pub const fn media_manager(&self) -> MediaManagerHandle {
        MediaManagerHandle(0)
    }

    pub const fn close_room(&self, _: RoomHandle) {}

    pub const fn dispose(self) {}
}
