use flutter_rust_bridge::{Opaque, SyncReturn};

#[cfg(feature = "mockable")]
pub use self::mock::Jason;
use crate::api::Error;
pub use crate::api::{MediaManagerHandle, RoomHandle};
#[cfg(not(feature = "mockable"))]
pub use crate::jason::Jason;

pub use super::{
    media_manager_handle_api::MyDartFuture, utils::IntoDartFuture,
};

pub type JasonRH = RoomHandle;

/// Instantiates a new [`Jason`] interface to interact with this library.
pub fn jason_new() -> SyncReturn<Opaque<Jason>> {
    SyncReturn(Opaque::new(Jason::new()))
}

/// Creates a new [`Room`] and returns its [`RoomHandle`].
///
/// [`Room`]: crate::room::Room
pub fn jason_init_room(jason: Opaque<Jason>) -> SyncReturn<Opaque<RoomHandle>> {
    SyncReturn(Opaque::new(jason.init_room()))
}

/// Returns a [`MediaManagerHandle`].
pub fn jason_media_manager(
    jason: Opaque<Jason>,
) -> SyncReturn<Opaque<MediaManagerHandle>> {
    SyncReturn(Opaque::new(jason.media_manager()))
}

/// Closes the provided [`RoomHandle`].
pub fn jason_close_room(
    jason: Opaque<Jason>,
    room_to_delete: Opaque<JasonRH>,
) -> SyncReturn<()> {
    jason.close_room(RoomHandle::clone(&room_to_delete));
    SyncReturn(())
}
