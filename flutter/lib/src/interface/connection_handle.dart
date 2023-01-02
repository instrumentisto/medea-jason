import '/src/util/rust_handles_storage.dart';
import 'media_track.dart';

/// External handler to a `Connection` with a remote `Member`.
abstract class ConnectionHandle implements SyncPlatformHandle {
  /// Returns ID of the remote `Member`.
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  String getRemoteMemberId();

  /// Sets callback, invoked when this `Connection` is closed.
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  void onClose(void Function() f);

  /// Sets callback, invoked when a new [RemoteMediaTrack] is added to this
  /// `Connection`.
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f);

  /// Sets callback, invoked when a connection quality score is updated by a
  /// server.
  ///
  /// Throws a [StateError] if an underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  void onQualityScoreUpdate(void Function(int) f);

  /// Enables inbound audio in this `Connection`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if
  /// [ConnectionHandle.disableRemoteAudio] was called while enabling or a
  /// media server didn't approve this state transition.
  Future<void> enableRemoteAudio();

  /// Disables inbound audio in this `Connection`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if
  /// [ConnectionHandle.enableRemoteAudio] was called while disabling or a
  /// media server didn't approve this state transition.
  Future<void> disableRemoteAudio();

  /// Enables inbound video in this `Connection`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if
  /// [ConnectionHandle.disableRemoteVideo] was called while enabling or a
  /// media server didn't approve this state transition.
  Future<void> enableRemoteVideo([MediaSourceKind? kind]);

  /// Disables inbound video in this `Connection`.
  ///
  /// Throws a [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Throws a `MediaStateTransitionException` if
  /// [ConnectionHandle.enableRemoteVideo] was called while disabling or a
  /// media server didn't approve this state transition.
  Future<void> disableRemoteVideo([MediaSourceKind? kind]);
}
