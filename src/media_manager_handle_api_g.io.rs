use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_init_local_tracks(
    manager: *mut wire_MediaManagerHandle,
    caps: *mut wire_MediaStreamSettings,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_init_local_tracks_impl(manager, caps)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_enumerate_devices(
    manager: *mut wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_enumerate_devices_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_enumerate_displays(
    manager: *mut wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_enumerate_displays_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_set_output_audio_id(
    manager: *mut wire_MediaManagerHandle,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_set_output_audio_id_impl(manager, device_id)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_set_microphone_volume(
    manager: *mut wire_MediaManagerHandle,
    level: i64,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_set_microphone_volume_impl(manager, level)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_microphone_volume_is_available(
    manager: *mut wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_microphone_volume_is_available_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_microphone_volume(
    manager: *mut wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_microphone_volume_impl(manager)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MediaManagerHandle() -> *mut wire_MediaManagerHandle {
    support::new_leak_box_ptr(wire_MediaManagerHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_MediaStreamSettings() -> *mut wire_MediaStreamSettings {
    support::new_leak_box_ptr(wire_MediaStreamSettings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_8(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<MediaManagerHandle>> for *mut wire_MediaManagerHandle {
    fn wire2api(self) -> Opaque<MediaManagerHandle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<MediaStreamSettings>> for *mut wire_MediaStreamSettings {
    fn wire2api(self) -> Opaque<MediaStreamSettings> {
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
pub struct wire_MediaManagerHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MediaStreamSettings {
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

impl NewWithNullPtr for wire_MediaManagerHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MediaStreamSettings {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
