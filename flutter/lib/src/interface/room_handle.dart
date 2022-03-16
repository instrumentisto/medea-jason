import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';
import 'track_kinds.dart';

/// External handle to a `Room`.
abstract class RoomHandle implements PlatformHandle {
  /// Connects to a media server and joins the `Room` with the provided
  /// authorization [token].
  ///
  /// Authorization token has a fixed format:
  /// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
  /// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed or if some
  /// mandatory callback is not set. These callbacks are:
  /// [RoomHandle.onConnectionLoss] and [RoomHandle.onFailedLocalMedia].
  ///
  /// Throws [FormatException] if the provided [token] string has bad format.
  ///
  /// Throws `RpcClientException` if could not connect to media server.
  Future<void> join(String token);

  /// Updates this `Room`'s [MediaStreamSettings]. This affects all the
  /// `PeerConnection`s in this `Room`. If [MediaStreamSettings] are configured
  /// for some `Room`, then this `Room` can only send media tracks that
  /// correspond to these settings. [MediaStreamSettings] update will change
  /// media tracks in all sending peers, so that might cause a new
  /// [getUserMedia()][1] request to happen.
  ///
  /// Media obtaining/injection errors are additionally fired to
  /// [RoomHandle.onFailedLocalMedia()] callback.
  ///
  /// If [stop_first] set to `true` then affected local [LocalMediaTrack]s will
  /// be dropped before new [MediaStreamSettings] are applied. This is usually
  /// required when changing video source device due to hardware limitations,
  /// e.g. having an active track sourced from device `A` may hinder
  /// [getUserMedia()][1] requests to device `B`.
  ///
  /// [rollback_on_fail] option configures [MediaStreamSettings] update request
  /// to automatically rollback to previous settings if new settings cannot be
  /// applied.
  ///
  /// If recovering from fail state isn't possible then affected media types
  /// will be disabled.
  ///
  /// Throws a [MediaSettingsUpdateException] if settings could not be updated.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
  Future<void> setLocalMediaSettings(
      MediaStreamSettings settings, bool stopFirst, bool rollbackOnFail);

  /// Mutes outbound audio in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.unmuteAudio] was
  /// called while muting or a media server didn't approve this state
  /// transition.
  Future<void> muteAudio();

  /// Unmutes outbound audio in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.muteAudio] was
  /// called while unmuting or a media server didn't approve this state
  /// transition.
  Future<void> unmuteAudio();

  /// Enables outbound audio in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.disableAudio] was
  /// called while enabling or a media server didn't approve this state
  /// transition.
  ///
  /// Throws a `LocalMediaInitException` if a request of platform media devices
  /// access failed.
  Future<void> enableAudio();

  /// Disables outbound audio in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.enableAudio] was
  /// called while disabling or a media server didn't approve this state
  /// transition.
  Future<void> disableAudio();

  /// Mutes outbound video in this `Room`.
  ///
  /// Affects only video with specific [MediaSourceKind] if specified.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.unmuteVideo] was
  /// called while muting or a media server didn't approve this state
  /// transition.
  Future<void> muteVideo([MediaSourceKind? kind]);

  /// Unmutes outbound video in this `Room`.
  ///
  /// Affects only video with specific [MediaSourceKind] if specified.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.muteVideo] was
  /// called while unmuting or a media server didn't approve this state
  /// transition.
  Future<void> unmuteVideo([MediaSourceKind? kind]);

  /// Enables outbound video.
  ///
  /// Affects only video with specific [MediaSourceKind] if specified.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.disableVideo] was
  /// called while enabling or a media server didn't approve this state
  /// transition.
  ///
  /// Throws a `LocalMediaInitException` if a request of platform media devices
  /// access failed.
  Future<void> enableVideo([MediaSourceKind? kind]);

  /// Disables outbound video.
  ///
  /// Affects only video with specific [MediaSourceKind] if specified.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.enableVideo] was
  /// called while disabling or a media server didn't approve this state
  /// transition.
  Future<void> disableVideo([MediaSourceKind? kind]);

  /// Enables inbound audio in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if
  /// [RoomHandle.disableRemoteAudio] was called while enabling or a media
  /// server didn't approve this state transition.
  Future<void> enableRemoteAudio();

  /// Disables inbound audio in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if [RoomHandle.enableRemoteAudio]
  /// was called while disabling or a media server didn't approve this state
  /// transition.
  Future<void> disableRemoteAudio();

  /// Enables inbound video in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if
  /// [RoomHandle.disableRemoteVideo] was called while enabling or a media
  /// server didn't approve this state transition.
  Future<void> enableRemoteVideo();

  /// Disables inbound video in this `Room`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if
  /// [RoomHandle.enableRemoteVideo] was called while disabling or a media
  /// server didn't approve this state transition.
  Future<void> disableRemoteVideo();

  /// Sets callback, invoked when a new `Connection` with some remote `Peer`
  /// is established.
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed.
  void onNewConnection(void Function(ConnectionHandle) f);

  /// Sets callback, invoked when this `Room` is closed, providing a
  /// [RoomCloseReason].
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed.
  void onClose(void Function(RoomCloseReason) f);

  /// Sets callback, invoked when a new [LocalMediaTrack] is added to this
  /// `Room`.
  ///
  /// This might happen in the following cases:
  /// 1. Media server initiates a media request.
  /// 2. [RoomHandle.enableAudio] or [RoomHandle.enableVideo] call resulted in
  ///    new media track acquisition.
  /// 3. [MediaStreamSettings] were updated via
  ///    [RoomHandle.setLocalMediaSettings] method.
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed.
  void onLocalTrack(void Function(LocalMediaTrack) f);

  /// Sets callback, invoked when a connection with a media server is lost,
  /// providing a [ReconnectHandle].
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed.
  void onConnectionLoss(void Function(ReconnectHandle) f);

  /// Sets callback, invoked on a local media acquisition failures.
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed.
  void onFailedLocalMedia(void Function(Object) f);
}
