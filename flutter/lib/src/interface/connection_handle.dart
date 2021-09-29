import 'remote_media_track.dart';

abstract class ConnectionHandle {
  String getRemoteMemberId() {
    throw UnimplementedError();
  }

  void onClose(void Function() f) {
    throw UnimplementedError();
  }

  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    throw UnimplementedError();
  }

  void onQualityScoreUpdate(void Function(int) f) {
    throw UnimplementedError();
  }

  void free() {
    throw UnimplementedError();
  }
}
