import 'dart:collection';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;
import 'package:medea_jason_example/control_api.dart';

import 'stuff/api/endpoint.dart';
import 'stuff/api/member.dart';
import 'stuff/api/room.dart';
import 'stuff/control.dart';

const controlDomain = 'http://127.0.0.1:8000';
const baseUrl = 'ws://127.0.0.1:8080/ws/';

class Call {
  final Jason _jason = Jason();
  late RoomHandle _room;
  var client = Client(controlDomain);
  late ControlApi controlApi;
  late Function(webrtc.MediaStreamTrack) _onLocalDeviceTrack;
  late Function(webrtc.MediaStreamTrack) _onLocalDisplayTrack;

  String? audioDeviceId;
  String? videoDeviceId;
  String? videoDisplayId;

  List<LocalMediaTrack> _tracks = [];

  Call() {
    controlApi = ControlApi(client);
    _room = _jason.initRoom();
  }

  Future<void> start(String roomId, String memberId, bool isPublish,
      bool publishVideo, bool publishAudio, bool fakeMedia) async {
    if (fakeMedia) {
      await webrtc.enableFakeMedia();
    }

    var constraints = MediaStreamSettings();

    var id = await _jason.mediaManager().enumerateDevices();

    if (publishVideo) {
      videoDeviceId = id
          .firstWhere((element) => element.kind() == MediaDeviceKind.videoinput)
          .deviceId();
      constraints.deviceVideo(DeviceVideoTrackConstraints());
    }

    if (publishAudio) {
      audioDeviceId = id
          .firstWhere((element) => element.kind() == MediaDeviceKind.audioinput)
          .deviceId();
      constraints.audio(AudioTrackConstraints());
    }

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
      await _room.join(baseUrl + roomId + '/' + memberId + '?token=test');
    } catch (e) {
      rethrow;
    }
  }

  MediaStreamSettings buildConstraints() {
    var constraints = MediaStreamSettings();

    if (videoDeviceId != null) {
      var vSetting = DeviceVideoTrackConstraints();
      vSetting.deviceId(videoDeviceId!);
      constraints.deviceVideo(vSetting);
    }

    if (audioDeviceId != null) {
      var aSetting = AudioTrackConstraints();
      aSetting.deviceId(audioDeviceId!);
      constraints.audio(aSetting);
    }

    return constraints;
  }

  Future<void> toggleScreenShare(DisplayVideoTrackConstraints? display) async {
    _tracks.forEach((element) async {
      await element.free();
    });

    var constraints = buildConstraints();
    if (display != null) {
      constraints.displayVideo(display);
    }

    _tracks = await _jason.mediaManager().initLocalTracks(constraints);
    await _room.setLocalMediaSettings(constraints, true, true);
    _tracks.forEach((track) async {
      if (track.kind() == MediaKind.Video) {
        if (track.mediaSourceKind() == MediaSourceKind.Display) {
          _onLocalDisplayTrack(track.getTrack());
        } else {
          _onLocalDeviceTrack(track.getTrack());
        }
      }
    });
  }

  Future<void> setDevices(
      DeviceVideoTrackConstraints video, AudioTrackConstraints audio) async {
    _tracks.forEach((element) async {
      await element.free();
    });

    var constraints = buildConstraints();
    constraints.deviceVideo(video);
    constraints.audio(audio);

    _tracks = await _jason.mediaManager().initLocalTracks(constraints);
    await _room.setLocalMediaSettings(constraints, true, true);
    _tracks.forEach((track) async {
      if (track.kind() == MediaKind.Video) {
        if (track.mediaSourceKind() == MediaSourceKind.Display) {
          _onLocalDisplayTrack(track.getTrack());
        } else {
          _onLocalDeviceTrack(track.getTrack());
        }
      }
    });
  }

  Future<void> dispose() async {
    _tracks.forEach((t) async => await t.free());
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
    try {
      return _jason.mediaManager().enumerateDisplays();
    } catch (e) {
      print(e);
      return [];
    }
  }

  Future<List<MediaDeviceInfo>> enumerateDevice() async {
    return _jason.mediaManager().enumerateDevices();
  }
}
