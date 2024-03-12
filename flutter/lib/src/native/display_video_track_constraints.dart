import '../interface/display_video_track_constraints.dart' as base;
import 'ffi/jason_api.g.dart' as frb;

class DisplayVideoTrackConstraints
    implements base.DisplayVideoTrackConstraints {
  /// Rust `flutter_rust_bridge` API representation.
  frb.ApiDisplayVideoTrackConstraints constraints =
      frb.ApiDisplayVideoTrackConstraints(
          deviceId: null, height: null, width: null, frameRate: null);

  @override
  void exactHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected `u32`');
    }
    constraints.height = frb.ConstrainU32_Exact(height);
  }

  @override
  void idealHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected `u32`');
    }
    constraints.height = frb.ConstrainU32_Ideal(height);
  }

  @override
  void exactWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected `u32`');
    }
    constraints.width = frb.ConstrainU32_Exact(width);
  }

  @override
  void idealWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected `u32`');
    }
    constraints.width = frb.ConstrainU32_Ideal(width);
  }

  @override
  void exactFrameRate(int frameRate) {
    if (frameRate.isNegative || frameRate.bitLength > 32) {
      throw ArgumentError.value(frameRate, 'frameRate', 'Expected `u32`');
    }
    constraints.frameRate = frb.ConstrainU32_Exact(frameRate);
  }

  @override
  void idealFrameRate(int frameRate) {
    if (frameRate.isNegative || frameRate.bitLength > 32) {
      throw ArgumentError.value(frameRate, 'frameRate', 'Expected `u32`');
    }
    constraints.frameRate = frb.ConstrainU32_Ideal(frameRate);
  }

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void free() {}
}
