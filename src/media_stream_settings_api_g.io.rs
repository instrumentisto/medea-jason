use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_new(
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_audio(
    media_stream_settings: *mut wire_RefCellMediaStreamSettings,
    constraints: *mut wire_AudioTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_audio_impl(media_stream_settings, constraints)
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_device_video(
    media_stream_settings: *mut wire_RefCellMediaStreamSettings,
    constraints: *mut wire_DeviceVideoTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_device_video_impl(
        media_stream_settings,
        constraints,
    )
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_display_video(
    media_stream_settings: *mut wire_RefCellMediaStreamSettings,
    constraints: *mut wire_DisplayVideoTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_display_video_impl(
        media_stream_settings,
        constraints,
    )
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_AudioTrackConstraints() -> *mut wire_AudioTrackConstraints
{
    support::new_leak_box_ptr(wire_AudioTrackConstraints::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_DeviceVideoTrackConstraints(
) -> *mut wire_DeviceVideoTrackConstraints {
    support::new_leak_box_ptr(
        wire_DeviceVideoTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_DisplayVideoTrackConstraints(
) -> *mut wire_DisplayVideoTrackConstraints {
    support::new_leak_box_ptr(
        wire_DisplayVideoTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RefCellMediaStreamSettings(
) -> *mut wire_RefCellMediaStreamSettings {
    support::new_leak_box_ptr(
        wire_RefCellMediaStreamSettings::new_with_null_ptr(),
    )
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<AudioTrackConstraints>>
    for *mut wire_AudioTrackConstraints
{
    fn wire2api(self) -> Opaque<AudioTrackConstraints> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<DeviceVideoTrackConstraints>>
    for *mut wire_DeviceVideoTrackConstraints
{
    fn wire2api(self) -> Opaque<DeviceVideoTrackConstraints> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<DisplayVideoTrackConstraints>>
    for *mut wire_DisplayVideoTrackConstraints
{
    fn wire2api(self) -> Opaque<DisplayVideoTrackConstraints> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RefCell<MediaStreamSettings>>>
    for *mut wire_RefCellMediaStreamSettings
{
    fn wire2api(self) -> Opaque<RefCell<MediaStreamSettings>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_AudioTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DeviceVideoTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DisplayVideoTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellMediaStreamSettings {
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

impl NewWithNullPtr for wire_AudioTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_DeviceVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_DisplayVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RefCellMediaStreamSettings {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
