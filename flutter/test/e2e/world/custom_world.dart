import 'dart:collection';
import 'dart:convert';

import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' show Response;
import 'package:tuple/tuple.dart';
import 'package:uuid/uuid.dart';

import 'package:medea_jason/medea_jason.dart';
import 'package:medea_jason/src/interface/enums.dart';
import 'package:medea_jason/src/native/platform/transport.dart';
import '../api/callback.dart';
import '../api/endpoint.dart';
import '../api/member.dart' as api;
import '../api/room.dart';
import '../conf.dart';
import '../control.dart';
import 'member.dart';

const bool isSfu = bool.fromEnvironment('SFU', defaultValue: false);

/// [FlutterWidgetTesterWorld] used by all E2E tests.
class CustomWorld extends FlutterWidgetTesterWorld {
  /// ID of the [Room] created for this [FlutterWidgetTesterWorld].
  late String roomId;

  /// Control API client to control the media server.
  late Client controlClient;

  /// All [Member]s created in this [FlutterWidgetTesterWorld].
  var members = HashMap<String, Member>();

  /// Metrics [Response]s collected in this [FlutterWidgetTesterWorld].
  List<Response> metricsResponses = List.empty(growable: true);

  /// All [Jason]s created in this [FlutterWidgetTesterWorld].
  var jasons = HashMap<String, Jason>();

  /// Creates a new fresh [CustomWorld].
  CustomWorld() {
    var uuid = const Uuid();
    roomId = uuid.v4();
    controlClient = Client(controlApiAddr);
  }

  /// Creates a new [Member] from the provided [MemberBuilder].
  ///
  /// [Room] for this [Member] will be created, but joining won't be done.
  Future<void> createMember(MemberBuilder builder) async {
    var pipeline = HashMap<String, Endpoint>();
    var sendState = HashMap<Tuple2<MediaKind, MediaSourceKind>, bool>();
    var recvState = HashMap<Tuple2<MediaKind, MediaSourceKind>, bool>();
    var builderId = builder.id;

    if (builder.isSend) {
      sendState.addAll({
        const Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.audio, MediaSourceKind.device): true
      });
      sendState.addAll({
        const Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.video, MediaSourceKind.device): true
      });
      if (isSfu) {
        sendState.addAll({
          const Tuple2<MediaKind, MediaSourceKind>(
              MediaKind.video, MediaSourceKind.display): true
        });
      }

      pipeline.addAll({
        'publish': WebRtcPublishEndpoint(
            'publish', isSfu ? P2pMode.Never : P2pMode.Always)
      });
    }

    if (builder.isRecv) {
      recvState.addAll({
        const Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.audio, MediaSourceKind.device): true
      });
      recvState.addAll({
        const Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.video, MediaSourceKind.device): true
      });

      if (isSfu) {
        recvState.addAll({
          const Tuple2<MediaKind, MediaSourceKind>(
              MediaKind.video, MediaSourceKind.display): true
        });
      }

      members.forEach((key, value) {
        if (value.isSend) {
          var id = value.id;
          var endpointId = 'play-$id';
          pipeline.addAll({
            endpointId:
                WebRtcPlayEndpoint(endpointId, 'local://$roomId/$id/publish')
          });
        }
      });
    }

    var createMember = api.Member(builderId, pipeline, api.Plain('test'),
        'grpc://127.0.0.1:9099', 'grpc://127.0.0.1:9099');
    await controlClient.create('$roomId/$builderId', createMember);

    if (builder.isSend) {
      var recvEndpoints =
          members.entries.where((element) => element.value.isRecv).map((e) {
        var endpointId = 'play-$builderId';
        var mId = e.value.id;
        var id = '$roomId/$mId/$endpointId';
        var elem = WebRtcPlayEndpoint(
            endpointId, 'local://$roomId/$builderId/publish');
        return Tuple2(id, elem);
      }).toList();

      for (var element in recvEndpoints) {
        await controlClient.create(element.item1, element.item2);
      }
    }

    var jason = Jason();
    var room = jason.initRoom();
    await room.disableVideo(MediaSourceKind.display);

    var member = builder.build(room, sendState, recvState);

    jasons.addAll({member.id: jason});
    members.addAll({member.id: member});
  }

  /// Joins a [Member] with the provided ID to the [Room] created for this
  /// [FlutterWidgetTesterWorld].
  Future<void> joinRoom(String memberId) async {
    await members[memberId]!.joinRoom(roomId);
    MockWebSocket.addMember(memberId);
  }

  /// Closes a [Room] of the provided [Member].
  void closeRoom(String memberId) {
    var jason = jasons[memberId]!;
    var member = members[memberId]!;
    var room = member.room;
    jason.closeRoom(room);
  }

  /// Waits for an [OnLeave] Control API callback for the provided [Member] ID.
  Future<void> waitForOnLeave(String memberId, String reason) async {
    while (true) {
      var callbacks = await getCallbacks(memberId);
      var events = callbacks
          .where((element) =>
              element.fid.contains(memberId) && element.event is OnLeave)
          .map((e) => e.event as OnLeave);
      if (events.isNotEmpty) {
        var ev = events.first;
        expect(ev.reason.name, reason);
        break;
      }
      await Future.delayed(const Duration(milliseconds: 100));
    }
  }

  /// Waits for the [Member]'s [Room] being closed.
  Future<RoomCloseReason> waitForOnClose(String memberId) async {
    var member = members[memberId]!;
    return member.connectionStore.closeReason.future;
  }

  /// Deletes a Control API element of the [WebRtcPublishEndpoint] with the
  /// provided ID.
  Future<void> deletePublishEndpoint(String memberId) async {
    var resp = await controlClient.delete('$roomId/$memberId/publish');
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Deletes a Control API element of the [WebRtcPlayEndpoint] with the
  /// provided ID.
  Future<void> deletePlayEndpoint(
      String memberId, String partnerMemberId) async {
    var playEndpointId = 'play-$partnerMemberId';

    var resp = await controlClient.delete('$roomId/$memberId/$playEndpointId');
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Deletes a Control API element of the [Member] with the provided ID.
  Future<void> deleteMemberElement(String memberId) async {
    var resp = await controlClient.delete('$roomId/$memberId');
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Deletes a Control API element of the [Room] with the provided ID.
  Future<void> deleteRoomElement() async {
    var resp = await controlClient.delete(roomId);
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Returns [Room] created for this [FlutterWidgetTesterWorld].
  Future<Room> getSpec() async {
    var resp = await controlClient.get(roomId);
    var jsonRoom = (json.decode(resp.body) as Map<String, dynamic>)['element'];
    var room = Room.fromJson(jsonRoom);
    return room;
  }

  /// Returns all [CallbackItem]s sent by Control API for this
  /// [FlutterWidgetTesterWorld]'s [Room].
  Future<List<CallbackItem>> getCallbacks(String memberId) async {
    var cbs = await controlClient.callbacks();
    return (json.decode(cbs.body) as List)
        .map((item) => CallbackItem.fromJson(item))
        .where((element) => element.fid.contains(roomId))
        .toList();
  }

  /// Applies provided [Room] spec to the [Room] created for this
  /// [FlutterWidgetTesterWorld].
  Future<void> apply(Room room) async {
    await controlClient.apply(roomId, room);
  }

  /// Waits until a [Member] with the provided ID will connect with his
  /// responders.
  Future<void> waitForInterconnection(String memberId) async {
    var interconnectedMembers = members.entries
        .where((element) =>
            element.value.isJoined &&
            element.value.id != memberId &&
            (element.value.isRecv || element.value.isSend))
        .toList();
    var member = members[memberId]!;

    for (var i = 0; i < interconnectedMembers.length; ++i) {
      var element = interconnectedMembers[i];
      var temp = member.countOfTracksBetweenMembers(element.value);
      var sendCount = temp.item1;
      var recvCount = temp.item2;

      var otherMember = members[element.key]!;
      await member.waitForConnect(element.key);
      await member.waitForTrackCount(element.key, recvCount);

      await otherMember.waitForConnect(member.id);
      await otherMember.waitForTrackCount(member.id, sendCount);

      if (isSfu) {
        if (!otherMember.enabledAudio) {
          for (var track in member
              .connectionStore.remoteTracks[element.key]!.values
              .map((e) => e.last)
              .where((element) => element.kind() == MediaKind.audio)) {
            await member.waitMediaDirectionTrack(
                MediaDirection.recvOnly, track);
          }
        }

        if (!otherMember.enabledVideo) {
          for (var track in member
              .connectionStore.remoteTracks[element.key]!.values
              .map((e) => e.last)
              .where((element) => element.kind() == MediaKind.video)) {
            await member.waitMediaDirectionTrack(
                MediaDirection.recvOnly, track);
          }
        }
        if (!member.enabledAudio) {
          for (var track in otherMember
              .connectionStore.remoteTracks[memberId]!.values
              .map((e) => e.last)
              .where((element) => element.kind() == MediaKind.audio)) {
            await otherMember.waitMediaDirectionTrack(
                MediaDirection.recvOnly, track);
          }
        }

        if (!member.enabledVideo) {
          for (var track in otherMember
              .connectionStore.remoteTracks[memberId]!.values
              .map((e) => e.last)
              .where((element) => element.kind() == MediaKind.video)) {
            await otherMember.waitMediaDirectionTrack(
                MediaDirection.recvOnly, track);
          }
        }
      }

      await Future.delayed(const Duration(milliseconds: 1000));
    }
  }

  /// Creates [WebRtcPublishEndpoint]s and [WebRtcPlayEndpoint]s for the
  /// provided [MembersPair].
  Future<void> interconnectMembers(MembersPair pair) async {
    if (pair.left.publishEndpoint() != null) {
      var publishEndpoint = pair.left.publishEndpoint()!;
      var leftMember = members[pair.left.id]!;
      if (publishEndpoint.audio_settings.publish_policy !=
          PublishPolicy.Disabled) {
        leftMember.updateSendMediaState(MediaKind.audio, null, true);
      }
      if (publishEndpoint.video_settings.publish_policy !=
          PublishPolicy.Disabled) {
        leftMember.updateSendMediaState(MediaKind.video, null, true);
      }
      try {
        await controlClient.create(
            '$roomId/${pair.left.id}/publish', publishEndpoint);
      } catch (e) {
        if (!e
            .toString()
            .contains('Endpoint with provided FID already exists')) {
          rethrow;
        }
      }
    }

    if (pair.right.publishEndpoint() != null) {
      var publishEndpoint = pair.right.publishEndpoint()!;
      var rightMember = members[pair.right.id]!;
      if (publishEndpoint.audio_settings.publish_policy !=
          PublishPolicy.Disabled) {
        rightMember.updateSendMediaState(MediaKind.audio, null, true);
      }
      if (publishEndpoint.video_settings.publish_policy !=
          PublishPolicy.Disabled) {
        rightMember.updateSendMediaState(MediaKind.video, null, true);
      }

      try {
        await controlClient.create(
            '$roomId/${pair.right.id}/publish', publishEndpoint);
      } catch (e) {
        if (!e
            .toString()
            .contains('Endpoint with provided FID already exists')) {
          rethrow;
        }
      }
    }

    if (pair.left.playEndpointFor(roomId, pair.right) != null) {
      var publishEndpoint = pair.left.playEndpointFor(roomId, pair.right)!;
      var leftMember = members[pair.left.id]!;

      await leftMember.updateRecvMediaState(MediaKind.video, null, true);
      await leftMember.updateRecvMediaState(MediaKind.audio, null, true);

      await controlClient.create(
          '$roomId/${pair.left.id}/${publishEndpoint.id}', publishEndpoint);
    }

    if (pair.right.playEndpointFor(roomId, pair.left) != null) {
      var publishEndpoint = pair.right.playEndpointFor(roomId, pair.left)!;
      var rightMember = members[pair.right.id]!;

      await rightMember.updateRecvMediaState(MediaKind.video, null, true);
      await rightMember.updateRecvMediaState(MediaKind.audio, null, true);

      await controlClient.create(
          '$roomId/${pair.right.id}/${publishEndpoint.id}', publishEndpoint);
    }

    {
      var leftMember = members[pair.left.id]!;
      leftMember.isSend = pair.left.isSend();
      leftMember.isRecv = pair.right.recv;
    }

    {
      var rightMember = members[pair.right.id]!;
      rightMember.isSend = pair.right.isSend();
      rightMember.isRecv = pair.right.recv;
    }
  }

  /// Creates [WebRtcPublishEndpoint]s and [WebRtcPlayEndpoint]s for the
  /// provided [MembersPair] using an `Apply` method of Control API.
  Future<void> interconnectMembersViaApply(MembersPair pair) async {
    var spec = await getSpec();
    if (spec.pipeline.containsKey(pair.left.id)) {
      var member = spec.pipeline[pair.left.id]!;
      member.pipeline.addAll({'publish': pair.left.publishEndpoint()!});

      var playEndpoint = pair.left.playEndpointFor(roomId, pair.right)!;
      member.pipeline.addAll({playEndpoint.id: playEndpoint});
    }

    if (spec.pipeline.containsKey(pair.right.id)) {
      var member = spec.pipeline[pair.right.id]!;

      member.pipeline.addAll({'publish': pair.right.publishEndpoint()!});

      var playEndpoint = pair.right.playEndpointFor(roomId, pair.left)!;
      member.pipeline.addAll({playEndpoint.id: playEndpoint});
    }

    await apply(spec);
  }

  /// Waits for [OnJoin] Control API callback for the provided [Member] ID.
  Future<void> waitForOnJoin(String memberId) async {
    while (true) {
      var callbacks = await getCallbacks(memberId);
      var onJoinFound = callbacks
          .where((element) => element.fid.contains(memberId))
          .any((element) => element.event is OnJoin);
      if (onJoinFound) {
        break;
      }
      await Future.delayed(const Duration(milliseconds: 50));
    }
  }
}

/// [Member]s pairing configuration.
///
/// Based on this configuration [FlutterWidgetTesterWorld] can dynamically
/// create [Endpoint]s for this [Member]s.
class MembersPair {
  /// First [PairedMember] in a pair.
  PairedMember left;

  /// Second [PairedMember] in a pair.
  PairedMember right;

  /// Creates a new [MembersPair].
  MembersPair(this.left, this.right);
}

/// [Endpoint]s configuration of a [Member].
class PairedMember {
  /// Unique ID of this [PairedMember].
  String id;

  /// Audio settings to be sent by this [PairedMember].
  AudioSettings? sendAudio;

  /// Video settings to be sent by this [PairedMember].
  VideoSettings? sendVideo;

  /// Indicator whether this is a receiving configuration, rather than
  /// publishing.
  bool recv = false;

  /// Creates a new [PairedMember].
  PairedMember(this.id, this.sendAudio, this.sendVideo, this.recv);

  /// Indicates whether this [PairedMember] should publish media.
  bool isSend() {
    return sendAudio != null || sendVideo != null;
  }

  /// Returns a [WebRtcPublishEndpoint] for this [PairedMember] if publishing is
  /// enabled.
  WebRtcPublishEndpoint? publishEndpoint() {
    WebRtcPublishEndpoint? res;
    if (isSend()) {
      var mode = P2pMode.Always;
      if (isSfu) {
        mode = P2pMode.Never;
      }
      res = WebRtcPublishEndpoint('publish', mode);
      if (sendAudio == null) {
        res.audio_settings = AudioSettings(PublishPolicy.Disabled);
      } else {
        res.audio_settings = sendAudio!;
      }

      if (sendVideo == null) {
        res.video_settings = VideoSettings(PublishPolicy.Disabled);
      } else {
        res.video_settings = sendVideo!;
      }
    }
    return res;
  }

  /// Returns a [WebRtcPlayEndpoint] for this [PairedMember] which will receive
  /// media from the provided [PairedMember] if receiving is enabled.
  WebRtcPlayEndpoint? playEndpointFor(String roomId, PairedMember publisher) {
    if (recv) {
      var res = WebRtcPlayEndpoint(
          'play-${publisher.id}', 'local://$roomId/${publisher.id}/publish');
      return res;
    }
    return null;
  }
}
