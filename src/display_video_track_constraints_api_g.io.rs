use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_new(
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_device_id(
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_device_id_impl(constraints, device_id)
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_exact_height(
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
    exact_height: u32,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_exact_height_impl(
        constraints,
        exact_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_ideal_height(
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
    ideal_height: u32,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_ideal_height_impl(
        constraints,
        ideal_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_exact_width(
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
    exact_width: u32,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_exact_width_impl(
        constraints,
        exact_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_ideal_width(
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
    ideal_width: u32,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_ideal_width_impl(
        constraints,
        ideal_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_ideal_frame_rate(
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
    ideal_frame_rate: u32,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_ideal_frame_rate_impl(
        constraints,
        ideal_frame_rate,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_exact_frame_rate(
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
    exact_frame_rate: u32,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_exact_frame_rate_impl(
        constraints,
        exact_frame_rate,
    )
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RefCellDisplayVideoTrackConstraints(
) -> *mut wire_RefCellDisplayVideoTrackConstraints {
    support::new_leak_box_ptr(
        wire_RefCellDisplayVideoTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_4(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: deallocate functions

// Section: impl Wire2Api

impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
    for *mut wire_RefCellDisplayVideoTrackConstraints
{
    fn wire2api(self) -> Opaque<RefCell<DisplayVideoTrackConstraints>> {
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
pub struct wire_RefCellDisplayVideoTrackConstraints {
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

impl NewWithNullPtr for wire_RefCellDisplayVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
