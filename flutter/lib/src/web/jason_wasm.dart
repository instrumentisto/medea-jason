// ignore_for_file: avoid_web_libraries_in_flutter, non_constant_identifier_names

import 'dart:js_interop';

import 'package:web/web.dart' as web;

@JS()
extension type FacingMode._(JSObject _) implements JSObject {
  external static num get User;
  external static num get Environment;
  external static num get Left;
  external static num get Right;
}

@JS()
extension type MediaSourceKind._(JSObject _) implements JSObject {
  external static num get Device;
  external static num get Display;
}

@JS()
extension type MediaKind._(JSObject _) implements JSObject {
  external static num get Audio;
  external static num get Video;
}

@JS()
extension type LocalMediaInitExceptionKind._(JSObject _) implements JSObject {
  external static num get GetUserMediaFailed;
  external static num get GetDisplayMediaFailed;
  external static num get LocalTrackIsEnded;
}

@JS()
extension type RpcClientExceptionKind._(JSObject _) implements JSObject {
  external static num get ConnectionLost;
  external static num get AuthorizationFailed;
  external static num get SessionFinished;
}

@JS()
extension type MemberConnectionState._(JSObject _) implements JSObject {
  external num kind();
  external JSAny value();
  external String test();
}

@JS()
extension type AudioTrackConstraints._(JSObject _) implements JSObject {
  external void free();
  external factory AudioTrackConstraints();
  external void device_id(String device_id);
  external void exact_auto_gain_control(bool auto_gain_control);
  external void ideal_auto_gain_control(bool auto_gain_control);
  external void ideal_echo_cancellation(bool echo_cancellation);
  external void exact_echo_cancellation(bool echo_cancellation);
  external void ideal_noise_suppression(bool noise_suppression);
  external void exact_noise_suppression(bool noise_suppression);
}

@JS()
extension type ConnectionHandle._(JSObject _) implements JSObject {
  external void free();
  external void on_close(JSFunction cb);
  external String get_remote_member_id();
  external MemberConnectionState? get_state();
  external void on_state_change(JSFunction cb);
  external void on_remote_track_added(JSFunction cb);
  external void on_quality_score_update(JSFunction cb);
  external JSPromise<JSAny?> disable_remote_audio();
  external JSPromise<JSAny?> disable_remote_video(num? source_kind);
  external JSPromise<JSAny?> enable_remote_audio();
  external JSPromise<JSAny?> enable_remote_video(num? source_kind);
}

@JS()
extension type DeviceVideoTrackConstraints._(JSObject _) implements JSObject {
  external void free();
  external factory DeviceVideoTrackConstraints();
  external void device_id(String device_id);
  external void exact_facing_mode(num facing_mode);
  external void ideal_facing_mode(num facing_mode);
  external void exact_height(num height);
  external void ideal_height(num height);
  external void height_in_range(num min, num max);
  external void exact_width(num width);
  external void ideal_width(num width);
  external void width_in_range(num min, num max);
  external void exact_frame_rate(num frame_rate);
  external void ideal_frame_rate(num frame_rate);
}

@JS()
extension type DisplayVideoTrackConstraints._(JSObject _) implements JSObject {
  external void free();
  external factory DisplayVideoTrackConstraints();
  external void exact_height(num height);
  external void ideal_height(num height);
  external void exact_width(num width);
  external void ideal_width(num width);
  external void exact_frame_rate(num frame_rate);
  external void ideal_frame_rate(num frame_rate);
}

@JS()
extension type EnumerateDevicesException._(JSObject _) implements JSObject {
  external void free();
  external JSObject cause();
  external String trace();
}

@JS()
extension type FormatException._(JSObject _) implements JSObject {
  external void free();
  external String message();
}

@JS()
extension type MediaDeviceDetails._(JSObject _) implements JSObject {
  external void free();
  external String device_id();
  external num kind();
  external String label();
  external String group_id();
}

@JS()
extension type InternalException._(JSObject _) implements JSObject {
  external void free();
  external String message();
  external JSObject cause();
  external String trace();
}

@JS()
extension type Jason._(JSObject _) implements JSObject {
  external void free();
  external factory Jason();
  external RoomHandle init_room();
  external MediaManagerHandle media_manager();
  external void close_room(RoomHandle room_to_delete);
  external void dispose();
}

@JS()
extension type LocalMediaInitException._(JSObject _) implements JSObject {
  external void free();
  external num kind();
  external String message();
  external JSObject cause();
  external String trace();
}

@JS()
extension type LocalMediaTrack._(JSObject _) implements JSObject {
  external void free();
  external web.MediaStreamTrack get_track();
  external num kind();
  external num media_source_kind();
  external JSPromise<JSAny?> state();
  external void on_enabled(JSFunction cb);
  external bool is_on_audio_level_available();
  external void on_audio_level_changed(JSFunction cb);
  external bool is_audio_processing_available();
  external JSPromise<JSAny?> set_noise_suppression_enabled(bool enabled);
  external JSPromise<JSAny?> set_echo_cancellation_enabled(bool enabled);
  external JSPromise<JSAny?> set_auto_gain_control_enabled(bool enabled);
  external JSPromise<JSBoolean> is_noise_suppression_enabled();
  external JSPromise<JSBoolean> is_auto_gain_control_enabled();
  external JSPromise<JSBoolean> is_echo_cancellation_enabled();
}

@JS()
extension type MediaManagerHandle._(JSObject _) implements JSObject {
  external void on_device_change(JSFunction cb);
  external void free();
  external JSPromise<JSArray<JSAny>> enumerate_devices();
  external JSPromise<JSArray<JSAny>> init_local_tracks(
    MediaStreamSettings caps,
  );
}

@JS()
extension type MediaSettingsUpdateException._(JSObject _) implements JSObject {
  external void free();
  external String message();
  external JSObject cause();
  external bool rolled_back();
}

@JS()
extension type MediaStateTransitionException._(JSObject _) implements JSObject {
  external void free();
  external String message();
  external String trace();
  external num kind();
}

@JS()
extension type MediaStreamSettings._(JSObject _) implements JSObject {
  external void free();
  external factory MediaStreamSettings();
  external void audio(AudioTrackConstraints constraints);
  external void device_video(DeviceVideoTrackConstraints constraints);
  external void display_video(DisplayVideoTrackConstraints constraints);
}

@JS('ReconnectHandle')
extension type ReconnectHandle._(JSObject _) implements JSObject {
  external void free();
  external JSPromise<JSAny?> reconnect_with_delay(num delay_ms);
  external JSPromise<JSAny?> reconnect_with_backoff(
    num starting_delay_ms,
    num multiplier,
    num max_delay,
    num? max_elapsed_time_ms,
  );
}

@JS()
extension type RemoteMediaTrack._(JSObject _) implements JSObject {
  external void free();
  external web.MediaStreamTrack get_track();
  external bool enabled();
  external bool muted();
  external void on_enabled(JSFunction cb);
  external void on_disabled(JSFunction cb);
  external void on_muted(JSFunction cb);
  external void on_unmuted(JSFunction cb);
  external void on_stopped(JSFunction cb);
  external void on_media_direction_changed(JSFunction cb);
  external num kind();
  external num media_source_kind();
  external num media_direction();
}

@JS()
extension type RoomCloseReason._(JSObject _) implements JSObject {
  external void free();
  external String reason();
  external bool is_closed_by_server();
  external bool is_err();
}

@JS()
extension type RoomHandle._(JSObject _) implements JSObject {
  external void free();
  external void on_new_connection(JSFunction cb);
  external void on_close(JSFunction cb);
  external void on_local_track(JSFunction cb);
  external void on_failed_local_media(JSFunction cb);
  external void on_connection_loss(JSFunction cb);
  external JSPromise<JSAny?> join(String token);
  external JSPromise<JSAny?> set_local_media_settings(
    MediaStreamSettings settings,
    bool stop_first,
    bool rollback_on_fail,
  );
  external JSPromise<JSAny?> mute_audio();
  external JSPromise<JSAny?> unmute_audio();
  external JSPromise<JSAny?> mute_video(num? source_kind);
  external JSPromise<JSAny?> unmute_video(num? source_kind);
  external JSPromise<JSAny?> disable_audio();
  external JSPromise<JSAny?> enable_audio();
  external JSPromise<JSAny?> disable_video(num? source_kind);
  external JSPromise<JSAny?> enable_video(num? source_kind);
  external JSPromise<JSAny?> disable_remote_audio();
  external JSPromise<JSAny?> disable_remote_video(num? source_kind);
  external JSPromise<JSAny?> enable_remote_audio();
  external JSPromise<JSAny?> enable_remote_video(num? source_kind);
}

@JS()
extension type RpcClientException._(JSObject _) implements JSObject {
  external void free();
  external num kind();
  external String message();
  external JSObject cause();
  external String trace();
}

@JS()
extension type StateError._(JSObject _) implements JSObject {
  external void free();
  external String message();
  external String trace();
}
