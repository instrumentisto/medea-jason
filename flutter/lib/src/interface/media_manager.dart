import 'media_device_info.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import '../util/move_semantic.dart';

/// External handle to a `MediaManager`.
///
/// `MediaManager` performs all media acquisition requests
/// ([`getUserMedia()`][1]/[`getDisplayMedia()`][2]) and stores all received
/// tracks for further re-usage.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
abstract class MediaManagerHandle {
  /// Obtains [LocalMediaTrack]s objects from local media devices (or screen
  /// capture) basing on the provided [MediaStreamSettings].
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [MediaManagerHandle], or on a [Jason] that
  /// implicitly owns native object behind this [MediaManagerHandle].
  ///
  /// Throws a [LocalMediaInitException] if a request of platform media devices
  /// access failed.
  Future<List<LocalMediaTrack>> initLocalTracks(MediaStreamSettings caps);

  /// Returns a list of [MediaDeviceInfo] objects representing available media
  /// input devices, such as microphones, cameras, and so forth.
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [MediaManagerHandle], or on a [Jason] that
  /// implicitly owns native object behind this [MediaManagerHandle].
  ///
  /// Throws a [EnumerateDevicesException] if a request of platform media
  /// devices access failed.
  Future<List<MediaDeviceInfo>> enumerateDevices();

  /// Switches output audio device to the device with the provided [deviceId].
  Future<void> setOutputAudioId(String deviceId);

  /// Subscribes onto the [MediaManagerHandle]'s `devicechange` event.
  void onDeviceChange(Function cb);

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}
