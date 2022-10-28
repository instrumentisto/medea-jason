

// ConnectionHandleDH;
// MediaManagerHandleDH;
// RemoteMediaTrackDH;
// RoomHandleDH;

pub use dart_sys::Dart_Handle;
use flutter_rust_bridge::{SyncReturn, Opaque};

use super::{jason_api::MyDartFuture, connection_handle_api::ConnectionHandleDH, media_manager_handle_api::MediaManagerHandleDH, remote_media_track_api::RemoteMediaTrackDH, room_handle_api::RoomHandleDH};

pub fn dart_future_to_usize(handle: Opaque<MyDartFuture>) -> SyncReturn<usize> {
    SyncReturn((&handle).handle as _)
}

pub fn dart_handle_to_connection_handle_dh(handle: usize) -> SyncReturn<Opaque<ConnectionHandleDH>> {
    SyncReturn(Opaque::new(handle as _))
}
pub fn dart_handle_to_media_manager_handle_dh(handle: usize) -> SyncReturn<Opaque<MediaManagerHandleDH>> {
    SyncReturn(Opaque::new(handle as _))
}
pub fn dart_handle_to_remote_media_track_dh(handle: usize) -> SyncReturn<Opaque<RemoteMediaTrackDH>> {
    SyncReturn(Opaque::new(handle as _))
}
pub fn dart_handle_to_room_handle_dh(handle: usize) -> SyncReturn<Opaque<RoomHandleDH>> {
    SyncReturn(Opaque::new(handle as _))
}