use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_dart_future_to_usize(
    handle: *mut wire_MyDartFuture,
) -> support::WireSyncReturnStruct {
    wire_dart_future_to_usize_impl(handle)
}

#[no_mangle]
pub extern "C" fn wire_dart_handle_to_connection_handle_dh(
    handle: usize,
) -> support::WireSyncReturnStruct {
    wire_dart_handle_to_connection_handle_dh_impl(handle)
}

#[no_mangle]
pub extern "C" fn wire_dart_handle_to_media_manager_handle_dh(
    handle: usize,
) -> support::WireSyncReturnStruct {
    wire_dart_handle_to_media_manager_handle_dh_impl(handle)
}

#[no_mangle]
pub extern "C" fn wire_dart_handle_to_remote_media_track_dh(
    handle: usize,
) -> support::WireSyncReturnStruct {
    wire_dart_handle_to_remote_media_track_dh_impl(handle)
}

#[no_mangle]
pub extern "C" fn wire_dart_handle_to_room_handle_dh(
    handle: usize,
) -> support::WireSyncReturnStruct {
    wire_dart_handle_to_room_handle_dh_impl(handle)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MyDartFuture() -> *mut wire_MyDartFuture {
    support::new_leak_box_ptr(wire_MyDartFuture::new_with_null_ptr())
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<MyDartFuture>> for *mut wire_MyDartFuture {
    fn wire2api(self) -> Opaque<MyDartFuture> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyDartFuture {
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

impl NewWithNullPtr for wire_MyDartFuture {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
