import '../interface/device_video_track_constraints.dart' as base;
import 'ffi/jason_api.g.dart' as frb;

class DeviceVideoTrackConstraints implements base.DeviceVideoTrackConstraints {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.ApiDeviceVideoTrackConstrs constraints =
      frb.ApiDeviceVideoTrackConstrs(
          deviceId: null, facingMode: null, height: null, width: null);

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    constraints.facingMode = frb.ApiConstrainFacingMode_Exact(facingMode);
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    constraints.facingMode = frb.ApiConstrainFacingMode_Ideal(facingMode);
  }

  @override
  void exactHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected u32');
    }
    constraints.height = frb.ConstrainU32_Exact(height);
  }

  @override
  void idealHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected u32');
    }
    constraints.height = frb.ConstrainU32_Ideal(height);
  }

  @override
  void heightInRange(int min, int max) {
    if (min.isNegative || min.bitLength > 32) {
      throw ArgumentError.value(min, 'min', 'Expected u32');
    }
    if (max.isNegative || max.bitLength > 32) {
      throw ArgumentError.value(max, 'max', 'Expected u32');
    }
    constraints.height = frb.ConstrainU32_Range(min, max);
  }

  @override
  void exactWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected u32');
    }
    constraints.width = frb.ConstrainU32_Exact(width);
  }

  @override
  void idealWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected u32');
    }
    constraints.width = frb.ConstrainU32_Ideal(width);
  }

  @override
  void widthInRange(int min, int max) {
    if (min.isNegative || min.bitLength > 32) {
      throw ArgumentError.value(min, 'min', 'Expected u32');
    }
    if (max.isNegative || max.bitLength > 32) {
      throw ArgumentError.value(max, 'max', 'Expected u32');
    }

    constraints.width = frb.ConstrainU32_Range(min, max);
  }

  @override
  void free() {}
}
