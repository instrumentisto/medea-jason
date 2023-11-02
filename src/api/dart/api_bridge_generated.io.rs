use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_connection_handle_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_connection_handle_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_close(
    connection: wire_ConnectionHandle,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_connection_handle_on_close_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_remote_track_added(
    connection: wire_ConnectionHandle,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_connection_handle_on_remote_track_added_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_quality_score_update(
    connection: wire_ConnectionHandle,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_connection_handle_on_quality_score_update_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_get_remote_member_id(
    connection: wire_ConnectionHandle,
) -> support::WireSyncReturn {
    wire_connection_handle_get_remote_member_id_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_audio(
    connection: wire_ConnectionHandle,
) -> support::WireSyncReturn {
    wire_connection_handle_enable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_audio(
    connection: wire_ConnectionHandle,
) -> support::WireSyncReturn {
    wire_connection_handle_disable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_video(
    connection: wire_ConnectionHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_connection_handle_enable_remote_video_impl(connection, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_video(
    connection: wire_ConnectionHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_connection_handle_disable_remote_video_impl(connection, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_on_panic(
    cb: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_on_panic_impl(cb)
}

#[no_mangle]
pub extern "C" fn wire_jason_new() -> support::WireSyncReturn {
    wire_jason_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_jason_init_room(
    jason: wire_Jason,
) -> support::WireSyncReturn {
    wire_jason_init_room_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_media_manager(
    jason: wire_Jason,
) -> support::WireSyncReturn {
    wire_jason_media_manager_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_close_room(
    jason: wire_Jason,
    room_to_delete: wire_RoomHandle,
) -> support::WireSyncReturn {
    wire_jason_close_room_impl(jason, room_to_delete)
}

#[no_mangle]
pub extern "C" fn wire_jason_dispose(
    jason: wire_Jason,
) -> support::WireSyncReturn {
    wire_jason_dispose_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_local_media_track_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_local_tracks_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_vec_local_tracks_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_get_track(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturn {
    wire_local_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_kind(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturn {
    wire_local_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_on_ended(
    track: wire_LocalMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_local_media_track_on_ended_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_state(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturn {
    wire_local_media_track_state_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_media_source_kind(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturn {
    wire_local_media_track_media_source_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_free(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturn {
    wire_local_media_track_free_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_device_details_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_vec_media_device_details_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_display_details_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_vec_media_display_details_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_init_local_tracks(
    manager: wire_MediaManagerHandle,
    caps: *mut wire_ApiMediaStreamSettings,
) -> support::WireSyncReturn {
    wire_media_manager_handle_init_local_tracks_impl(manager, caps)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_enumerate_devices(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturn {
    wire_media_manager_handle_enumerate_devices_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_enumerate_displays(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturn {
    wire_media_manager_handle_enumerate_displays_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_set_output_audio_id(
    manager: wire_MediaManagerHandle,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_media_manager_handle_set_output_audio_id_impl(manager, device_id)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_set_microphone_volume(
    manager: wire_MediaManagerHandle,
    level: i64,
) -> support::WireSyncReturn {
    wire_media_manager_handle_set_microphone_volume_impl(manager, level)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_microphone_volume_is_available(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturn {
    wire_media_manager_handle_microphone_volume_is_available_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_microphone_volume(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturn {
    wire_media_manager_handle_microphone_volume_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_on_device_change(
    manager: wire_MediaManagerHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_media_manager_handle_on_device_change_impl(manager, cb)
}

#[no_mangle]
pub extern "C" fn wire_reconnect_handle_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_reconnect_handle_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_reconnect_handle_reconnect_with_delay(
    reconnect_handle: wire_ReconnectHandle,
    delay_ms: u32,
) -> support::WireSyncReturn {
    wire_reconnect_handle_reconnect_with_delay_impl(reconnect_handle, delay_ms)
}

#[no_mangle]
pub extern "C" fn wire_reconnect_handle_reconnect_with_backoff(
    reconnect_handle: wire_ReconnectHandle,
    starting_delay: u32,
    multiplier: f64,
    max_delay: u32,
    max_elapsed_time_ms: *mut u32,
) -> support::WireSyncReturn {
    wire_reconnect_handle_reconnect_with_backoff_impl(
        reconnect_handle,
        starting_delay,
        multiplier,
        max_delay,
        max_elapsed_time_ms,
    )
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_remote_media_track_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_get_track(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturn {
    wire_remote_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_muted(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_remote_media_track_on_muted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_unmuted(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_remote_media_track_on_unmuted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_stopped(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_remote_media_track_on_stopped_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_media_direction_changed(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_remote_media_track_on_media_direction_changed_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_muted(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturn {
    wire_remote_media_track_muted_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_kind(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturn {
    wire_remote_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_source_kind(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturn {
    wire_remote_media_track_media_source_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_direction(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturn {
    wire_remote_media_track_media_direction_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_room_close_reason_from_ptr(
    ptr: usize,
) -> support::WireSyncReturn {
    wire_room_close_reason_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_join(
    room_handle: wire_RoomHandle,
    token: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_room_handle_join_impl(room_handle, token)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_set_local_media_settings(
    room_handle: wire_RoomHandle,
    settings: *mut wire_ApiMediaStreamSettings,
    stop_first: bool,
    rollback_on_fail: bool,
) -> support::WireSyncReturn {
    wire_room_handle_set_local_media_settings_impl(
        room_handle,
        settings,
        stop_first,
        rollback_on_fail,
    )
}

#[no_mangle]
pub extern "C" fn wire_room_handle_mute_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturn {
    wire_room_handle_mute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturn {
    wire_room_handle_unmute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturn {
    wire_room_handle_enable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturn {
    wire_room_handle_disable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_mute_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_room_handle_mute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_room_handle_unmute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_room_handle_enable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_room_handle_disable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturn {
    wire_room_handle_enable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturn {
    wire_room_handle_disable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_room_handle_enable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i32,
) -> support::WireSyncReturn {
    wire_room_handle_disable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_new_connection(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_room_handle_on_new_connection_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_close(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_room_handle_on_close_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_local_track(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_room_handle_on_local_track_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_connection_loss(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_room_handle_on_connection_loss_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_failed_local_media(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturn {
    wire_room_handle_on_failed_local_media_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_log_dart_exception(
    message: *mut wire_uint_8_list,
    stack_trace: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_log_dart_exception_impl(message, stack_trace)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_ConnectionHandle() -> wire_ConnectionHandle {
    wire_ConnectionHandle::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_DartOpaque() -> wire_DartOpaque {
    wire_DartOpaque::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_Jason() -> wire_Jason {
    wire_Jason::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_LocalMediaTrack() -> wire_LocalMediaTrack {
    wire_LocalMediaTrack::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MediaManagerHandle() -> wire_MediaManagerHandle {
    wire_MediaManagerHandle::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_ReconnectHandle() -> wire_ReconnectHandle {
    wire_ReconnectHandle::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RemoteMediaTrack() -> wire_RemoteMediaTrack {
    wire_RemoteMediaTrack::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RoomHandle() -> wire_RoomHandle {
    wire_RoomHandle::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_api_audio_constraints_0(
) -> *mut wire_ApiAudioConstraints {
    support::new_leak_box_ptr(wire_ApiAudioConstraints::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_api_constrain_facing_mode_0(
) -> *mut wire_ApiConstrainFacingMode {
    support::new_leak_box_ptr(wire_ApiConstrainFacingMode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_api_device_video_track_constraints_0(
) -> *mut wire_ApiDeviceVideoTrackConstraints {
    support::new_leak_box_ptr(
        wire_ApiDeviceVideoTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_api_display_video_track_constraints_0(
) -> *mut wire_ApiDisplayVideoTrackConstraints {
    support::new_leak_box_ptr(
        wire_ApiDisplayVideoTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_api_media_stream_settings_0(
) -> *mut wire_ApiMediaStreamSettings {
    support::new_leak_box_ptr(wire_ApiMediaStreamSettings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_constrain_u_32_0() -> *mut wire_ConstrainU32 {
    support::new_leak_box_ptr(wire_ConstrainU32::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_media_source_kind_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_ConnectionHandle(ptr: *const c_void) {
    unsafe {
        Arc::<ConnectionHandle>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ConnectionHandle(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<ConnectionHandle>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_Jason(ptr: *const c_void) {
    unsafe {
        Arc::<Jason>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_Jason(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Jason>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_LocalMediaTrack(ptr: *const c_void) {
    unsafe {
        Arc::<LocalMediaTrack>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_LocalMediaTrack(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<LocalMediaTrack>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MediaManagerHandle(ptr: *const c_void) {
    unsafe {
        Arc::<MediaManagerHandle>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MediaManagerHandle(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<MediaManagerHandle>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_ReconnectHandle(ptr: *const c_void) {
    unsafe {
        Arc::<ReconnectHandle>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ReconnectHandle(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<ReconnectHandle>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RemoteMediaTrack(ptr: *const c_void) {
    unsafe {
        Arc::<RemoteMediaTrack>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RemoteMediaTrack(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<RemoteMediaTrack>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RoomHandle(ptr: *const c_void) {
    unsafe {
        Arc::<RoomHandle>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RoomHandle(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RoomHandle>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<ConnectionHandle>> for wire_ConnectionHandle {
    fn wire2api(self) -> RustOpaque<ConnectionHandle> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<DartOpaque> for wire_DartOpaque {
    fn wire2api(self) -> DartOpaque {
        unsafe { DartOpaque::new(self.handle as _, self.port) }
    }
}
impl Wire2Api<RustOpaque<Jason>> for wire_Jason {
    fn wire2api(self) -> RustOpaque<Jason> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<LocalMediaTrack>> for wire_LocalMediaTrack {
    fn wire2api(self) -> RustOpaque<LocalMediaTrack> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<MediaManagerHandle>> for wire_MediaManagerHandle {
    fn wire2api(self) -> RustOpaque<MediaManagerHandle> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<ReconnectHandle>> for wire_ReconnectHandle {
    fn wire2api(self) -> RustOpaque<ReconnectHandle> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<RemoteMediaTrack>> for wire_RemoteMediaTrack {
    fn wire2api(self) -> RustOpaque<RemoteMediaTrack> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<RoomHandle>> for wire_RoomHandle {
    fn wire2api(self) -> RustOpaque<RoomHandle> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<ApiAudioConstraints> for wire_ApiAudioConstraints {
    fn wire2api(self) -> ApiAudioConstraints {
        ApiAudioConstraints {
            device_id: self.device_id.wire2api(),
        }
    }
}
impl Wire2Api<ApiConstrainFacingMode> for wire_ApiConstrainFacingMode {
    fn wire2api(self) -> ApiConstrainFacingMode {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Exact);
                ApiConstrainFacingMode::Exact(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Ideal);
                ApiConstrainFacingMode::Ideal(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ApiDeviceVideoTrackConstraints>
    for wire_ApiDeviceVideoTrackConstraints
{
    fn wire2api(self) -> ApiDeviceVideoTrackConstraints {
        ApiDeviceVideoTrackConstraints {
            device_id: self.device_id.wire2api(),
            facing_mode: self.facing_mode.wire2api(),
            height: self.height.wire2api(),
            width: self.width.wire2api(),
        }
    }
}
impl Wire2Api<ApiDisplayVideoTrackConstraints>
    for wire_ApiDisplayVideoTrackConstraints
{
    fn wire2api(self) -> ApiDisplayVideoTrackConstraints {
        ApiDisplayVideoTrackConstraints {
            device_id: self.device_id.wire2api(),
            height: self.height.wire2api(),
            width: self.width.wire2api(),
            frame_rate: self.frame_rate.wire2api(),
        }
    }
}
impl Wire2Api<ApiMediaStreamSettings> for wire_ApiMediaStreamSettings {
    fn wire2api(self) -> ApiMediaStreamSettings {
        ApiMediaStreamSettings {
            audio: self.audio.wire2api(),
            device_video: self.device_video.wire2api(),
            display_video: self.display_video.wire2api(),
        }
    }
}

impl Wire2Api<ApiAudioConstraints> for *mut wire_ApiAudioConstraints {
    fn wire2api(self) -> ApiAudioConstraints {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApiAudioConstraints>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApiConstrainFacingMode> for *mut wire_ApiConstrainFacingMode {
    fn wire2api(self) -> ApiConstrainFacingMode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApiConstrainFacingMode>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApiDeviceVideoTrackConstraints>
    for *mut wire_ApiDeviceVideoTrackConstraints
{
    fn wire2api(self) -> ApiDeviceVideoTrackConstraints {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApiDeviceVideoTrackConstraints>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApiDisplayVideoTrackConstraints>
    for *mut wire_ApiDisplayVideoTrackConstraints
{
    fn wire2api(self) -> ApiDisplayVideoTrackConstraints {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApiDisplayVideoTrackConstraints>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApiMediaStreamSettings> for *mut wire_ApiMediaStreamSettings {
    fn wire2api(self) -> ApiMediaStreamSettings {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApiMediaStreamSettings>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ConstrainU32> for *mut wire_ConstrainU32 {
    fn wire2api(self) -> ConstrainU32 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ConstrainU32>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MediaSourceKind> for *mut i32 {
    fn wire2api(self) -> MediaSourceKind {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MediaSourceKind>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<ConstrainU32> for wire_ConstrainU32 {
    fn wire2api(self) -> ConstrainU32 {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Exact);
                ConstrainU32::Exact(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Ideal);
                ConstrainU32::Ideal(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Range);
                ConstrainU32::Range(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                )
            },
            _ => unreachable!(),
        }
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
pub struct wire_ConnectionHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DartOpaque {
    port: i64,
    handle: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Jason {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LocalMediaTrack {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MediaManagerHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ReconnectHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RemoteMediaTrack {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RoomHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiAudioConstraints {
    device_id: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiDeviceVideoTrackConstraints {
    device_id: *mut wire_uint_8_list,
    facing_mode: *mut wire_ApiConstrainFacingMode,
    height: *mut wire_ConstrainU32,
    width: *mut wire_ConstrainU32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiDisplayVideoTrackConstraints {
    device_id: *mut wire_uint_8_list,
    height: *mut wire_ConstrainU32,
    width: *mut wire_ConstrainU32,
    frame_rate: *mut wire_ConstrainU32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiMediaStreamSettings {
    audio: *mut wire_ApiAudioConstraints,
    device_video: *mut wire_ApiDeviceVideoTrackConstraints,
    display_video: *mut wire_ApiDisplayVideoTrackConstraints,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiConstrainFacingMode {
    tag: i32,
    kind: *mut ApiConstrainFacingModeKind,
}

#[repr(C)]
pub union ApiConstrainFacingModeKind {
    Exact: *mut wire_ApiConstrainFacingMode_Exact,
    Ideal: *mut wire_ApiConstrainFacingMode_Ideal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiConstrainFacingMode_Exact {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiConstrainFacingMode_Ideal {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConstrainU32 {
    tag: i32,
    kind: *mut ConstrainU32Kind,
}

#[repr(C)]
pub union ConstrainU32Kind {
    Exact: *mut wire_ConstrainU32_Exact,
    Ideal: *mut wire_ConstrainU32_Ideal,
    Range: *mut wire_ConstrainU32_Range,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConstrainU32_Exact {
    field0: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConstrainU32_Ideal {
    field0: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConstrainU32_Range {
    field0: u32,
    field1: u32,
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

impl NewWithNullPtr for wire_ConnectionHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_DartOpaque {
    fn new_with_null_ptr() -> Self {
        Self { port: 0, handle: 0 }
    }
}
impl NewWithNullPtr for wire_Jason {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_LocalMediaTrack {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MediaManagerHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_ReconnectHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RemoteMediaTrack {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RoomHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_ApiAudioConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            device_id: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_ApiAudioConstraints {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl Default for wire_ApiConstrainFacingMode {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ApiConstrainFacingMode {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_ApiConstrainFacingMode_Exact(
) -> *mut ApiConstrainFacingModeKind {
    support::new_leak_box_ptr(ApiConstrainFacingModeKind {
        Exact: support::new_leak_box_ptr(wire_ApiConstrainFacingMode_Exact {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_ApiConstrainFacingMode_Ideal(
) -> *mut ApiConstrainFacingModeKind {
    support::new_leak_box_ptr(ApiConstrainFacingModeKind {
        Ideal: support::new_leak_box_ptr(wire_ApiConstrainFacingMode_Ideal {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_ApiDeviceVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            device_id: core::ptr::null_mut(),
            facing_mode: core::ptr::null_mut(),
            height: core::ptr::null_mut(),
            width: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_ApiDeviceVideoTrackConstraints {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ApiDisplayVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            device_id: core::ptr::null_mut(),
            height: core::ptr::null_mut(),
            width: core::ptr::null_mut(),
            frame_rate: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_ApiDisplayVideoTrackConstraints {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ApiMediaStreamSettings {
    fn new_with_null_ptr() -> Self {
        Self {
            audio: core::ptr::null_mut(),
            device_video: core::ptr::null_mut(),
            display_video: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_ApiMediaStreamSettings {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl Default for wire_ConstrainU32 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ConstrainU32 {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_ConstrainU32_Exact() -> *mut ConstrainU32Kind {
    support::new_leak_box_ptr(ConstrainU32Kind {
        Exact: support::new_leak_box_ptr(wire_ConstrainU32_Exact {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_ConstrainU32_Ideal() -> *mut ConstrainU32Kind {
    support::new_leak_box_ptr(ConstrainU32Kind {
        Ideal: support::new_leak_box_ptr(wire_ConstrainU32_Ideal {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_ConstrainU32_Range() -> *mut ConstrainU32Kind {
    support::new_leak_box_ptr(ConstrainU32Kind {
        Range: support::new_leak_box_ptr(wire_ConstrainU32_Range {
            field0: Default::default(),
            field1: Default::default(),
        }),
    })
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
