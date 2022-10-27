use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_close(
    connection: *mut wire_ConnectionHandle,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_close_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_remote_track_added(
    connection: *mut wire_ConnectionHandle,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_remote_track_added_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_quality_score_update(
    connection: *mut wire_ConnectionHandle,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_quality_score_update_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_get_remote_member_id(
    connection: *mut wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_get_remote_member_id_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_audio(
    connection: *mut wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_enable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_audio(
    connection: *mut wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_disable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_video(
    connection: *mut wire_ConnectionHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_enable_remote_video_impl(connection, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_video(
    connection: *mut wire_ConnectionHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_disable_remote_video_impl(connection, source_kind)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_ConnectionHandle() -> *mut wire_ConnectionHandle {
    support::new_leak_box_ptr(wire_ConnectionHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_DartHandle() -> *mut wire_DartHandle {
    support::new_leak_box_ptr(wire_DartHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u8_2(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

// Section: deallocate functions

#[no_mangle]
pub extern "C" fn drop_box_autoadd_u8_2(raw: *mut u8) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

// Section: impl Wire2Api

impl Wire2Api<Opaque<ConnectionHandle>> for *mut wire_ConnectionHandle {
    fn wire2api(self) -> Opaque<ConnectionHandle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<Dart_Handle>> for *mut wire_DartHandle {
    fn wire2api(self) -> Opaque<Dart_Handle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConnectionHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DartHandle {
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

impl NewWithNullPtr for wire_ConnectionHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_DartHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
