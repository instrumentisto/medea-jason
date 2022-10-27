use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_audio_track_constraints_new(
) -> support::WireSyncReturnStruct {
    wire_audio_track_constraints_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_audio_track_constraints_device_id(
    track: *mut wire_RefCellAudioTrackConstraints,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_audio_track_constraints_device_id_impl(track, device_id)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RefCellAudioTrackConstraints(
) -> *mut wire_RefCellAudioTrackConstraints {
    support::new_leak_box_ptr(
        wire_RefCellAudioTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_1(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<RefCell<AudioTrackConstraints>>>
    for *mut wire_RefCellAudioTrackConstraints
{
    fn wire2api(self) -> Opaque<RefCell<AudioTrackConstraints>> {
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
pub struct wire_RefCellAudioTrackConstraints {
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

impl NewWithNullPtr for wire_RefCellAudioTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
