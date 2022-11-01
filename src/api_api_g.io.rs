use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_dart_future_to_usize(
    handle: *mut wire_MyDartFuture,
) -> support::WireSyncReturnStruct {
    wire_dart_future_to_usize_impl(handle)
}

#[no_mangle]
pub extern "C" fn wire_opaque_to_usize(
    handle: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_opaque_to_usize_impl(handle)
}

#[no_mangle]
pub extern "C" fn wire_dart_handle_to_opaque(
    handle: usize,
) -> support::WireSyncReturnStruct {
    wire_dart_handle_to_opaque_impl(handle)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_local_tracks_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_vec_local_tracks_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_local_tracks_pop(
    vec: *mut wire_RefCellVecLocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_vec_local_tracks_pop_impl(vec)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_display_info_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_vec_media_display_info_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_display_info_pop(
    vec: *mut wire_RefCellVecMediaDisplayInfo,
) -> support::WireSyncReturnStruct {
    wire_vec_media_display_info_pop_impl(vec)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_device_info_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_vec_media_device_info_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_device_info_pop(
    vec: *mut wire_RefCellVecMediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_vec_media_device_info_pop_impl(vec)
}

#[no_mangle]
pub extern "C" fn wire_audio_track_constraints_new(
) -> support::WireSyncReturnStruct {
    wire_audio_track_constraints_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_audio_track_constraints_cast(
    track: *mut wire_RefCellAudioTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_audio_track_constraints_cast_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_audio_track_constraints_device_id(
    track: *mut wire_RefCellAudioTrackConstraints,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_audio_track_constraints_device_id_impl(track, device_id)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_close(
    connection: *mut wire_ConnectionHandle,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_close_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_remote_track_added(
    connection: *mut wire_ConnectionHandle,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_remote_track_added_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_quality_score_update(
    connection: *mut wire_ConnectionHandle,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_quality_score_update_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_get_remote_member_id(
    connection: *mut wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_get_remote_member_id_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_audio(
    connection: *mut wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_enable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_audio(
    connection: *mut wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_disable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_video(
    connection: *mut wire_ConnectionHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_enable_remote_video_impl(connection, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_video(
    connection: *mut wire_ConnectionHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_disable_remote_video_impl(connection, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_new(
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_device_id(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_device_id_impl(constraints, device_id)
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_exact_facing_mode(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    facing_mode: u8,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_exact_facing_mode_impl(
        constraints,
        facing_mode,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_ideal_facing_mode(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    facing_mode: u8,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_ideal_facing_mode_impl(
        constraints,
        facing_mode,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_exact_height(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    exact_height: u32,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_exact_height_impl(
        constraints,
        exact_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_ideal_height(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    ideal_height: u32,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_ideal_height_impl(
        constraints,
        ideal_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_exact_width(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    exact_width: u32,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_exact_width_impl(
        constraints,
        exact_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_ideal_width(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    ideal_width: u32,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_ideal_width_impl(
        constraints,
        ideal_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_height_in_range(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    min: u32,
    max: u32,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_height_in_range_impl(
        constraints,
        min,
        max,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_width_in_range(
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
    min: u32,
    max: u32,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_width_in_range_impl(
        constraints,
        min,
        max,
    )
}

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

#[no_mangle]
pub extern "C" fn wire_jason_new() -> support::WireSyncReturnStruct {
    wire_jason_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_jason_init_room(
    jason: *mut wire_RefCellOptionJason,
) -> support::WireSyncReturnStruct {
    wire_jason_init_room_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_media_manager(
    jason: *mut wire_RefCellOptionJason,
) -> support::WireSyncReturnStruct {
    wire_jason_media_manager_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_close_room(
    jason: *mut wire_RefCellOptionJason,
    room_to_delete: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_jason_close_room_impl(jason, room_to_delete)
}

#[no_mangle]
pub extern "C" fn wire_jason_dispose(
    jason: *mut wire_RefCellOptionJason,
) -> support::WireSyncReturnStruct {
    wire_jason_dispose_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_get_track(
    track: *mut wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_kind(
    track: *mut wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_media_source_kind(
    track: *mut wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_media_source_kind_impl(track)
}

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

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_init_local_tracks(
    manager: *mut wire_MediaManagerHandle,
    caps: *mut wire_RefCellMediaStreamSettings,
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

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_on_device_change(
    manager: *mut wire_MediaManagerHandle,
    cb: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_on_device_change_impl(manager, cb)
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_new(
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_audio(
    media_stream_settings: *mut wire_RefCellMediaStreamSettings,
    constraints: *mut wire_RefCellAudioTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_audio_impl(media_stream_settings, constraints)
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_device_video(
    media_stream_settings: *mut wire_RefCellMediaStreamSettings,
    constraints: *mut wire_RefCellDeviceVideoTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_device_video_impl(
        media_stream_settings,
        constraints,
    )
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_display_video(
    media_stream_settings: *mut wire_RefCellMediaStreamSettings,
    constraints: *mut wire_RefCellDisplayVideoTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_display_video_impl(
        media_stream_settings,
        constraints,
    )
}

#[no_mangle]
pub extern "C" fn wire_reconnect_handle_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_reconnect_handle_from_ptr_impl(ptr)
}

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

#[no_mangle]
pub extern "C" fn wire_remote_media_track_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_get_track(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_muted(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_muted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_unmuted(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_unmuted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_stopped(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_stopped_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_media_direction_changed(
    track: *mut wire_RemoteMediaTrack,
    f: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_media_direction_changed_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_muted(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_muted_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_kind(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_source_kind(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_media_source_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_direction(
    track: *mut wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_media_direction_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_room_close_reason_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_room_close_reason_from_ptr_impl(ptr)
}

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

#[no_mangle]
pub extern "C" fn wire_room_handle_join(
    room_handle: *mut wire_RoomHandle,
    token: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_room_handle_join_impl(room_handle, token)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_set_local_media_settings(
    room_handle: *mut wire_RoomHandle,
    settings: *mut wire_RefCellMediaStreamSettings,
    stop_first: bool,
    rollback_on_fail: bool,
) -> support::WireSyncReturnStruct {
    wire_room_handle_set_local_media_settings_impl(
        room_handle,
        settings,
        stop_first,
        rollback_on_fail,
    )
}

#[no_mangle]
pub extern "C" fn wire_room_handle_mute_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_mute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_unmute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_mute_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_mute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_unmute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_audio(
    room_handle: *mut wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_video(
    room_handle: *mut wire_RoomHandle,
    source_kind: *mut u8,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_new_connection(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_new_connection_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_close(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_close_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_local_track(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_local_track_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_connection_loss(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_connection_loss_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_failed_local_media(
    room_handle: *mut wire_RoomHandle,
    cb: *mut wire_DartHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_failed_local_media_impl(room_handle, cb)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_ConnectionHandle() -> *mut wire_ConnectionHandle {
    support::new_leak_box_ptr(wire_ConnectionHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_DartHandle() -> *mut wire_DartHandle {
    support::new_leak_box_ptr(wire_DartHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_LocalMediaTrack() -> *mut wire_LocalMediaTrack {
    support::new_leak_box_ptr(wire_LocalMediaTrack::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_MediaDeviceInfo() -> *mut wire_MediaDeviceInfo {
    support::new_leak_box_ptr(wire_MediaDeviceInfo::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_MediaDisplayInfo() -> *mut wire_MediaDisplayInfo {
    support::new_leak_box_ptr(wire_MediaDisplayInfo::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_MediaManagerHandle() -> *mut wire_MediaManagerHandle {
    support::new_leak_box_ptr(wire_MediaManagerHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_MyDartFuture() -> *mut wire_MyDartFuture {
    support::new_leak_box_ptr(wire_MyDartFuture::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_ReconnectHandle() -> *mut wire_ReconnectHandle {
    support::new_leak_box_ptr(wire_ReconnectHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RefCellAudioTrackConstraints(
) -> *mut wire_RefCellAudioTrackConstraints {
    support::new_leak_box_ptr(
        wire_RefCellAudioTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RefCellDeviceVideoTrackConstraints(
) -> *mut wire_RefCellDeviceVideoTrackConstraints {
    support::new_leak_box_ptr(
        wire_RefCellDeviceVideoTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RefCellDisplayVideoTrackConstraints(
) -> *mut wire_RefCellDisplayVideoTrackConstraints {
    support::new_leak_box_ptr(
        wire_RefCellDisplayVideoTrackConstraints::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RefCellMediaStreamSettings(
) -> *mut wire_RefCellMediaStreamSettings {
    support::new_leak_box_ptr(
        wire_RefCellMediaStreamSettings::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RefCellOptionJason() -> *mut wire_RefCellOptionJason {
    support::new_leak_box_ptr(wire_RefCellOptionJason::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RefCellVecLocalMediaTrack(
) -> *mut wire_RefCellVecLocalMediaTrack {
    support::new_leak_box_ptr(
        wire_RefCellVecLocalMediaTrack::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RefCellVecMediaDeviceInfo(
) -> *mut wire_RefCellVecMediaDeviceInfo {
    support::new_leak_box_ptr(
        wire_RefCellVecMediaDeviceInfo::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RefCellVecMediaDisplayInfo(
) -> *mut wire_RefCellVecMediaDisplayInfo {
    support::new_leak_box_ptr(
        wire_RefCellVecMediaDisplayInfo::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_RemoteMediaTrack() -> *mut wire_RemoteMediaTrack {
    support::new_leak_box_ptr(wire_RemoteMediaTrack::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RoomCloseReason() -> *mut wire_RoomCloseReason {
    support::new_leak_box_ptr(wire_RoomCloseReason::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RoomHandle() -> *mut wire_RoomHandle {
    support::new_leak_box_ptr(wire_RoomHandle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u8_0(value: u8) -> *mut u8 {
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

// Section: deallocate functions

#[no_mangle]
pub extern "C" fn drop_box_autoadd_u32_0(raw: *mut u32) {
    unsafe {
        {
            drop(support::box_from_leak_ptr(raw));
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_u8_0(raw: *mut u8) {
    unsafe {
        {
            drop(support::box_from_leak_ptr(raw));
        }
    }
}

// Section: impl Wire2Api

impl Wire2Api<Opaque<ConnectionHandle>> for *mut wire_ConnectionHandle {
    fn wire2api(self) -> Opaque<ConnectionHandle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<Dart_Handle>> for *mut wire_DartHandle {
    fn wire2api(self) -> Opaque<Dart_Handle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<LocalMediaTrack>> for *mut wire_LocalMediaTrack {
    fn wire2api(self) -> Opaque<LocalMediaTrack> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<MediaDeviceInfo>> for *mut wire_MediaDeviceInfo {
    fn wire2api(self) -> Opaque<MediaDeviceInfo> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<MediaDisplayInfo>> for *mut wire_MediaDisplayInfo {
    fn wire2api(self) -> Opaque<MediaDisplayInfo> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<MediaManagerHandle>> for *mut wire_MediaManagerHandle {
    fn wire2api(self) -> Opaque<MediaManagerHandle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<MyDartFuture>> for *mut wire_MyDartFuture {
    fn wire2api(self) -> Opaque<MyDartFuture> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<ReconnectHandle>> for *mut wire_ReconnectHandle {
    fn wire2api(self) -> Opaque<ReconnectHandle> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
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
impl Wire2Api<Opaque<RefCell<DeviceVideoTrackConstraints>>>
    for *mut wire_RefCellDeviceVideoTrackConstraints
{
    fn wire2api(self) -> Opaque<RefCell<DeviceVideoTrackConstraints>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
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
impl Wire2Api<Opaque<RefCell<Option<Jason>>>> for *mut wire_RefCellOptionJason {
    fn wire2api(self) -> Opaque<RefCell<Option<Jason>>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RefCell<Vec<LocalMediaTrack>>>>
    for *mut wire_RefCellVecLocalMediaTrack
{
    fn wire2api(self) -> Opaque<RefCell<Vec<LocalMediaTrack>>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RefCell<Vec<MediaDeviceInfo>>>>
    for *mut wire_RefCellVecMediaDeviceInfo
{
    fn wire2api(self) -> Opaque<RefCell<Vec<MediaDeviceInfo>>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RefCell<Vec<MediaDisplayInfo>>>>
    for *mut wire_RefCellVecMediaDisplayInfo
{
    fn wire2api(self) -> Opaque<RefCell<Vec<MediaDisplayInfo>>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RemoteMediaTrack>> for *mut wire_RemoteMediaTrack {
    fn wire2api(self) -> Opaque<RemoteMediaTrack> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RoomCloseReason>> for *mut wire_RoomCloseReason {
    fn wire2api(self) -> Opaque<RoomCloseReason> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RoomHandle>> for *mut wire_RoomHandle {
    fn wire2api(self) -> Opaque<RoomHandle> {
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
pub struct wire_ConnectionHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DartHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LocalMediaTrack {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MediaDeviceInfo {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MediaDisplayInfo {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MediaManagerHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyDartFuture {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ReconnectHandle {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellAudioTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellDeviceVideoTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellDisplayVideoTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellMediaStreamSettings {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellOptionJason {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellVecLocalMediaTrack {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellVecMediaDeviceInfo {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RefCellVecMediaDisplayInfo {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RemoteMediaTrack {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RoomCloseReason {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RoomHandle {
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

impl NewWithNullPtr for wire_ConnectionHandle {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_DartHandle {
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
impl NewWithNullPtr for wire_MediaDeviceInfo {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MediaDisplayInfo {
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
impl NewWithNullPtr for wire_MyDartFuture {
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
impl NewWithNullPtr for wire_RefCellAudioTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RefCellDeviceVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RefCellDisplayVideoTrackConstraints {
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
impl NewWithNullPtr for wire_RefCellOptionJason {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RefCellVecLocalMediaTrack {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RefCellVecMediaDeviceInfo {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RefCellVecMediaDisplayInfo {
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
impl NewWithNullPtr for wire_RoomCloseReason {
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

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(
    val: support::WireSyncReturnStruct,
) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
