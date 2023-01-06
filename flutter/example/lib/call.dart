import 'dart:collection';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'stuff/api/endpoint.dart';
import 'stuff/api/member.dart';
import 'stuff/api/room.dart';
import 'stuff/control.dart';

const MEDEA_HOST = '127.0.0.1';
const CONTROL_API_ADDR = 'http://$MEDEA_HOST:8000';
const BASE_URL = 'ws://127.0.0.1:8080/ws/';

class Call {
  final Jason _jason = Jason();
  late RoomHandle _room;
  var client = Client(CONTROL_API_ADDR);
  late Function(webrtc.MediaStreamTrack) _onLocalDeviceTrack;
  late Function(webrtc.MediaStreamTrack) _onLocalDisplayTrack;
  var constraints = MediaStreamSettings();
  List<LocalMediaTrack> _tracks = [];

  Call() {
    _room = _jason.initRoom();
  }

  Future<void> start(String roomId, String memberId, bool isPublish,
      bool publishVideo, bool publishAudio, bool fakeMedia) async {
    if (fakeMedia) {
      await webrtc.enableFakeMedia();
    }

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
        _onLocalDeviceTrack(track.getTrack());
      }
    });

    try {
      await client.get(roomId);
    } catch (e) {
      if (e.toString().contains('Room not found.')) {
        await _room.join(await createRoom(
            roomId, memberId, isPublish, publishVideo, publishAudio));
        return;
      } else {
        rethrow;
      }
    }
    try {
      await client.get(roomId + '/' + memberId);
    } catch (e) {
      await _room.join(await createMember(
          roomId, memberId, isPublish, publishVideo, publishAudio));
      return;
    }
    try {
      await _room.join(BASE_URL + roomId + '/' + memberId + '?token=test');
    } catch (e) {
      rethrow;
    }
  }

  Future<void> toggleScreenShare(
      bool enabled, DisplayVideoTrackConstraints display) async {
    _tracks = _tracks.where((element) {
      if (element.mediaSourceKind() == MediaSourceKind.Display) {
        element.free();
        return false;
      }
      return true;
    }).toList();


    if (enabled) {
      constraints.displayVideo(display);
      _tracks = await _jason.mediaManager().initLocalTracks(constraints);
      _tracks.forEach((track) async {
        if (track.kind() == MediaKind.Video &&
            track.mediaSourceKind() == MediaSourceKind.Display) {
          _onLocalDisplayTrack(track.getTrack());
        }
      });
    }
    await Future.delayed(Duration(seconds: 1));
    await _room.setLocalMediaSettings(constraints, false, false);
  }

  Future<void> setVideoDevices(DeviceVideoTrackConstraints device) async {
    for (var t in _tracks) {
      if (t.mediaSourceKind() == MediaSourceKind.Device &&
          t.kind() == MediaKind.Video) {
        t.free();
      }
    }

    constraints.deviceVideo(device);
    await _room.setLocalMediaSettings(constraints, false, false);
    _tracks = await _jason.mediaManager().initLocalTracks(constraints);
    _tracks.forEach((track) async {
      if (track.kind() == MediaKind.Video &&
          track.mediaSourceKind() == MediaSourceKind.Device) {
        _onLocalDeviceTrack(track.getTrack());
      }
    });
  }

  Future<void> setAudioDevices(AudioTrackConstraints device) async {
    for (var t in _tracks) {
      if (t.mediaSourceKind() == MediaSourceKind.Device &&
          t.kind() == MediaKind.Audio) {
        t.free();
      }
    }
    constraints.audio(device);
    await _room.setLocalMediaSettings(constraints, false, false);
    _tracks = await _jason.mediaManager().initLocalTracks(constraints);
  }

  Future<void> dispose() async {
    _tracks.forEach((t) => t.free());
    _jason.closeRoom(_room);
  }

  void onLocalDeviceStream(Function(webrtc.MediaStreamTrack) f) {
    _onLocalDeviceTrack = f;
  }

  void onLocalDisplayStream(Function(webrtc.MediaStreamTrack) f) {
    _onLocalDisplayTrack = f;
  }

  void onNewRemoteStream(Function(webrtc.MediaStreamTrack, String) f) {
    _room.onNewConnection((conn) {
      conn.onRemoteTrackAdded((track) async {
        if (track.kind() == MediaKind.Audio && !kIsWeb) {
          return;
        }
        var sysTrack = track.getTrack();
        f(sysTrack, conn.getRemoteMemberId());
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

  Future<String> createRoom(String roomId, String memberId, bool isPublish,
      bool publishAudio, bool publishVideo) async {
    var pipeline = HashMap<String, Endpoint>();

    if (isPublish) {
      var end = WebRtcPublishEndpoint('publish', P2pMode.Always);
      end.audio_settings = AudioSettings(
          publishAudio ? PublishPolicy.Optional : PublishPolicy.Disabled);
      end.video_settings = VideoSettings(
          publishVideo ? PublishPolicy.Optional : PublishPolicy.Disabled);
      pipeline.addAll({'publish': end});
    }

    var resp = await client.create(
        roomId,
        Room(roomId, {
          memberId: Member(memberId, pipeline, Plain('test'),
              'grpc://127.0.0.1:9099', 'grpc://127.0.0.1:9099')
        }));
    return jsonDecode(resp.body)['sids'][memberId];
  }

  Future<String> createMember(String roomId, String memberId, bool isPublish,
      bool publishAudio, bool publishVideo) async {
    var pipeline = HashMap<String, Endpoint>();

    if (isPublish) {
      var end = WebRtcPublishEndpoint('publish', P2pMode.Always);
      end.audio_settings = AudioSettings(
          publishAudio ? PublishPolicy.Optional : PublishPolicy.Disabled);
      end.video_settings = VideoSettings(
          publishVideo ? PublishPolicy.Optional : PublishPolicy.Disabled);
      pipeline.addAll({'publish': end});
    }

    var controlRoom =
        Room.fromJson(jsonDecode((await client.get(roomId)).body)['element']);
    var anotherMembers = controlRoom.pipeline.values;

    for (var m in anotherMembers) {
      var memberId = m.id;
      if (m.pipeline.keys.where((element) => element == 'publish').isNotEmpty) {
        pipeline['play-' + memberId] = WebRtcPlayEndpoint('play-' + memberId,
            'local://' + roomId + '/' + memberId + '/publish');
      }
    }

    var resp = await client.create(
        roomId + '/' + memberId,
        Member(memberId, pipeline, Plain('test'), 'grpc://127.0.0.1:9099',
            'grpc://127.0.0.1:9099'));

    if (isPublish) {
      try {
        for (var m in anotherMembers) {
          var id = m.id;
          await client.create(
              roomId + '/' + id + '/' + 'play-' + memberId,
              WebRtcPlayEndpoint(
                  id, 'local://' + roomId + '/' + memberId + '/publish'));
        }
      } catch (e) {
        print(e);
      }
    }

    return jsonDecode(resp.body)['sids'][memberId];
  }

  Future<void> setSendVideo(bool enabled, [MediaSourceKind? kind]) async {
    if (enabled) {
      await _room.enableVideo(kind);
    } else {
      await _room.disableVideo(kind);
    }
  }

  Future<void> setRecvVideo(bool enabled, [MediaSourceKind? kind]) async {
    if (enabled) {
      await _room.enableRemoteVideo(kind);
    } else {
      await _room.disableRemoteVideo(kind);
    }
  }

  Future<void> setSendAudio(bool enabled) async {
    if (enabled) {
      await _room.enableAudio();
    } else {
      await _room.disableAudio();
    }
  }

  Future<void> setRecvAudio(bool enabled) async {
    if (enabled) {
      await _room.enableRemoteAudio();
    } else {
      await _room.disableRemoteAudio();
    }
  }

  Future<List<MediaDisplayInfo>> enumerateDisplay() async {
    return _jason.mediaManager().enumerateDisplays();
  }

  Future<List<MediaDeviceInfo>> enumerateDevice() async {
    return _jason.mediaManager().enumerateDevices();
  }
}
