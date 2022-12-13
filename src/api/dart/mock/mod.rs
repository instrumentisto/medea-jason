pub mod connection_handle;
pub mod jason;
pub mod local_media_track;
pub mod media_device_info;
pub mod media_display_info;
pub mod media_manager_handle;
pub mod reconnect_handle;
pub mod remote_media_track;
pub mod room_handle;

// TODO(alexlapa): Can be deleted? We dont really need FFI tests if we use
//                 frb. mockable trait can also be removed.
pub use super::mock::{
    connection_handle::ConnectionHandle, jason::Jason,
    local_media_track::LocalMediaTrack, media_device_info::MediaDeviceInfo,
    media_display_info::MediaDisplayInfo,
    media_manager_handle::MediaManagerHandle,
    reconnect_handle::ReconnectHandle, remote_media_track::RemoteMediaTrack,
    room_handle::RoomHandle,
};
