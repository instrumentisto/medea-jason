use flutter_rust_bridge::{Opaque, SyncReturn};

pub use crate::room::RoomCloseReason;

/// Returns a close reason of a [`Room`].
///
/// [`Room`]: crate::room::Room
pub fn room_close_reason_reason(
    room_close_reason: Opaque<RoomCloseReason>,
) -> SyncReturn<String> {
    SyncReturn(room_close_reason.reason())
}

/// Indicates whether a [`Room`] was closed by server.
///
/// [`Room`]: crate::room::Room
pub fn room_close_reason_is_closed_by_server(
    room_close_reason: Opaque<RoomCloseReason>,
) -> SyncReturn<bool> {
    SyncReturn(room_close_reason.is_closed_by_server())
}

/// Indicates whether a [`Room`]'s close reason is considered as an error.
///
/// [`Room`]: crate::room::Room
pub fn room_close_reason_is_err(
    room_close_reason: Opaque<RoomCloseReason>,
) -> SyncReturn<bool> {
    SyncReturn(room_close_reason.is_err())
}
