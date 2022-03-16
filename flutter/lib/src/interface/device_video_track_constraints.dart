import '/src/util/rust_handles_storage.dart';

/// Describes the directions that the camera can face, as seen from a user's
/// perspective.
///
/// Representation of the [`VideoFacingModeEnum`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
enum FacingMode {
  /// Facing towards the user (a self-view camera).
  User,

  /// Facing away from the user (viewing an environment).
  Environment,

  /// Facing to the left of the user.
  Left,

  /// Facing to the right of the user.
  Right,
}

abstract class DeviceVideoTrackConstraints implements PlatformHandle {
  /// Sets an exact [`deviceId`][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  void deviceId(String deviceId);

  /// Sets an exact [`facingMode`][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
  void exactFacingMode(FacingMode facingMode);

  /// Sets an ideal [`facingMode`][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
  void idealFacingMode(FacingMode facingMode);

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

  /// Sets a range of a [`height`][1] constraint.
  ///
  /// Converts the provided [min] and [max] into an `u32`. Throws an
  /// [ArgumentError] if conversion fails.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
  void heightInRange(int min, int max);

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

  /// Sets a range of a [`width`][1] constraint.
  ///
  /// Converts the provided [min] and [max] into an `u32`. Throws an
  /// [ArgumentError] if conversion fails.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
  void widthInRange(int min, int max);
}
