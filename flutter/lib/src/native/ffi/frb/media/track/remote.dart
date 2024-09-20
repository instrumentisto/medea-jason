// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../frb_generated.dart';

/// Media exchange direction of a [`Track`].
enum MediaDirection {
  /// [`Track`] is enabled on both receiver and sender sides.
  sendRecv,

  /// [`Track`] is enabled on sender side only.
  sendOnly,

  /// [`Track`] is enabled on receiver side only.
  recvOnly,

  /// [`Track`] is disabled on both sides.
  inactive,
  ;
}
