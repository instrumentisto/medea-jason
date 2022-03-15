import '/src/native/audio_track_constraints.dart';
import '/src/native/connection_handle.dart';
import '/src/native/device_video_track_constraints.dart';
import '/src/native/display_video_track_constraints.dart';
import '/src/native/input_device_info.dart';
import '/src/native/jason.dart';
import '/src/native/local_media_track.dart';
import '/src/native/media_manager.dart';
import '/src/native/media_stream_settings.dart';
import '/src/native/reconnect_handle.dart';
import '/src/native/remote_media_track.dart';
import '/src/native/room_close_reason.dart';
import '/src/native/room_handle.dart';

/// Store for the all Rust handles created returned from Rust.
class RustHandlesStorage {
  static final RustHandlesStorage _singleton = RustHandlesStorage._internal();

  /// All handles given for the Dart side from the Rust side.
  List<dynamic> _handles = [];

  factory RustHandlesStorage() {
    return _singleton;
  }

  RustHandlesStorage._internal();

  /// Insert provided [handle] to this [RustHandlesStorage].
  void insertHandle(dynamic handle) {
    _handles.add(handle);
  }

  /// Disposes all Rust handles registered in this [RustHandlesStorage].
  void freeAll() {
    _handles.forEach((h) {
      // TODO(alexlapa): this should be abstracted out so we won't need
      //                 type checks
      switch (h.runtimeType) {
        case AudioTrackConstraints:
          h as AudioTrackConstraints;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeConnectionHandle:
          h as NativeConnectionHandle;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case DeviceVideoTrackConstraints:
          h as DeviceVideoTrackConstraints;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case DisplayVideoTrackConstraints:
          h as DisplayVideoTrackConstraints;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeInputDeviceInfo:
          h as NativeInputDeviceInfo;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case Jason:
          h as Jason;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeLocalMediaTrack:
          h as NativeLocalMediaTrack;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeMediaManagerHandle:
          h as NativeMediaManagerHandle;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case MediaStreamSettings:
          h as MediaStreamSettings;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeReconnectHandle:
          h as NativeReconnectHandle;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeRemoteMediaTrack:
          h as NativeRemoteMediaTrack;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeRoomCloseReason:
          h as NativeRoomCloseReason;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
        case NativeRoomHandle:
          h as NativeRoomHandle;
          if (!h.ptr.isFreed()) {
            h.free();
          }
          break;
      }
    });
    _handles = [];
  }
}
