// ignore_for_file: avoid_web_libraries_in_flutter, non_constant_identifier_names

@JS()
library medea_jason;

import 'dart:js_interop';

import 'package:web/web.dart' as web;

@JS()
class FacingMode {
  external static num get User;
  external static num get Environment;
  external static num get Left;
  external static num get Right;
}

@JS()
class MediaSourceKind {
  external static num get Device;
  external static num get Display;
}

@JS()
class MediaKind {
  external static num get Audio;
  external static num get Video;
}

@JS()
class LocalMediaInitExceptionKind {
  external static num get GetUserMediaFailed;
  external static num get GetDisplayMediaFailed;
  external static num get LocalTrackIsEnded;
}

@JS()
class RpcClientExceptionKind {
  external static num get ConnectionLost;
  external static num get AuthorizationFailed;
  external static num get SessionFinished;
}

@JS()
class AudioTrackConstraints {
  external void free();
  external factory AudioTrackConstraints();
  external void device_id(String device_id);
  external void exact_auto_gain_control(bool auto_gain_control);
  external void ideal_auto_gain_control(bool auto_gain_control);
}

@JS()
class ConnectionHandle {
  external void free();
  external void on_close(JSFunction cb);
  external String get_remote_member_id();
  external void on_remote_track_added(JSFunction cb);
  external void on_quality_score_update(JSFunction cb);
}

@JS('ConnectionHandle')
abstract class _ConnectionHandle {
  external JSPromise<JSAny?> disable_remote_audio();
  external JSPromise<JSAny?> disable_remote_video(num? source_kind);
  external JSPromise<JSAny?> enable_remote_audio();
  external JSPromise<JSAny?> enable_remote_video(num? source_kind);
}

extension ConnectionHandleExtensions on ConnectionHandle {
  Future<dynamic> disable_remote_audio() {
    final tt = this as _ConnectionHandle;
    return tt.disable_remote_audio().toDart;
  }

  Future<dynamic> disable_remote_video(num? source_kind) {
    final tt = this as _ConnectionHandle;
    return tt.disable_remote_video(source_kind).toDart;
  }

  Future<dynamic> enable_remote_audio() {
    final tt = this as _ConnectionHandle;
    return tt.enable_remote_audio().toDart;
  }

  Future<dynamic> enable_remote_video(num? source_kind) {
    final tt = this as _ConnectionHandle;
    return tt.enable_remote_video(source_kind).toDart;
  }
}

@JS()
class DeviceVideoTrackConstraints {
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
class DisplayVideoTrackConstraints {
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
class EnumerateDevicesException {
  external void free();
  external Error cause();
  external String trace();
}

@JS()
class FormatException {
  external void free();
  external String message();
}

@JS()
class MediaDeviceDetails {
  external void free();
  external String device_id();
  external num kind();
  external String label();
  external String group_id();
}

@JS()
class InternalException {
  external void free();
  external String message();
  external dynamic cause();
  external String trace();
}

@JS()
class Jason {
  external void free();
  external factory Jason();
  external RoomHandle init_room();
  external MediaManagerHandle media_manager();
  external void close_room(RoomHandle room_to_delete);
  external void dispose();
}

@JS()
class LocalMediaInitException {
  external void free();
  external num kind();
  external String message();
  external dynamic cause();
  external String trace();
}

@JS()
class LocalMediaTrack {
  external void free();
  external web.MediaStreamTrack get_track();
  external num kind();
  external num media_source_kind();
  external void on_enabled(JSFunction cb);
}

@JS('LocalMediaTrack')
abstract class _LocalMediaTrack {
  external JSPromise<JSAny?> state();
}

extension LocalMediaTrackExtensions on LocalMediaTrack {
  Future<dynamic> state() {
    final tt = this as _LocalMediaTrack;
    return tt.state().toDart;
  }
}

@JS()
class MediaManagerHandle {
  external void on_device_change(JSFunction cb);
  external void free();
}

@JS('MediaManagerHandle')
abstract class _MediaManagerHandle {
  external JSPromise<JSArray<JSAny>> enumerate_devices();
  external JSPromise<JSArray<JSAny>> init_local_tracks(
      MediaStreamSettings caps);
}

extension MediaManagerHandleExtensions on MediaManagerHandle {
  Future<List<dynamic>> enumerate_devices() async {
    final tt = this as _MediaManagerHandle;
    return (await tt.enumerate_devices().toDart).toDart;
  }

  Future<List<dynamic>> init_local_tracks(MediaStreamSettings caps) async {
    final tt = this as _MediaManagerHandle;
    return (await tt.init_local_tracks(caps).toDart).toDart;
  }
}

@JS()
class MediaSettingsUpdateException {
  external void free();
  external String message();
  external dynamic cause();
  external bool rolled_back();
}

@JS()
class MediaStateTransitionException {
  external void free();
  external String message();
  external String trace();
  external num kind();
}

@JS()
class MediaStreamSettings {
  external void free();
  external factory MediaStreamSettings();
  external void audio(AudioTrackConstraints constraints);
  external void device_video(DeviceVideoTrackConstraints constraints);
  external void display_video(DisplayVideoTrackConstraints constraints);
}

@JS()
class ReconnectHandle {
  external void free();
}

@JS('ReconnectHandle')
abstract class _ReconnectHandle {
  external JSPromise<JSAny?> reconnect_with_delay(num delay_ms);
  external JSPromise<JSAny?> reconnect_with_backoff(
    num starting_delay_ms,
    num multiplier,
    num max_delay,
    num? max_elapsed_time_ms,
  );
}

extension ReconnectHandleExtensions on ReconnectHandle {
  Future<dynamic> reconnect_with_delay(num delay_ms) {
    final tt = this as _ReconnectHandle;
    return tt.reconnect_with_delay(delay_ms).toDart;
  }

  Future<dynamic> reconnect_with_backoff(
    num starting_delay_ms,
    num multiplier,
    num max_delay,
    num? max_elapsed_time_ms,
  ) {
    final tt = this as _ReconnectHandle;
    return tt
        .reconnect_with_backoff(
          starting_delay_ms,
          multiplier,
          max_delay,
          max_elapsed_time_ms,
        )
        .toDart;
  }
}

@JS()
class RemoteMediaTrack {
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
class RoomCloseReason {
  external void free();
  external String reason();
  external bool is_closed_by_server();
  external bool is_err();
}

@JS()
class RoomHandle {
  external void free();
  external void on_new_connection(JSFunction cb);
  external void on_close(JSFunction cb);
  external void on_local_track(JSFunction cb);
  external void on_failed_local_media(JSFunction cb);
  external void on_connection_loss(JSFunction cb);
}

@JS('RoomHandle')
abstract class _RoomHandle {
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

extension RoomHandleExtensions on RoomHandle {
  Future<dynamic> join(String token) {
    final tt = this as _RoomHandle;
    return tt.join(token).toDart;
  }

  Future<dynamic> set_local_media_settings(
    MediaStreamSettings settings,
    bool stop_first,
    bool rollback_on_fail,
  ) {
    final tt = this as _RoomHandle;
    return tt
        .set_local_media_settings(settings, stop_first, rollback_on_fail)
        .toDart;
  }

  Future<dynamic> mute_audio() {
    final tt = this as _RoomHandle;
    return tt.mute_audio().toDart;
  }

  Future<dynamic> unmute_audio() {
    final tt = this as _RoomHandle;
    return tt.unmute_audio().toDart;
  }

  Future<dynamic> mute_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return tt.mute_video(source_kind).toDart;
  }

  Future<dynamic> unmute_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return tt.unmute_video(source_kind).toDart;
  }

  Future<dynamic> disable_audio() {
    final tt = this as _RoomHandle;
    return tt.disable_audio().toDart;
  }

  Future<dynamic> enable_audio() {
    final tt = this as _RoomHandle;
    return tt.enable_audio().toDart;
  }

  Future<dynamic> disable_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return tt.disable_video(source_kind).toDart;
  }

  Future<dynamic> enable_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return tt.enable_video(source_kind).toDart;
  }

  Future<dynamic> disable_remote_audio() {
    final tt = this as _RoomHandle;
    return tt.disable_remote_audio().toDart;
  }

  Future<dynamic> disable_remote_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return tt.disable_remote_video(source_kind).toDart;
  }

  Future<dynamic> enable_remote_audio() {
    final tt = this as _RoomHandle;
    return tt.enable_remote_audio().toDart;
  }

  Future<dynamic> enable_remote_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return tt.enable_remote_video(source_kind).toDart;
  }
}

@JS()
class RpcClientException {
  external void free();
  external num kind();
  external String message();
  external dynamic cause();
  external String trace();
}

@JS()
class StateError {
  external void free();
  external String message();
  external String trace();
}
