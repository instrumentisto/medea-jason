import '/src/util/rust_handles_storage.dart';

/// Constraints applicable to video tracks sourced from a screen capturing.
abstract class DisplayVideoTrackConstraints implements PlatformHandle {
  /// Sets an exact [`height`][1] constraint.
  ///
  /// Converts the provided [height] into an `u32`. Throws an [ArgumentError] if
  /// conversion fails.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
  void exactHeight(int height);

  /// Sets an ideal [`height`][1] constraint.
  ///
  /// Converts the provided [height] into an `u32`. Throws an [ArgumentError] if
  /// conversion fails.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
  void idealHeight(int height);

  /// Sets an exact [`width`][1] constraint.
  ///
  /// Converts the provided [width] into an `u32`. Throws an [ArgumentError] if
  /// conversion fails.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
  void exactWidth(int width);

  /// Sets an ideal [`width`][1] constraint.
  ///
  /// Converts the provided [width] into an `u32`. Throws an [ArgumentError] if
  /// conversion fails.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
  void idealWidth(int width);

  /// Sets an ideal [`frameRate`][1] constraint.
  ///
  /// Converts the provided [frameRate] into an `u32`. Throws an [ArgumentError]
  /// if conversion fails.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
  void idealFrameRate(int frameRate);

  /// Sets an exact [`frameRate`][1] constraint.
  ///
  /// Converts the provided [frameRate] into an `u32`. Throws an [ArgumentError]
  /// if conversion fails.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
  void exactFrameRate(int frameRate);

  /// Sets an exact [`deviceId`][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  void deviceId(String deviceId);
}
