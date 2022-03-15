import '/src/util/rust_handles_storage.dart';
import 'remote_media_track.dart';

/// External handler to a `Connection` with a remote `Member`.
abstract class ConnectionHandle implements FreeableHandle {
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
}
