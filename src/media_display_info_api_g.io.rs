use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_media_display_info_device_id(
    media_display: *mut wire_MediaDisplayInfo,
) -> support::WireSyncReturnStruct {
    wire_media_display_info_device_id_impl(media_display)
}

#[no_mangle]
pub extern "C" fn wire_media_display_info_title(
    media_display: *mut wire_MediaDisplayInfo,
) -> support::WireSyncReturnStruct {
    wire_media_display_info_title_impl(media_display)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MediaDisplayInfo() -> *mut wire_MediaDisplayInfo {
    support::new_leak_box_ptr(wire_MediaDisplayInfo::new_with_null_ptr())
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<MediaDisplayInfo>> for *mut wire_MediaDisplayInfo {
    fn wire2api(self) -> Opaque<MediaDisplayInfo> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MediaDisplayInfo {
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

impl NewWithNullPtr for wire_MediaDisplayInfo {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
