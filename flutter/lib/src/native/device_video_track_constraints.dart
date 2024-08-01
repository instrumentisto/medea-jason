import '../interface/device_video_track_constraints.dart' as base;
import 'ffi/frb//api/dart/api.dart' as frb;
import 'ffi/frb/media/constraints.dart';

class DeviceVideoTrackConstraints implements base.DeviceVideoTrackConstraints {
  /// Rust `flutter_rust_bridge` API representation.
  final frb.ApiDeviceVideoTrackConstraints constraints =
      frb.ApiDeviceVideoTrackConstraints(
          deviceId: null, facingMode: null, height: null, width: null);

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    constraints.facingMode = frb.ApiConstrainFacingMode.exact(facingMode);
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    constraints.facingMode = frb.ApiConstrainFacingMode.ideal(facingMode);
  }

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
  void heightInRange(int min, int max) {
    if (min.isNegative || min.bitLength > 32) {
      throw ArgumentError.value(min, 'min', 'Expected `u32`');
    }
    if (max.isNegative || max.bitLength > 32) {
      throw ArgumentError.value(max, 'max', 'Expected `u32`');
    }
    constraints.height = ConstrainU32.range(min, max);
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
  void widthInRange(int min, int max) {
    if (min.isNegative || min.bitLength > 32) {
      throw ArgumentError.value(min, 'min', 'Expected `u32`');
    }
    if (max.isNegative || max.bitLength > 32) {
      throw ArgumentError.value(max, 'max', 'Expected `u32`');
    }

    constraints.width = ConstrainU32.range(min, max);
  }

  @override
  void free() {}
}
