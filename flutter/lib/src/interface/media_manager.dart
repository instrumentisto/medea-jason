import '../util/move_semantic.dart';
import 'input_device_info.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';

abstract class MediaManagerHandle {
  /// Obtains [LocalMediaTrack]s objects from local media devices (or screen
  /// capture) basing on the provided [IMediaStreamSettings].
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [MediaManagerHandle], or on a [Jason] that
  /// implicitly owns native object behind this [MediaManagerHandle].
  ///
  /// Throws a [LocalMediaInitException] if a request of platform media devices
  /// access failed.
  Future<List<LocalMediaTrack>> initLocalTracks(IMediaStreamSettings caps) {
    throw UnimplementedError();
  }

  /// Returns a list of [InputDeviceInfo] objects representing available media
  /// input devices, such as microphones, cameras, and so forth.
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [MediaManagerHandle], or on a [Jason] that
  /// implicitly owns native object behind this [MediaManagerHandle].
  ///
  /// Throws a [EnumerateDevicesException] if a request of platform media
  /// devices access failed.
  Future<List<InputDeviceInfo>> enumerateDevices() {
    throw UnimplementedError();
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
