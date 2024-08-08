// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.1.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../../frb_generated.dart';
import 'media_manager.dart';
import 'room.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<JasonHandle>>
abstract class JasonHandle implements RustOpaqueInterface {
  /// Closes the provided [`RoomHandle`].
  void jasonCloseRoom({required RoomHandle roomToDelete});

  /// Closes the provided [`RoomHandle`].
  void jasonDispose();

  /// Creates a new [`Room`] and returns its [`RoomHandle`].
  ///
  /// [`Room`]: room::Room
  RoomHandle jasonInitRoom();

  /// Returns a [`MediaManagerHandle`].
  MediaManagerHandle jasonMediaManager();

  factory JasonHandle() =>
      RustLib.instance.api.crateApiDartApiJasonJasonHandleNew();
}
