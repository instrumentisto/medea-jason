import '/src/util/rust_handles_storage.dart';
import 'media_track.dart';
import 'member_connection_state.dart';

/// External handler to a `Connection` with a remote `Member`.
abstract class ConnectionHandle implements SyncPlatformHandle {
  /// Returns ID of the remote `Member`.
  ///
  /// Throws a [StateError] if the underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  String getRemoteMemberId();

  /// Returns [MemberConnectionState] of this `Connection`.
  ///
  /// NOTE: Only works in [P2P mesh] mode and is subject to change.
  ///
  /// Throws a [StateError] if the underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  ///
  /// [P2P mesh]: https://webrtcglossary.com/mesh
  MemberConnectionState? getState();

  /// Sets a callback to be invoked once a state of this `Connection` is
  /// changed.
  ///
  /// NOTE: Only works in [P2P mesh] mode and is subject to change.
  ///
  /// Throws a [StateError] if the underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  ///
  /// [P2P mesh]: https://webrtcglossary.com/mesh
  void onStateChange(void Function(MemberConnectionState) f);

  /// Sets callback, invoked when this `Connection` is closed.
  ///
  /// Throws a [StateError] if the underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  void onClose(void Function() f);

  /// Sets callback, invoked when a new [RemoteMediaTrack] is added to this
  /// `Connection`.
  ///
  /// Throws a [StateError] if the underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f);

  /// Sets a callback to be invoked when the quality estimate for this
  /// [`Connection`] changes.
  ///
  /// If the value provided to the callback is positive then it is a
  /// [ITU-T G.107] R-factor. Negative value means [ICE] connection
  /// failure.
  ///
  /// In [P2P mesh] mode the quality indication describes the single
  /// connection between local peer and remote. In [SFU] mode it describes
  /// the remote member connection to the [SFU] server.
  ///
  /// # R-factor to user satisfaction
  ///
  /// While you can interpret R-factor however you want the recommended
  /// values are defined in [ITU-T G.107] as follows:
  ///
  /// | R-factor |       User satisfaction       |
  /// |----------|-------------------------------|
  /// | >90      | Very satisfied                |
  /// | 80-90    | Satisfied                     |
  /// | 70-80    | Some users dissatisfied       |
  /// | 60-70    | Many users dissatisfied       |
  /// | 50-60    | Nearly all users dissatisfied |
  /// | <50      | All users dissatisfied        |
  ///
  /// Throws a [StateError] if the underlying object has been disposed, e.g.
  /// [free] was called on this [ConnectionHandle], or on a [Jason], or on a
  /// `RoomHandle` that implicitly owns native object behind this
  /// [ConnectionHandle].
  ///
  /// [P2P mesh]: https://webrtcglossary.com/mesh
  /// [SFU]: https://webrtcglossary.com/sfu
  /// [ITU-T G.107]: https://itu.int/rec/T-REC-G.107
  /// [ICE]: https://webrtcglossary.com/ice
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
