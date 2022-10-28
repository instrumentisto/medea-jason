use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_room_handle_join(
    room_handle: *mut wire_RoomHandle,
    token: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_room_handle_join_impl(room_handle, token)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_cast(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_cast_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_set_local_media_settings(
    room_handle: *mut wire_RoomHandle,
    settings: *mut wire_RoomHandleMs,
    stop_first: bool,
    rollback_on_fail: bool,
) -> support::WireSyncReturnStruct {
    wire_room_handle_set_local_media_settings_impl(
        room_handle,
        settings,
        stop_first,
        rollback_on_fail,
    )
}

#[no_mangle]
pub extern "C" fn wire_room_handle_mute_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_mute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_unmute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_mute_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_mute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_unmute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_new_connection(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_RoomHandleDh,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_new_connection_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_close(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_RoomHandleDh,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_close_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_local_track(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_RoomHandleDh,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_local_track_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_connection_loss(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_RoomHandleDh,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_connection_loss_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_failed_local_media(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_RoomHandleDh,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_failed_local_media_impl(room_handle, cb)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RoomHandle() -> *mut wire_RoomHandle {
    support::new_leak_box_ptr(wire_RoomHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RoomHandleDh() -> *mut wire_RoomHandleDh {
    support::new_leak_box_ptr(wire_RoomHandleDh::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RoomHandleMs() -> *mut wire_RoomHandleMs {
    support::new_leak_box_ptr(wire_RoomHandleMs::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u8_13(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_13(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: deallocate functions

#[no_mangle]
pub extern "C" fn drop_box_autoadd_u8_13(raw: *mut u8) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

// Section: impl Wire2Api

impl Wire2Api<Opaque<RoomHandle>> for *mut wire_RoomHandle {
    fn wire2api(self) -> Opaque<RoomHandle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RoomHandleDH>> for *mut wire_RoomHandleDh {
    fn wire2api(self) -> Opaque<RoomHandleDH> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RoomHandleMS>> for *mut wire_RoomHandleMs {
    fn wire2api(self) -> Opaque<RoomHandleMS> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_RoomHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RoomHandleDh {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RoomHandleMs {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
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

impl NewWithNullPtr for wire_RoomHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RoomHandleDh {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RoomHandleMs {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
