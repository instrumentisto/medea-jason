@JS()
library medea_jason;

import 'dart:html' as html;

import 'package:js/js.dart';
import 'package:js/js_util.dart' show promiseToFuture;

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
}

@JS()
class ConnectionHandle {
  external void free();
  external void on_close(Function cb);
  external String get_remote_member_id();
  external void on_remote_track_added(Function cb);
  external void on_quality_score_update(Function cb);
}

@JS('ConnectionHandle')
abstract class _ConnectionHandle {
  external Promise<dynamic> disable_remote_audio();
  external Promise<dynamic> disable_remote_video(num? source_kind);
  external Promise<dynamic> enable_remote_audio();
  external Promise<dynamic> enable_remote_video(num? source_kind);
}

extension ConnectionHandleExtensions on ConnectionHandle {
  Future<dynamic> disable_remote_audio() {
    final tt = this as _ConnectionHandle;
    return promiseToFuture(tt.disable_remote_audio());
  }

  Future<dynamic> disable_remote_video(num? source_kind) {
    final tt = this as _ConnectionHandle;
    return promiseToFuture(tt.disable_remote_video(source_kind));
  }

  Future<dynamic> enable_remote_audio() {
    final tt = this as _ConnectionHandle;
    return promiseToFuture(tt.enable_remote_audio());
  }

  Future<dynamic> enable_remote_video(num? source_kind) {
    final tt = this as _ConnectionHandle;
    return promiseToFuture(tt.enable_remote_video(source_kind));
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
}

@JS()
class DisplayVideoTrackConstraints {
  external void free();
  external factory DisplayVideoTrackConstraints();
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
class MediaDeviceInfo {
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
  external html.MediaStreamTrack get_track();
  external num kind();
  external num media_source_kind();
}

@JS()
class MediaManagerHandle {
  external void on_device_change(Function cb);
  external void free();
}

@JS('MediaManagerHandle')
abstract class _MediaManagerHandle {
  external Promise<List<dynamic>> enumerate_devices();
  external Promise<List<dynamic>> init_local_tracks(MediaStreamSettings caps);
}

extension MediaManagerHandleExtensions on MediaManagerHandle {
  Future<List<dynamic>> enumerate_devices() {
    final tt = this as _MediaManagerHandle;
    return promiseToFuture(tt.enumerate_devices());
  }

  Future<List<dynamic>> init_local_tracks(MediaStreamSettings caps) {
    final tt = this as _MediaManagerHandle;
    return promiseToFuture(tt.init_local_tracks(caps));
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
  external Promise<dynamic> reconnect_with_delay(num delay_ms);
  external Promise<dynamic> reconnect_with_backoff(num starting_delay_ms,
      num multiplier, num max_delay, num? max_elapsed_time_ms);
}

extension ReconnectHandleExtensions on ReconnectHandle {
  Future<dynamic> reconnect_with_delay(num delay_ms) {
    final tt = this as _ReconnectHandle;
    return promiseToFuture(tt.reconnect_with_delay(delay_ms));
  }

  Future<dynamic> reconnect_with_backoff(num starting_delay_ms, num multiplier,
      num max_delay, num? max_elapsed_time_ms) {
    final tt = this as _ReconnectHandle;
    return promiseToFuture(tt.reconnect_with_backoff(
        starting_delay_ms, multiplier, max_delay, max_elapsed_time_ms));
  }
}

@JS()
class RemoteMediaTrack {
  external void free();
  external html.MediaStreamTrack get_track();
  external bool enabled();
  external bool muted();
  external void on_enabled(Function cb);
  external void on_disabled(Function cb);
  external void on_muted(Function cb);
  external void on_track_update(Function cb);
  external void on_unmuted(Function cb);
  external void on_stopped(Function cb);
  external void on_media_direction_changed(Function cb);
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
  external void on_new_connection(Function cb);
  external void on_close(Function cb);
  external void on_local_track(Function cb);
  external void on_failed_local_media(Function cb);
  external void on_connection_loss(Function cb);
}

@JS('RoomHandle')
abstract class _RoomHandle {
  external Promise<dynamic> join(String token);
  external Promise<dynamic> set_local_media_settings(
      MediaStreamSettings settings, bool stop_first, bool rollback_on_fail);
  external Promise<dynamic> mute_audio();
  external Promise<dynamic> unmute_audio();
  external Promise<dynamic> mute_video(num? source_kind);
  external Promise<dynamic> unmute_video(num? source_kind);
  external Promise<dynamic> disable_audio();
  external Promise<dynamic> enable_audio();
  external Promise<dynamic> disable_video(num? source_kind);
  external Promise<dynamic> enable_video(num? source_kind);
  external Promise<dynamic> disable_remote_audio();
  external Promise<dynamic> disable_remote_video(num? source_kind);
  external Promise<dynamic> enable_remote_audio();
  external Promise<dynamic> enable_remote_video(num? source_kind);
}

extension RoomHandleExtensions on RoomHandle {
  Future<dynamic> join(String token) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.join(token));
  }

  Future<dynamic> set_local_media_settings(
      MediaStreamSettings settings, bool stop_first, bool rollback_on_fail) {
    final tt = this as _RoomHandle;
    return promiseToFuture(
        tt.set_local_media_settings(settings, stop_first, rollback_on_fail));
  }

  Future<dynamic> mute_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.mute_audio());
  }

  Future<dynamic> unmute_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.unmute_audio());
  }

  Future<dynamic> mute_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.mute_video(source_kind));
  }

  Future<dynamic> unmute_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.unmute_video(source_kind));
  }

  Future<dynamic> disable_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_audio());
  }

  Future<dynamic> enable_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_audio());
  }

  Future<dynamic> disable_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_video(source_kind));
  }

  Future<dynamic> enable_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_video(source_kind));
  }

  Future<dynamic> disable_remote_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_remote_audio());
  }

  Future<dynamic> disable_remote_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_remote_video(source_kind));
  }

  Future<dynamic> enable_remote_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_remote_audio());
  }

  Future<dynamic> enable_remote_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_remote_video(source_kind));
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

@JS()
abstract class Promise<T> {
  external factory Promise(
      void Function(void Function(T result) resolve, Function reject) executor);
  external Promise then(void Function(T result) onFulfilled,
      [Function onRejected]);
}
