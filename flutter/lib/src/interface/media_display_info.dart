import '/src/util/rust_handles_storage.dart';

//todo
/// [`MediaDisplayInfo`] interface.
abstract class MediaDisplayInfo implements PlatformHandle {
  /// Returns an unique identifier of the represented display.
  String deviceId();

  String? title();
}
