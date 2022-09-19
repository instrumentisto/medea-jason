import '/src/util/rust_handles_storage.dart';
import 'media_device_info.dart';
import 'media_display_info.dart';
import 'media_stream_settings.dart';
import 'media_track.dart';

/// External handle to a `MediaManager`.
///
/// `MediaManager` performs all media acquisition requests
/// ([`getUserMedia()`][1]/[`getDisplayMedia()`][2]) and stores all received
/// tracks for further re-usage.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
abstract class MediaManagerHandle implements PlatformHandle {
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

  /// Returns a list of [MediaDisplayInfo] objects representing available
  /// displays.
  ///
  /// This method is supported on Linux, macOS and Windows platforms only.
  /// Throws an [UnsupportedError] on other platforms.
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [MediaManagerHandle], or on a [Jason] that
  /// implicitly owns native object behind this [MediaManagerHandle].
  ///
  /// Throws an [InternalException] on unexpected platform error.
  Future<List<MediaDisplayInfo>> enumerateDisplays();

  /// Switches output audio device to the device with the provided [deviceId].
  Future<void> setOutputAudioId(String deviceId);

  /// Indicates whether it's possible to access microphone volume settings.
  Future<bool> microphoneVolumeIsAvailable();

  /// Sets the microphone volume level in percents.
  Future<void> setMicrophoneVolume(int level);

  /// Returns the current microphone volume level in percents.
  Future<int> microphoneVolume();

  /// Subscribes onto the [MediaManagerHandle]'s `devicechange` event.
  void onDeviceChange(void Function() cb);
}
