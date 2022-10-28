use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_reconnect_handle_reconnect_with_delay(
    reconnect_handle: *mut wire_ReconnectHandle,
    delay_ms: i64,
) -> support::WireSyncReturnStruct {
    wire_reconnect_handle_reconnect_with_delay_impl(reconnect_handle, delay_ms)
}

#[no_mangle]
pub extern "C" fn wire_reconnect_handle_reconnect_with_backoff(
    reconnect_handle: *mut wire_ReconnectHandle,
    starting_delay: i64,
    multiplier: f64,
    max_delay: u32,
    max_elapsed_time_ms: *mut u32,
) -> support::WireSyncReturnStruct {
    wire_reconnect_handle_reconnect_with_backoff_impl(
        reconnect_handle,
        starting_delay,
        multiplier,
        max_delay,
        max_elapsed_time_ms,
    )
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_ReconnectHandle() -> *mut wire_ReconnectHandle {
    support::new_leak_box_ptr(wire_ReconnectHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_10(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

// Section: deallocate functions

#[no_mangle]
pub extern "C" fn drop_box_autoadd_u32_10(raw: *mut u32) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

// Section: impl Wire2Api

impl Wire2Api<Opaque<ReconnectHandle>> for *mut wire_ReconnectHandle {
    fn wire2api(self) -> Opaque<ReconnectHandle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_ReconnectHandle {
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

impl NewWithNullPtr for wire_ReconnectHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
