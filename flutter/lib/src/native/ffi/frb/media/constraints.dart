// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;

import '../frb_generated.dart';

part 'constraints.freezed.dart';

@freezed
sealed class ConstrainBoolean with _$ConstrainBoolean {
  const ConstrainBoolean._();

  /// Exact value required for this property.
  const factory ConstrainBoolean.exact(bool field0) = ConstrainBoolean_Exact;

  /// Ideal (target) value for this property.
  const factory ConstrainBoolean.ideal(bool field0) = ConstrainBoolean_Ideal;
}

@freezed
sealed class ConstrainU32 with _$ConstrainU32 {
  const ConstrainU32._();

  /// Must be the parameter's value.
  const factory ConstrainU32.exact(int field0) = ConstrainU32_Exact;

  /// Should be used if possible.
  const factory ConstrainU32.ideal(int field0) = ConstrainU32_Ideal;

  /// Parameter's value must be in this range.
  const factory ConstrainU32.range(int field0, int field1) = ConstrainU32_Range;
}

/// Describes directions that a camera can face, as seen from a user's
/// perspective.
///
/// Representation of a [VideoFacingModeEnum][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
enum FacingMode {
  /// Facing towards a user (a self-view camera).
  user,

  /// Facing away from a user (viewing an environment).
  environment,

  /// Facing to the left of a user.
  left,

  /// Facing to the right of a user.
  right,
}

/// Audio processing noise suppression aggressiveness.
enum NoiseSuppressionLevel {
  /// Minimal noise suppression.
  low,

  /// Moderate level of suppression.
  moderate,

  /// Aggressive noise suppression.
  high,

  /// Maximum suppression.
  veryHigh,
}
