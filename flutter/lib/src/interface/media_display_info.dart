import '/src/util/rust_handles_storage.dart';

/// [`MediaDisplayInfo`] interface.
abstract class MediaDisplayInfo implements PlatformHandle {
  /// Returns a unique identifier of the represented display.
  String deviceId();

  /// Title of the represented display.
  String? title();
}
