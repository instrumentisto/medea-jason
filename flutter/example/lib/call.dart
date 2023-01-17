import 'dart:collection';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'control_api/entities/endpoint.dart';
import 'control_api/entities/member.dart';
import 'control_api/entities/room.dart';
import 'control_api/client.dart';
import 'control_api/http.dart';

const controlDomain = 'http://127.0.0.1:8000';
const baseUrl = 'ws://127.0.0.1:8080/ws/';

class Call {
  var client = HttpClient(controlDomain);

  /// Provides access to the control api.
  late ControlApi controlApi = ControlApi(client);

  final Jason _jason = Jason();
  late final MediaManagerHandle _mediaManager = _jason.mediaManager();
  late final RoomHandle _room = _jason.initRoom();

  /// Used to create/change a render from a local video device track.
  late Function(webrtc.MediaStreamTrack) _onLocalDeviceTrack;

  /// Used to create/change a render from a local video display track.
  late Function(webrtc.MediaStreamTrack) _onLocalDisplayTrack;

  /// Used to handle error.
  Function(String) _onError = (p0) {};

  /// Saved selected audio device id.
  String? audioDeviceId;

  /// Saved selected video device id.
  String? videoDeviceId;

  /// Saved selected video device width.
  int? selectedDeviceWidth;

  /// Saved selected video device height.
  int? selectedDeviceHeight;

  /// Saved selected display id.
  String? videoDisplayId;

  /// Saved selected display width.
  int? selectedDisplayWidth;

  /// Saved selected display height.
  int? selectedDisplayHeight;

  /// Saved selected display framerate.
  int? selectedDisplayFrameRate;

  /// All local track for current member.
  List<LocalMediaTrack> _tracks = [];

  /// Indicates screen share.
  bool screenShare = false;

  /// Starts a call to the room.
  Future<void> start(String roomId, String memberId, bool isPublish,
      bool publishVideo, bool publishAudio, bool fakeMedia) async {
    if (fakeMedia) {
      await webrtc.enableFakeMedia();
    }

    var constraints = MediaStreamSettings();

    var devices = await _mediaManager.enumerateDevices();

    if (publishVideo) {
      videoDeviceId = devices
          .firstWhere((element) => element.kind() == MediaDeviceKind.videoinput)
          .deviceId();
      constraints.deviceVideo(DeviceVideoTrackConstraints());
    }

    if (publishAudio) {
      audioDeviceId = devices
          .firstWhere((element) => element.kind() == MediaDeviceKind.audioinput)
          .deviceId();
      constraints.audio(AudioTrackConstraints());
    }

    var tracks = await _mediaManager.initLocalTracks(constraints);
    _room.onFailedLocalMedia((e) {
      _onError('onFailedLocalMedia: $e');
    });
    _room.onConnectionLoss((e) {
      _onError('onConnectionLoss: $e');
    });
    await _room.setLocalMediaSettings(constraints, false, false);
    _tracks = tracks;

    tracks.forEach((track) async {
      if (track.kind() == MediaKind.Video) {
        _onLocalDeviceTrack(track.getTrack());
      }
    });

    _room.onLocalTrack((track) {
      _tracks.add(track);
      if (track.kind() == MediaKind.Video) {
        if (track.mediaSourceKind() == MediaSourceKind.Device) {
          _onLocalDeviceTrack(track.getTrack());
        } else {
          _onLocalDisplayTrack(track.getTrack());
        }
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

  /// Sets media tracks according to the passed settings.
  Future<void> setMedia(DeviceVideoTrackConstraints video,
      AudioTrackConstraints audio, DisplayVideoTrackConstraints display) async {
    for (var t in _tracks) {
      await t.free();
    }

    var constraints = MediaStreamSettings();
    constraints.deviceVideo(video);
    constraints.audio(audio);
    if (screenShare) {
      constraints.displayVideo(display);
    }

    _tracks = await _mediaManager.initLocalTracks(constraints);
    await _room.setLocalMediaSettings(constraints, true, true);
    for (var track in _tracks) {
      if (track.kind() == MediaKind.Video) {
        if (track.mediaSourceKind() == MediaSourceKind.Display) {
          _onLocalDisplayTrack(track.getTrack());
        } else {
          _onLocalDeviceTrack(track.getTrack());
        }
      }
    }
  }

  /// Сlears the media and closes the room.
  Future<void> dispose() async {
    _tracks.forEach((t) async => await t.free());
    _mediaManager.free();
    _jason.closeRoom(_room);
  }

  /// Sets a callback for new local device track.
  void onLocalDeviceStream(Function(webrtc.MediaStreamTrack) f) {
    _onLocalDeviceTrack = f;
  }

  /// Sets a callback for new local display track.
  void onLocalDisplayStream(Function(webrtc.MediaStreamTrack) f) {
    _onLocalDisplayTrack = f;
  }

  /// Sets a callback for new video remote track.
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

  /// Sets a callback for the `onDeviceСhange` event.
  void onDeviceChange(Function() f) {
    _mediaManager.onDeviceChange(f);
  }

  /// Sets a callback for for error handling.
  void onError(Function(String err) f) {
    _onError = f;
  }

  /// mute / unmute audio.
  Future<void> toggleAudio(bool enabled) async {
    if (enabled) {
      await _room.unmuteAudio();
    } else {
      await _room.muteAudio();
    }
  }

  /// mute / unmute video.
  Future<void> toggleVideo(bool enabled) async {
    if (enabled) {
      await _room.unmuteVideo(MediaSourceKind.Device);
    } else {
      await _room.muteVideo(MediaSourceKind.Device);
    }
  }

  /// Creates a new room.
  /// Returns url for join.
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

  /// Creates a member for the room.
  /// Returns url for join.
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
      if (m.pipeline.keys.where((element) => element == 'publish').isNotEmpty) {
        pipeline['play-' + m.id] = WebRtcPlayEndpoint(
            'play-' + m.id, 'local://' + roomId + '/' + m.id + '/publish');
      }
    }

    var resp = await client.create(
        roomId + '/' + memberId,
        Member(memberId, pipeline, Plain('test'), 'grpc://127.0.0.1:9099',
            'grpc://127.0.0.1:9099'));

    if (isPublish) {
      for (var m in anotherMembers) {
        await client.create(
            roomId + '/' + m.id + '/' + 'play-' + memberId,
            WebRtcPlayEndpoint(
                m.id, 'local://' + roomId + '/' + memberId + '/publish'));
      }
    }

    return jsonDecode(resp.body)['sids'][memberId];
  }

  /// Sets video send.
  Future<void> setSendVideo(bool enabled, [MediaSourceKind? kind]) async {
    if (enabled) {
      await _room.enableVideo(kind);
    } else {
      for (var track in _tracks) {
        try {
          if (track.kind() == MediaKind.Video) {
            await track.free();
          }
        } catch (_) {}
      }
      await _room.disableVideo(kind);
    }
  }

  /// Sets video reception.
  Future<void> setRecvVideo(bool enabled, [MediaSourceKind? kind]) async {
    if (enabled) {
      await _room.enableRemoteVideo(kind);
    } else {
      await _room.disableRemoteVideo(kind);
    }
  }

  /// Sets audio send.
  Future<void> setSendAudio(bool enabled) async {
    if (enabled) {
      await _room.enableAudio();
    } else {
      for (var track in _tracks) {
        try {
          if (track.kind() == MediaKind.Video) {
            await track.free();
          }
        } catch (_) {}
      }
      await _room.disableAudio();
    }
  }

  /// Sets audio reception.
  Future<void> setRecvAudio(bool enabled) async {
    if (enabled) {
      await _room.enableRemoteAudio();
    } else {
      await _room.disableRemoteAudio();
    }
  }

  /// Returns a list of current displays.
  Future<List<MediaDisplayInfo>> enumerateDisplay() async {
    return _mediaManager.enumerateDisplays();
  }

  /// Returns a list of current devices.
  Future<List<MediaDeviceInfo>> enumerateDevice() async {
    return _mediaManager.enumerateDevices();
  }
}
