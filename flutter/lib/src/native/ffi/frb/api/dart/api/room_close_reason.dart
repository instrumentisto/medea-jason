// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.1.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../../frb_generated.dart';
import '../api.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RoomCloseReason>>
abstract class RoomCloseReason implements RustOpaqueInterface, ForeignClass {
  /// Constructs a [`ForeignClass`] from the given raw pointer via
  /// [`Box::from_raw()`].
  ///
  /// # Safety
  ///
  /// Same as for [`Box::from_raw()`].
  static RoomCloseReason fromPtr({required int ptr}) => RustLib.instance.api
      .crateApiDartApiRoomCloseReasonRoomCloseReasonFromPtr(ptr: ptr);

  /// Indicates whether the [`Room`] was closed by server.
  ///
  /// [`Room`]: room::Room
  bool isClosedByServer();

  /// Indicates whether the [`Room`] close reason is considered as an error.
  ///
  /// [`Room`]: room::Room
  bool isErr();

  /// Returns the [`Room`]'s close reason.
  ///
  /// [`Room`]: room::Room
  String reason();
}
