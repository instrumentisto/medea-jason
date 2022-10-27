use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_media_device_info_device_id(
    media_device: *mut wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_device_id_impl(media_device)
}

#[no_mangle]
pub extern "C" fn wire_media_device_info_kind(
    media_device: *mut wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_kind_impl(media_device)
}

#[no_mangle]
pub extern "C" fn wire_media_device_info_label(
    media_device: *mut wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_label_impl(media_device)
}

#[no_mangle]
pub extern "C" fn wire_media_device_info_group_id(
    media_device: *mut wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_group_id_impl(media_device)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MediaDeviceInfo() -> *mut wire_MediaDeviceInfo {
    support::new_leak_box_ptr(wire_MediaDeviceInfo::new_with_null_ptr())
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<MediaDeviceInfo>> for *mut wire_MediaDeviceInfo {
    fn wire2api(self) -> Opaque<MediaDeviceInfo> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MediaDeviceInfo {
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

impl NewWithNullPtr for wire_MediaDeviceInfo {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
