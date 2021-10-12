import 'package:medea_jason/medea_jason.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/interface/exceptions.dart' as exceptions;

const MEDEA_HOST = '127.0.0.1';
const MEDEA_PORT = 8080;

class Call {
  final Jason _jason = Jason();
  late RoomHandle _room;
  late Function(MediaStream) _onLocalStream;
  List<LocalMediaTrack> _tracks = [];

  Call() {
    _room = _jason.initRoom();
  }

  Future<void> start(String roomId, String memberId) async {
    var constraints = MediaStreamSettings();
    constraints.audio(AudioTrackConstraints());
    constraints.deviceVideo(DeviceVideoTrackConstraints());

    var tracks = await _jason.mediaManager().initLocalTracks(constraints);
    _room.onFailedLocalMedia((e) {
      print('onFailedLocalMedia');
    });
    _room.onConnectionLoss((e) {
      print('onConnectionLoss');
    });
    await _room.setLocalMediaSettings(constraints, false, false);
    _tracks = tracks;

    tracks.forEach((track) async {
      if (track.kind() == MediaKind.Video) {
        var localStream = await createLocalMediaStream('local');
        await localStream.addTrack(track.getTrack());
        _onLocalStream(localStream);
      }
    });

    var url = 'ws://$MEDEA_HOST:$MEDEA_PORT/ws/$roomId/$memberId?token=test';
    try {
      await _room.join(url);
    } catch (e) {
      print((e as exceptions.RpcClientException).cause());
    }
  }

  void dispose() {
    _tracks.forEach((t) => t.free());
    _jason.closeRoom(_room);
  }

  void onLocalStream(Function(MediaStream) f) {
    _onLocalStream = f;
  }

  void onNewRemoteStream(Function(MediaStream) f) {
    _room.onNewConnection((conn) {
      var remoteMemberId = conn.getRemoteMemberId();
      conn.onRemoteTrackAdded((track) async {
        var sysTrack = track.getTrack();
        var remoteStream = await createLocalMediaStream(remoteMemberId);
        await remoteStream.addTrack(sysTrack);
        f(remoteStream);
      });
    });
  }

  Future<void> toggleAudio(bool enabled) async {
    if (enabled) {
      await _room.unmuteAudio();
    } else {
      await _room.muteAudio();
    }
  }

  Future<void> toggleVideo(bool enabled) async {
    if (enabled) {
      await _room.unmuteVideo(MediaSourceKind.Device);
    } else {
      await _room.muteVideo(MediaSourceKind.Device);
    }
  }
}
