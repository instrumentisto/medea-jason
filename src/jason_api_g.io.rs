use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_jason_new() -> support::WireSyncReturnStruct {
    wire_jason_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_jason_init_room(
    jason: *mut wire_Jason,
) -> support::WireSyncReturnStruct {
    wire_jason_init_room_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_media_manager(
    jason: *mut wire_Jason,
) -> support::WireSyncReturnStruct {
    wire_jason_media_manager_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_close_room(
    jason: *mut wire_Jason,
    room_to_delete: *mut wire_JasonRh,
) -> support::WireSyncReturnStruct {
    wire_jason_close_room_impl(jason, room_to_delete)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_Jason() -> *mut wire_Jason {
    support::new_leak_box_ptr(wire_Jason::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_JasonRh() -> *mut wire_JasonRh {
    support::new_leak_box_ptr(wire_JasonRh::new_with_null_ptr())
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<Jason>> for *mut wire_Jason {
    fn wire2api(self) -> Opaque<Jason> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<JasonRH>> for *mut wire_JasonRh {
    fn wire2api(self) -> Opaque<JasonRH> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_Jason {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JasonRh {
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

impl NewWithNullPtr for wire_Jason {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_JasonRh {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(
    val: support::WireSyncReturnStruct,
) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
