use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_room_close_reason_reason(
    room_close_reason: *mut wire_RoomCloseReason,
) -> support::WireSyncReturnStruct {
    wire_room_close_reason_reason_impl(room_close_reason)
}

#[no_mangle]
pub extern "C" fn wire_room_close_reason_is_closed_by_server(
    room_close_reason: *mut wire_RoomCloseReason,
) -> support::WireSyncReturnStruct {
    wire_room_close_reason_is_closed_by_server_impl(room_close_reason)
}

#[no_mangle]
pub extern "C" fn wire_room_close_reason_is_err(
    room_close_reason: *mut wire_RoomCloseReason,
) -> support::WireSyncReturnStruct {
    wire_room_close_reason_is_err_impl(room_close_reason)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RoomCloseReason() -> *mut wire_RoomCloseReason {
    support::new_leak_box_ptr(wire_RoomCloseReason::new_with_null_ptr())
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<RoomCloseReason>> for *mut wire_RoomCloseReason {
    fn wire2api(self) -> Opaque<RoomCloseReason> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_RoomCloseReason {
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

impl NewWithNullPtr for wire_RoomCloseReason {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
