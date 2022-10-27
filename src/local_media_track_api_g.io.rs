use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_local_media_track_get_track(
    track: *mut wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_kind(
    track: *mut wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_media_source_kind(
    track: *mut wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_media_source_kind_impl(track)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_LocalMediaTrack() -> *mut wire_LocalMediaTrack {
    support::new_leak_box_ptr(wire_LocalMediaTrack::new_with_null_ptr())
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<LocalMediaTrack>> for *mut wire_LocalMediaTrack {
    fn wire2api(self) -> Opaque<LocalMediaTrack> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_LocalMediaTrack {
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

impl NewWithNullPtr for wire_LocalMediaTrack {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
