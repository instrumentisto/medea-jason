// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../../frb_generated.dart';
import '../../../media.dart';
import '../../../media/track.dart';
import '../../../media/track/remote.dart';
import '../api.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RemoteMediaTrack>>
abstract class RemoteMediaTrack implements RustOpaqueInterface, ForeignClass {
  /// Constructs a [`ForeignClass`] from the given raw pointer via
  /// [`Box::from_raw()`].
  ///
  /// # Safety
  ///
  /// Same as for [`Box::from_raw()`].
  static RemoteMediaTrack fromPtr({required int ptr}) => RustLib.instance.api
      .crateApiDartApiRemoteMediaTrackRemoteMediaTrackFromPtr(ptr: ptr);

  /// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
  /// [`RemoteMediaTrack`].
  ///
  /// [`MediaStreamTrack`]: platform::MediaStreamTrack
  Object getTrack();

  /// Returns this [`RemoteMediaTrack`]'s kind (audio/video).
  MediaKind kind();

  /// Returns the current general [`MediaDirection`] of this
  /// [`RemoteMediaTrack`].
  MediaDirection mediaDirection();

  /// Returns this [`RemoteMediaTrack`]'s media source kind.
  MediaSourceKind mediaSourceKind();

  /// Indicate whether this [`RemoteMediaTrack`] is muted.
  bool muted();

  /// Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
  /// [`MediaDirection`] is changed.
  void onMediaDirectionChanged({required Object f});

  /// Sets callback to invoke once this [`RemoteMediaTrack`] is muted.
  void onMuted({required Object f});

  /// Sets callback to invoke once this [`RemoteMediaTrack`] is stopped.
  void onStopped({required Object f});

  /// Sets callback to invoke once this [`RemoteMediaTrack`] is unmuted.
  void onUnmuted({required Object f});
}
