use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_remote_media_track_get_track(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_muted(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_RemoteMediaTrackDh,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_muted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_unmuted(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_RemoteMediaTrackDh,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_unmuted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_stopped(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_RemoteMediaTrackDh,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_stopped_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_media_direction_changed(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_RemoteMediaTrackDh,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_media_direction_changed_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_muted(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_muted_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_kind(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_source_kind(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_media_source_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_direction(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_media_direction_impl(track)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RemoteMediaTrack() -> *mut wire_RemoteMediaTrack {
    support::new_leak_box_ptr(wire_RemoteMediaTrack::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RemoteMediaTrackDh() -> *mut wire_RemoteMediaTrackDh {
    support::new_leak_box_ptr(wire_RemoteMediaTrackDh::new_with_null_ptr())
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<RemoteMediaTrack>> for *mut wire_RemoteMediaTrack {
    fn wire2api(self) -> Opaque<RemoteMediaTrack> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RemoteMediaTrackDH>> for *mut wire_RemoteMediaTrackDh {
    fn wire2api(self) -> Opaque<RemoteMediaTrackDH> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_RemoteMediaTrack {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RemoteMediaTrackDh {
    ptr: *const core::ffi::c_void,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_RemoteMediaTrack {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RemoteMediaTrackDh {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
