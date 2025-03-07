// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../../frb_generated.dart';
import '../../../media/constraints.dart';
import '../../../media/track.dart';
import '../api.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RoomHandle>>
abstract class RoomHandle implements RustOpaqueInterface {
  /// Disables outbound audio in the provided [`Room`].
  Object disableAudio();

  /// Disables inbound audio in the provided [`Room`].
  Object disableRemoteAudio();

  /// Disables inbound video in the provided [`Room`].
  ///
  /// Affects only video with the provided [`MediaSourceKind`], if any.
  ///
  /// # Errors
  ///
  /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
  Object disableRemoteVideo({MediaSourceKind? sourceKind});

  /// Disables outbound video in the provided [`Room`].
  ///
  /// Affects only video with the provided [`MediaSourceKind`], if any.
  ///
  /// # Errors
  ///
  /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
  Object disableVideo({MediaSourceKind? sourceKind});

  /// Enables outbound audio in the provided [`Room`].
  Object enableAudio();

  /// Enables inbound audio in the provided [`Room`].
  Object enableRemoteAudio();

  /// Enables inbound video in the provided [`Room`].
  ///
  /// Affects only video with the provided [`MediaSourceKind`], if any.
  ///
  /// # Errors
  ///
  /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
  Object enableRemoteVideo({MediaSourceKind? sourceKind});

  /// Enables outbound video in the provided [`Room`].
  ///
  /// Affects only video with the provided [`MediaSourceKind`], if any.
  ///
  /// # Errors
  ///
  /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
  Object enableVideo({MediaSourceKind? sourceKind});

  /// Connects to a media server and joins the [`Room`] with the provided
  /// authorization `token`.
  ///
  /// Authorization token has a fixed format:
  /// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
  /// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
  Object join({required String token});

  /// Mutes outbound audio in the provided [`Room`].
  Object muteAudio();

  /// Mutes outbound video in the provided [`Room`].
  ///
  /// Affects only video with the provided [`MediaSourceKind`], if any.
  ///
  /// # Errors
  ///
  /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
  Object muteVideo({MediaSourceKind? sourceKind});

  /// Sets a callback to be invoked once the provided [`Room`] is closed,
  /// providing a [`RoomCloseReason`].
  ///
  /// # Errors
  ///
  /// If the [`core::RoomHandle::on_close()`] method errors.
  void onClose({required Object cb});

  /// Sets a callback to be invoked once a connection with a media server is
  /// lost.
  ///
  /// # Errors
  ///
  /// If the [`core::RoomHandle::on_connection_loss()`] method errors.
  void onConnectionLoss({required Object cb});

  /// Sets a callback to be invoked on local media acquisition failures.
  ///
  /// # Errors
  ///
  /// If the [`core::RoomHandle::on_failed_local_media()`] method errors.
  void onFailedLocalMedia({required Object cb});

  /// Sets a callback to be invoked once a new [`LocalMediaTrack`] is added
  /// to the provided [`Room`].
  ///
  /// This might happen in such cases:
  /// 1. Media server initiates a media request.
  /// 2. [`enable_audio()`]/[`enable_video()`] is called.
  /// 3. [`MediaStreamSettings`] are updated via
  ///    [`set_local_media_settings()`].
  ///
  /// # Errors
  ///
  /// If the [`core::RoomHandle::on_local_track()`] method errors.
  ///
  /// [`enable_audio()`]: RoomHandle::enable_audio
  /// [`enable_video()`]: RoomHandle::enable_video
  /// [`MediaStreamSettings`]: media::MediaStreamSettings
  /// [`set_local_media_settings()`]: RoomHandle::set_local_media_settings
  void onLocalTrack({required Object cb});

  /// Sets a callback to be invoked once a new [`Connection`] with some remote
  /// `Peer` is established.
  ///
  /// # Errors
  ///
  /// If the [`core::RoomHandle::on_new_connection()`] method errors.
  ///
  /// [`Connection`]: connection::Connection
  void onNewConnection({required Object cb});

  /// Updates this [`Room`]'s [`ApiMediaStreamSettings`].
  ///
  /// This affects all the [`PeerConnection`]s in this [`Room`]. If
  /// [`ApiMediaStreamSettings`] are configured for some [`Room`], then this
  /// [`Room`] can only send media tracks that correspond to these settings.
  /// [`ApiMediaStreamSettings`] update will change media tracks in all
  /// sending peers, so that might cause a new [getUserMedia()][1] request to
  /// happen.
  ///
  /// Media obtaining/injection errors are additionally fired to a
  /// [`on_failed_local_media`] callback.
  ///
  /// If the `stop_first` argument is [`true`], then affected
  /// [`LocalMediaTrack`]s will be dropped before new
  /// [`ApiMediaStreamSettings`] are applied. This is usually required when
  /// changing video source device due to hardware limitations, e.g. having an
  /// active track sourced from device `A` may hinder [getUserMedia()][1]
  /// requests to device `B`.
  ///
  /// The `rollback_on_fail` argument configures [`ApiMediaStreamSettings`]
  /// update request to automatically roll back to previous settings if new
  /// settings cannot be applied.
  ///
  /// If recovering from fail state isn't possible then affected media types
  /// will be disabled.
  ///
  /// [`on_failed_local_media`]: RoomHandle::on_failed_local_media
  /// [`PeerConnection`]: crate::peer::PeerConnection
  /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
  Object setLocalMediaSettings({
    required ApiMediaStreamSettings settings,
    required bool stopFirst,
    required bool rollbackOnFail,
  });

  /// Unmutes outbound audio in the provided [`Room`].
  Object unmuteAudio();

  /// Unmutes outbound video in the provided [`Room`].
  ///
  /// Affects only video with the provided [`MediaSourceKind`], if any.
  ///
  /// # Errors
  ///
  /// If the provided `source_kind` is not a [`MediaSourceKind`] index.
  Object unmuteVideo({MediaSourceKind? sourceKind});
}
