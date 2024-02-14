import '/src/util/rust_handles_storage.dart';

/// [`MediaDisplayDetails`] interface.
abstract class MediaDisplayDetails implements SyncPlatformHandle {
  /// Returns a unique identifier of the represented display.
  String deviceId();

  /// Title of the represented display.
  String? title();
}
