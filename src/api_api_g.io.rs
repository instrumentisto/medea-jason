use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_audio_track_constraints_new(
) -> support::WireSyncReturnStruct {
    wire_audio_track_constraints_new_impl()
}

#[no_mangle]
pub extern "C" fn wire_audio_track_constraints_device_id(
    track: wire_AudioTrackConstraints,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_audio_track_constraints_device_id_impl(track, device_id)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_close(
    connection: wire_ConnectionHandle,
    f: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_close_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_remote_track_added(
    connection: wire_ConnectionHandle,
    f: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_remote_track_added_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_on_quality_score_update(
    connection: wire_ConnectionHandle,
    f: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_on_quality_score_update_impl(connection, f)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_get_remote_member_id(
    connection: wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_get_remote_member_id_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_audio(
    connection: wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_enable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_audio(
    connection: wire_ConnectionHandle,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_disable_remote_audio_impl(connection)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_enable_remote_video(
    connection: wire_ConnectionHandle,
    source_kind: *mut i64,
) -> support::WireSyncReturnStruct {
    wire_connection_handle_enable_remote_video_impl(connection, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_connection_handle_disable_remote_video(
    connection: wire_ConnectionHandle,
    source_kind: *mut i64,
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
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_device_id_impl(constraints, device_id)
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_exact_facing_mode(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    facing_mode: i64,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_exact_facing_mode_impl(
        constraints,
        facing_mode,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_ideal_facing_mode(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    facing_mode: i64,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_ideal_facing_mode_impl(
        constraints,
        facing_mode,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_exact_height(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    exact_height: i64,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_exact_height_impl(
        constraints,
        exact_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_ideal_height(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    ideal_height: i64,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_ideal_height_impl(
        constraints,
        ideal_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_exact_width(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    exact_width: i64,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_exact_width_impl(
        constraints,
        exact_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_ideal_width(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    ideal_width: i64,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_ideal_width_impl(
        constraints,
        ideal_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_height_in_range(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    min: i64,
    max: i64,
) -> support::WireSyncReturnStruct {
    wire_device_video_track_constraints_height_in_range_impl(
        constraints,
        min,
        max,
    )
}

#[no_mangle]
pub extern "C" fn wire_device_video_track_constraints_width_in_range(
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
    min: i64,
    max: i64,
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
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_device_id_impl(constraints, device_id)
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_exact_height(
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
    exact_height: i64,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_exact_height_impl(
        constraints,
        exact_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_ideal_height(
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
    ideal_height: i64,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_ideal_height_impl(
        constraints,
        ideal_height,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_exact_width(
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
    exact_width: i64,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_exact_width_impl(
        constraints,
        exact_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_ideal_width(
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
    ideal_width: i64,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_ideal_width_impl(
        constraints,
        ideal_width,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_ideal_frame_rate(
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
    ideal_frame_rate: i64,
) -> support::WireSyncReturnStruct {
    wire_display_video_track_constraints_ideal_frame_rate_impl(
        constraints,
        ideal_frame_rate,
    )
}

#[no_mangle]
pub extern "C" fn wire_display_video_track_constraints_exact_frame_rate(
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
    exact_frame_rate: i64,
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
    jason: wire_Jason,
) -> support::WireSyncReturnStruct {
    wire_jason_init_room_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_media_manager(
    jason: wire_Jason,
) -> support::WireSyncReturnStruct {
    wire_jason_media_manager_impl(jason)
}

#[no_mangle]
pub extern "C" fn wire_jason_close_room(
    jason: wire_Jason,
    room_to_delete: wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_jason_close_room_impl(jason, room_to_delete)
}

#[no_mangle]
pub extern "C" fn wire_jason_dispose(
    jason: wire_Jason,
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
pub extern "C" fn wire_vec_local_tracks_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_vec_local_tracks_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_local_tracks_pop(
    vec: wire_ApiWrapVecLocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_vec_local_tracks_pop_impl(vec)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_get_track(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_kind(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_local_media_track_media_source_kind(
    track: wire_LocalMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_local_media_track_media_source_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_device_info_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_vec_media_device_info_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_device_info_pop(
    vec: wire_ApiWrapVecMediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_vec_media_device_info_pop_impl(vec)
}

#[no_mangle]
pub extern "C" fn wire_media_device_info_device_id(
    media_device: wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_device_id_impl(media_device)
}

#[no_mangle]
pub extern "C" fn wire_media_device_info_kind(
    media_device: wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_kind_impl(media_device)
}

#[no_mangle]
pub extern "C" fn wire_media_device_info_label(
    media_device: wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_label_impl(media_device)
}

#[no_mangle]
pub extern "C" fn wire_media_device_info_group_id(
    media_device: wire_MediaDeviceInfo,
) -> support::WireSyncReturnStruct {
    wire_media_device_info_group_id_impl(media_device)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_display_info_from_ptr(
    ptr: usize,
) -> support::WireSyncReturnStruct {
    wire_vec_media_display_info_from_ptr_impl(ptr)
}

#[no_mangle]
pub extern "C" fn wire_vec_media_display_info_pop(
    vec: wire_ApiWrapVecMediaDisplayInfo,
) -> support::WireSyncReturnStruct {
    wire_vec_media_display_info_pop_impl(vec)
}

#[no_mangle]
pub extern "C" fn wire_media_display_info_device_id(
    media_display: wire_MediaDisplayInfo,
) -> support::WireSyncReturnStruct {
    wire_media_display_info_device_id_impl(media_display)
}

#[no_mangle]
pub extern "C" fn wire_media_display_info_title(
    media_display: wire_MediaDisplayInfo,
) -> support::WireSyncReturnStruct {
    wire_media_display_info_title_impl(media_display)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_init_local_tracks(
    manager: wire_MediaManagerHandle,
    caps: wire_MediaStreamSettings,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_init_local_tracks_impl(manager, caps)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_enumerate_devices(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_enumerate_devices_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_enumerate_displays(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_enumerate_displays_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_set_output_audio_id(
    manager: wire_MediaManagerHandle,
    device_id: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_set_output_audio_id_impl(manager, device_id)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_set_microphone_volume(
    manager: wire_MediaManagerHandle,
    level: i64,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_set_microphone_volume_impl(manager, level)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_microphone_volume_is_available(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_microphone_volume_is_available_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_microphone_volume(
    manager: wire_MediaManagerHandle,
) -> support::WireSyncReturnStruct {
    wire_media_manager_handle_microphone_volume_impl(manager)
}

#[no_mangle]
pub extern "C" fn wire_media_manager_handle_on_device_change(
    manager: wire_MediaManagerHandle,
    cb: wire_DartOpaque,
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
    media_stream_settings: wire_MediaStreamSettings,
    constraints: wire_AudioTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_audio_impl(media_stream_settings, constraints)
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_device_video(
    media_stream_settings: wire_MediaStreamSettings,
    constraints: wire_ApiWrapDeviceVideoTrackConstraints,
) -> support::WireSyncReturnStruct {
    wire_media_stream_settings_device_video_impl(
        media_stream_settings,
        constraints,
    )
}

#[no_mangle]
pub extern "C" fn wire_media_stream_settings_display_video(
    media_stream_settings: wire_MediaStreamSettings,
    constraints: wire_ApiWrapDisplayVideoTrackConstraints,
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
    reconnect_handle: wire_ReconnectHandle,
    delay_ms: i64,
) -> support::WireSyncReturnStruct {
    wire_reconnect_handle_reconnect_with_delay_impl(reconnect_handle, delay_ms)
}

#[no_mangle]
pub extern "C" fn wire_reconnect_handle_reconnect_with_backoff(
    reconnect_handle: wire_ReconnectHandle,
    starting_delay: i64,
    multiplier: f64,
    max_delay: i64,
    max_elapsed_time_ms: *mut i64,
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
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_get_track_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_muted(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_muted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_unmuted(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_unmuted_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_stopped(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_stopped_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_on_media_direction_changed(
    track: wire_RemoteMediaTrack,
    f: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_on_media_direction_changed_impl(track, f)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_muted(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_muted_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_kind(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_source_kind(
    track: wire_RemoteMediaTrack,
) -> support::WireSyncReturnStruct {
    wire_remote_media_track_media_source_kind_impl(track)
}

#[no_mangle]
pub extern "C" fn wire_remote_media_track_media_direction(
    track: wire_RemoteMediaTrack,
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
    room_close_reason: wire_RoomCloseReason,
) -> support::WireSyncReturnStruct {
    wire_room_close_reason_reason_impl(room_close_reason)
}

#[no_mangle]
pub extern "C" fn wire_room_close_reason_is_closed_by_server(
    room_close_reason: wire_RoomCloseReason,
) -> support::WireSyncReturnStruct {
    wire_room_close_reason_is_closed_by_server_impl(room_close_reason)
}

#[no_mangle]
pub extern "C" fn wire_room_close_reason_is_err(
    room_close_reason: wire_RoomCloseReason,
) -> support::WireSyncReturnStruct {
    wire_room_close_reason_is_err_impl(room_close_reason)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_join(
    room_handle: wire_RoomHandle,
    token: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_room_handle_join_impl(room_handle, token)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_set_local_media_settings(
    room_handle: wire_RoomHandle,
    settings: wire_MediaStreamSettings,
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
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_mute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_unmute_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_mute_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i64,
) -> support::WireSyncReturnStruct {
    wire_room_handle_mute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_unmute_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i64,
) -> support::WireSyncReturnStruct {
    wire_room_handle_unmute_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i64,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i64,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_audio(
    room_handle: wire_RoomHandle,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_remote_audio_impl(room_handle)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_enable_remote_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i64,
) -> support::WireSyncReturnStruct {
    wire_room_handle_enable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_disable_remote_video(
    room_handle: wire_RoomHandle,
    source_kind: *mut i64,
) -> support::WireSyncReturnStruct {
    wire_room_handle_disable_remote_video_impl(room_handle, source_kind)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_new_connection(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_new_connection_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_close(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_close_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_local_track(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_local_track_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_connection_loss(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_connection_loss_impl(room_handle, cb)
}

#[no_mangle]
pub extern "C" fn wire_room_handle_on_failed_local_media(
    room_handle: wire_RoomHandle,
    cb: wire_DartOpaque,
) -> support::WireSyncReturnStruct {
    wire_room_handle_on_failed_local_media_impl(room_handle, cb)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_ApiWrapDeviceVideoTrackConstraints(
) -> wire_ApiWrapDeviceVideoTrackConstraints {
    wire_ApiWrapDeviceVideoTrackConstraints::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_ApiWrapDisplayVideoTrackConstraints(
) -> wire_ApiWrapDisplayVideoTrackConstraints {
    wire_ApiWrapDisplayVideoTrackConstraints::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_ApiWrapVecLocalMediaTrack(
) -> wire_ApiWrapVecLocalMediaTrack {
    wire_ApiWrapVecLocalMediaTrack::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_ApiWrapVecMediaDeviceInfo(
) -> wire_ApiWrapVecMediaDeviceInfo {
    wire_ApiWrapVecMediaDeviceInfo::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_ApiWrapVecMediaDisplayInfo(
) -> wire_ApiWrapVecMediaDisplayInfo {
    wire_ApiWrapVecMediaDisplayInfo::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_AudioTrackConstraints() -> wire_AudioTrackConstraints {
    wire_AudioTrackConstraints::new_with_null_ptr()
}

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
pub extern "C" fn new_MediaDeviceInfo() -> wire_MediaDeviceInfo {
    wire_MediaDeviceInfo::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MediaDisplayInfo() -> wire_MediaDisplayInfo {
    wire_MediaDisplayInfo::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MediaManagerHandle() -> wire_MediaManagerHandle {
    wire_MediaManagerHandle::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MediaStreamSettings() -> wire_MediaStreamSettings {
    wire_MediaStreamSettings::new_with_null_ptr()
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
pub extern "C" fn new_RoomCloseReason() -> wire_RoomCloseReason {
    wire_RoomCloseReason::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RoomHandle() -> wire_RoomHandle {
    wire_RoomHandle::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i64_0(value: i64) -> *mut i64 {
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
pub extern "C" fn drop_opaque_ApiWrapDeviceVideoTrackConstraints(
    ptr: *const c_void,
) {
    unsafe {
        Arc::<ApiWrap<DeviceVideoTrackConstraints>>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ApiWrapDeviceVideoTrackConstraints(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<ApiWrap<DeviceVideoTrackConstraints>>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_ApiWrapDisplayVideoTrackConstraints(
    ptr: *const c_void,
) {
    unsafe {
        Arc::<ApiWrap<DisplayVideoTrackConstraints>>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ApiWrapDisplayVideoTrackConstraints(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<ApiWrap<DisplayVideoTrackConstraints>>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_ApiWrapVecLocalMediaTrack(ptr: *const c_void) {
    unsafe {
        Arc::<ApiWrap<Vec<LocalMediaTrack>>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ApiWrapVecLocalMediaTrack(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<ApiWrap<Vec<LocalMediaTrack>>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_ApiWrapVecMediaDeviceInfo(ptr: *const c_void) {
    unsafe {
        Arc::<ApiWrap<Vec<MediaDeviceInfo>>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ApiWrapVecMediaDeviceInfo(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<ApiWrap<Vec<MediaDeviceInfo>>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_ApiWrapVecMediaDisplayInfo(ptr: *const c_void) {
    unsafe {
        Arc::<ApiWrap<Vec<MediaDisplayInfo>>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ApiWrapVecMediaDisplayInfo(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<ApiWrap<Vec<MediaDisplayInfo>>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_AudioTrackConstraints(ptr: *const c_void) {
    unsafe {
        Arc::<AudioTrackConstraints>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_AudioTrackConstraints(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<AudioTrackConstraints>::increment_strong_count(ptr as _);
        ptr
    }
}

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
pub extern "C" fn drop_opaque_MediaDeviceInfo(ptr: *const c_void) {
    unsafe {
        Arc::<MediaDeviceInfo>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MediaDeviceInfo(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<MediaDeviceInfo>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MediaDisplayInfo(ptr: *const c_void) {
    unsafe {
        Arc::<MediaDisplayInfo>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MediaDisplayInfo(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<MediaDisplayInfo>::increment_strong_count(ptr as _);
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
pub extern "C" fn drop_opaque_MediaStreamSettings(ptr: *const c_void) {
    unsafe {
        Arc::<MediaStreamSettings>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MediaStreamSettings(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<MediaStreamSettings>::increment_strong_count(ptr as _);
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
pub extern "C" fn drop_opaque_RoomCloseReason(ptr: *const c_void) {
    unsafe {
        Arc::<RoomCloseReason>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RoomCloseReason(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<RoomCloseReason>::increment_strong_count(ptr as _);
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

impl Wire2Api<RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>>
    for wire_ApiWrapDeviceVideoTrackConstraints
{
    fn wire2api(self) -> RustOpaque<ApiWrap<DeviceVideoTrackConstraints>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>>
    for wire_ApiWrapDisplayVideoTrackConstraints
{
    fn wire2api(self) -> RustOpaque<ApiWrap<DisplayVideoTrackConstraints>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<ApiWrap<Vec<LocalMediaTrack>>>>
    for wire_ApiWrapVecLocalMediaTrack
{
    fn wire2api(self) -> RustOpaque<ApiWrap<Vec<LocalMediaTrack>>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<ApiWrap<Vec<MediaDeviceInfo>>>>
    for wire_ApiWrapVecMediaDeviceInfo
{
    fn wire2api(self) -> RustOpaque<ApiWrap<Vec<MediaDeviceInfo>>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<ApiWrap<Vec<MediaDisplayInfo>>>>
    for wire_ApiWrapVecMediaDisplayInfo
{
    fn wire2api(self) -> RustOpaque<ApiWrap<Vec<MediaDisplayInfo>>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<AudioTrackConstraints>>
    for wire_AudioTrackConstraints
{
    fn wire2api(self) -> RustOpaque<AudioTrackConstraints> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
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
impl Wire2Api<RustOpaque<MediaDeviceInfo>> for wire_MediaDeviceInfo {
    fn wire2api(self) -> RustOpaque<MediaDeviceInfo> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<MediaDisplayInfo>> for wire_MediaDisplayInfo {
    fn wire2api(self) -> RustOpaque<MediaDisplayInfo> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<MediaManagerHandle>> for wire_MediaManagerHandle {
    fn wire2api(self) -> RustOpaque<MediaManagerHandle> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<MediaStreamSettings>> for wire_MediaStreamSettings {
    fn wire2api(self) -> RustOpaque<MediaStreamSettings> {
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
impl Wire2Api<RustOpaque<RoomCloseReason>> for wire_RoomCloseReason {
    fn wire2api(self) -> RustOpaque<RoomCloseReason> {
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
pub struct wire_ApiWrapDeviceVideoTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiWrapDisplayVideoTrackConstraints {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiWrapVecLocalMediaTrack {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiWrapVecMediaDeviceInfo {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApiWrapVecMediaDisplayInfo {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AudioTrackConstraints {
    ptr: *const core::ffi::c_void,
}

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
pub struct wire_MediaStreamSettings {
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

impl NewWithNullPtr for wire_ApiWrapDeviceVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_ApiWrapDisplayVideoTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_ApiWrapVecLocalMediaTrack {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_ApiWrapVecMediaDeviceInfo {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_ApiWrapVecMediaDisplayInfo {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_AudioTrackConstraints {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
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
impl NewWithNullPtr for wire_MediaStreamSettings {
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
