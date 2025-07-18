// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../../frb_generated.dart';
import 'media_manager.dart';
import 'room.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Jason>>
abstract class Jason implements RustOpaqueInterface {
  /// Closes the provided [`RoomHandle`].
  void jasonCloseRoom({required RoomHandle roomToDelete});

  /// Closes this [`Jason`].
  void jasonDispose();

  /// Creates a new [`Room`] and returns its [`RoomHandle`].
  RoomHandle jasonInitRoom();

  /// Returns a [`MediaManagerHandle`].
  MediaManagerHandle jasonMediaManager();

  /// Instantiates a new [`Jason`] interface to interact with this library.
  factory Jason() => RustLib.instance.api.crateApiDartApiJasonJasonNew();
}
