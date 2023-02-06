import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'ffi/jason_api.g.dart' as frb;

class DeviceVideoTrackConstraints implements base.DeviceVideoTrackConstraints {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.ApiDeviceVideoTrackConstrs constraints =
      frb.ApiDeviceVideoTrackConstrs(
          deviceId: null,
          facingMode: frb.ApiOptionConstrainFacingMode_None(),
          height: frb.ApiOptionConstrainU32_None(),
          width: frb.ApiOptionConstrainU32_None());

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    constraints.facingMode = frb.ApiOptionConstrainFacingMode_Some(
        frb.ApiConstrainFacingMode_Exact(facingMode));
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    constraints.facingMode = frb.ApiOptionConstrainFacingMode_Some(
        frb.ApiConstrainFacingMode_Ideal(facingMode));
  }

  @override
  void exactHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected u32');
    }
    constraints.height =
        frb.ApiOptionConstrainU32_Some(frb.ConstrainU32_Exact(height));
  }

  @override
  void idealHeight(int height) {
    if (height.isNegative || height.bitLength > 32) {
      throw ArgumentError.value(height, 'height', 'Expected u32');
    }
    constraints.height =
        frb.ApiOptionConstrainU32_Some(frb.ConstrainU32_Ideal(height));
  }

  @override
  void heightInRange(int min, int max) {
    if (min.isNegative || min.bitLength > 32) {
      throw ArgumentError.value(min, 'min', 'Expected u32');
    }
    if (max.isNegative || max.bitLength > 32) {
      throw ArgumentError.value(max, 'max', 'Expected u32');
    }
    constraints.height =
        frb.ApiOptionConstrainU32_Some(frb.ConstrainU32_Range(min, max));
  }

  @override
  void exactWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected u32');
    }
    constraints.width =
        frb.ApiOptionConstrainU32_Some(frb.ConstrainU32_Exact(width));
  }

  @override
  void idealWidth(int width) {
    if (width.isNegative || width.bitLength > 32) {
      throw ArgumentError.value(width, 'width', 'Expected u32');
    }
    constraints.width =
        frb.ApiOptionConstrainU32_Some(frb.ConstrainU32_Ideal(width));
  }

  @override
  void widthInRange(int min, int max) {
    if (min.isNegative || min.bitLength > 32) {
      throw ArgumentError.value(min, 'min', 'Expected u32');
    }
    if (max.isNegative || max.bitLength > 32) {
      throw ArgumentError.value(max, 'max', 'Expected u32');
    }

    constraints.width =
        frb.ApiOptionConstrainU32_Some(frb.ConstrainU32_Range(min, max));
  }

  @moveSemantics
  @override
  void free() {}
}
