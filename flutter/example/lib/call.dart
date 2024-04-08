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

  /// Provides access to Control API of the media server.
  late ControlApi controlApi = ControlApi(client);

  final Jason _jason = Jason();
  late final MediaManagerHandle _mediaManager = _jason.mediaManager();
  late final RoomHandle _room = _jason.initRoom();

  /// Callback for creating/changing a render from a local video device track.
  late Function(webrtc.MediaStreamTrack) _onLocalDeviceTrack;

  /// Callback for creating/changing a render from a local video display track.
  late Function(webrtc.MediaStreamTrack) _onLocalDisplayTrack;

  /// Errors handler.
  Function(String) _onError = (p0) {};

  /// Saved selected audio device ID.
  String? audioDeviceId;

  /// Saved selected video device ID.
  String? videoDeviceId;

  /// Saved selected video device width.
  int? selectedDeviceWidth;

  /// Saved selected video device height.
  int? selectedDeviceHeight;

  /// Saved selected display ID.
  String? videoDisplayId;

  /// Saved selected display width.
  int? selectedDisplayWidth;

  /// Saved selected display height.
  int? selectedDisplayHeight;

  /// Saved selected display framerate.
  int? selectedDisplayFrameRate;

  /// All local track for current member.
  List<LocalMediaTrack> _tracks = [];

  /// Indicator of screen sharing.
  bool screenShare = false;

  /// Starts a call in the specified room.
  Future<void> start(String roomId, String memberId, bool isPublish,
      bool publishVideo, bool publishAudio, bool fakeMedia) async {
    if (fakeMedia) {
      await webrtc.enableFakeMedia();
    }

    var constraints = MediaStreamSettings();

    var devices = await _mediaManager.enumerateDevices();

    if (publishVideo) {
      videoDeviceId = devices
          .firstWhere((element) => element.kind() == MediaDeviceKind.videoInput)
          .deviceId();
      constraints.deviceVideo(DeviceVideoTrackConstraints());
    }

    if (publishAudio) {
      audioDeviceId = devices
          .firstWhere((element) => element.kind() == MediaDeviceKind.audioInput)
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

    for (var track in tracks) {
      if (track.kind() == MediaKind.video) {
        _onLocalDeviceTrack(track.getTrack());
      }
    }

    _room.onLocalTrack((track) {
      _tracks.add(track);
      if (track.kind() == MediaKind.video) {
        if (track.mediaSourceKind() == MediaSourceKind.device) {
          _onLocalDeviceTrack(track.getTrack());
        } else {
          _onLocalDisplayTrack(track.getTrack());
        }
      }
    });

    var getRoom = await client.get(roomId);
    if (getRoom.body == '{}') {
      await _room.join(await createRoom(
          roomId, memberId, isPublish, publishVideo, publishAudio));
      return;
    }

    var getMember = await client.get('$roomId/$memberId');
    if (getMember.body == '{}') {
      await _room.join(await createMember(
          roomId, memberId, isPublish, publishVideo, publishAudio));
      return;
    }

    try {
      await _room.join('$baseUrl$roomId/$memberId?token=test');
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
      if (track.kind() == MediaKind.video) {
        if (track.mediaSourceKind() == MediaSourceKind.display) {
          _onLocalDisplayTrack(track.getTrack());
        } else {
          _onLocalDeviceTrack(track.getTrack());
        }
      }
    }
  }

  /// Clears the media and closes the room.
  Future<void> dispose() async {
    for (var track in _tracks) {
      await track.free();
    }
    _mediaManager.free();
    _jason.closeRoom(_room);
  }

  /// Sets the callback for a new local device track.
  void onLocalDeviceStream(Function(webrtc.MediaStreamTrack) f) {
    _onLocalDeviceTrack = f;
  }

  /// Sets the callback for a new local display track.
  void onLocalDisplayStream(Function(webrtc.MediaStreamTrack) f) {
    _onLocalDisplayTrack = f;
  }

  /// Sets the callback for a new video remote track.
  void onNewRemoteStream(
      Function(RemoteMediaTrack, String, ConnectionHandle) f) {
    _room.onNewConnection((conn) {
      conn.onRemoteTrackAdded((track) async {
        if (track.kind() == MediaKind.audio && !kIsWeb) {
          return;
        }
        f(track, conn.getRemoteMemberId(), conn);
      });
    });
  }

  /// Sets the callback for `onDeviceСhange` events.
  void onDeviceChange(Function() f) {
    _mediaManager.onDeviceChange(f);
  }

  /// Sets the callback for errors handling.
  void onError(Function(String err) f) {
    _onError = f;
  }

  /// Mutes or unmutes audio.
  Future<void> toggleAudio(bool enabled) async {
    if (enabled) {
      await _room.unmuteAudio();
    } else {
      await _room.muteAudio();
    }
  }

  /// Mutes or unmutes video.
  Future<void> toggleVideo(bool enabled) async {
    if (enabled) {
      await _room.unmuteVideo(MediaSourceKind.device);
    } else {
      await _room.muteVideo(MediaSourceKind.device);
    }
  }

  /// Creates a new room. Returns an URL for joining.
  Future<String> createRoom(String roomId, String memberId, bool isPublish,
      bool publishAudio, bool publishVideo) async {
    var pipeline = HashMap<String, Endpoint>();

    if (isPublish) {
      var end = WebRtcPublishEndpoint('publish', P2pMode.Never);
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

  /// Creates a member for the specified room. Returns an URL for joining.
  Future<String> createMember(String roomId, String memberId, bool isPublish,
      bool publishAudio, bool publishVideo) async {
    var pipeline = HashMap<String, Endpoint>();

    if (isPublish) {
      var end = WebRtcPublishEndpoint('publish', P2pMode.Never);
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
        pipeline['play-${m.id}'] = WebRtcPlayEndpoint(
            'play-${m.id}', 'local://$roomId/${m.id}/publish');
      }
    }

    var resp = await client.create(
        '$roomId/$memberId',
        Member(memberId, pipeline, Plain('test'), 'grpc://127.0.0.1:9099',
            'grpc://127.0.0.1:9099'));

    if (isPublish) {
      for (var m in anotherMembers) {
        await client.create('$roomId/${m.id}/play-$memberId',
            WebRtcPlayEndpoint(m.id, 'local://$roomId/$memberId/publish'));
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
          if (track.kind() == MediaKind.video) {
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
          if (track.kind() == MediaKind.video) {
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

  /// Returns a list of the current displays.
  Future<List<MediaDisplayDetails>> enumerateDisplay() async {
    return _mediaManager.enumerateDisplays();
  }

  /// Returns a list of the current devices.
  Future<List<MediaDeviceDetails>> enumerateDevice() async {
    return _mediaManager.enumerateDevices();
  }
}
