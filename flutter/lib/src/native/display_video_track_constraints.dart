import '../interface/display_video_track_constraints.dart' as base;
import 'ffi/frb/frb.dart' as frb;
import 'ffi/frb/media/constraints.dart';

class DisplayVideoTrackConstraints
    implements base.DisplayVideoTrackConstraints {
  /// Rust `flutter_rust_bridge` API representation.
  frb.ApiDisplayVideoTrackConstraints constraints =
      frb.ApiDisplayVideoTrackConstraints(
        deviceId: null,
        height: null,
        width: null,
        frameRate: null,
      );

  @override
  void exactHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected `u32`');
    }
    constraints.height = ConstrainU32.exact(height);
  }

  @override
  void idealHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected `u32`');
    }
    constraints.height = ConstrainU32.ideal(height);
  }

  @override
  void exactWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected `u32`');
    }
    constraints.width = ConstrainU32.exact(width);
  }

  @override
  void idealWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected `u32`');
    }
    constraints.width = ConstrainU32.ideal(width);
  }

  @override
  void exactFrameRate(int frameRate) {
    if (frameRate.isNegative || frameRate.bitLength > 32) {
      throw ArgumentError.value(frameRate, 'frameRate', 'Expected `u32`');
    }
    constraints.frameRate = ConstrainU32.exact(frameRate);
  }

  @override
  void idealFrameRate(int frameRate) {
    if (frameRate.isNegative || frameRate.bitLength > 32) {
      throw ArgumentError.value(frameRate, 'frameRate', 'Expected `u32`');
    }
    constraints.frameRate = ConstrainU32.ideal(frameRate);
  }

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void free() {}
}
