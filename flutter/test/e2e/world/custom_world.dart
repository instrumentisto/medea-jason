import 'dart:collection';
import 'dart:convert';

import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:tuple/tuple.dart';
import 'package:flutter_test/flutter_test.dart';

import '../api/callback.dart';
import '../api/endpoint.dart';
import '../api/member.dart' as api;
import '../api/room.dart';
import '../conf.dart';
import '../control.dart';
import 'member.dart';
import 'package:uuid/uuid.dart';

/// [FlutterWidgetTesterWorld] used by all E2E tests.
class CustomWorld extends FlutterWidgetTesterWorld {
  /// ID of the `Room` created for this [FlutterWidgetTesterWorld].
  late String room_id;

  /// Client of a Medea Control API.
  late Client control_client;

  /// All [Member]s created in this [FlutterWidgetTesterWorld].
  var members = HashMap<String, Member>();

  /// All [Jason]'s created in this [FlutterWidgetTesterWorld].
  var jasons = HashMap<String, Jason>();

  /// Creates a new [CustomWorld].
  CustomWorld() {
    var uuid = Uuid();
    room_id = uuid.v4();
    control_client = Client(CONTROL_API_ADDR);
  }

  /// Creates a new [Member] from the provided [MemberBuilder].
  ///
  /// `Room` for this [Member] will be created, but joining won't be done.
  Future<void> create_member(MemberBuilder builder) async {
    var pipeline = HashMap<String, Endpoint>();
    var send_state = HashMap<Tuple2<MediaKind, MediaSourceKind>, bool>();
    var recv_state = HashMap<Tuple2<MediaKind, MediaSourceKind>, bool>();
    var builder_id = builder.id;

    if (builder.is_send) {
      send_state.addAll({
        Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.Audio, MediaSourceKind.Device): true
      });
      send_state.addAll({
        Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.Video, MediaSourceKind.Device): true
      });

      pipeline.addAll(
          {'publish': WebRtcPublishEndpoint('publish', P2pMode.Always)});
    }

    if (builder.is_recv) {
      recv_state.addAll({
        Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.Audio, MediaSourceKind.Device): true
      });
      recv_state.addAll({
        Tuple2<MediaKind, MediaSourceKind>(
            MediaKind.Video, MediaSourceKind.Device): true
      });

      members.forEach((key, value) {
        if (value.is_send) {
          var id = value.id;
          var endpoint_id = 'play-$id';
          pipeline.addAll({
            endpoint_id:
                WebRtcPlayEndpoint(endpoint_id, 'local://$room_id/$id/publish')
          });
        }
      });
    }

    var create_member = api.Member(builder_id, pipeline, api.Plain('test'),
        'grpc://127.0.0.1:9099', 'grpc://127.0.0.1:9099');
    await control_client.create('$room_id/$builder_id', create_member);

    if (builder.is_send) {
      var recv_endpoints =
          members.entries.where((element) => element.value.is_recv).map((e) {
        var endpoint_id = 'play-$builder_id';
        var m_id = e.value.id;
        var id = '$room_id/$m_id/$endpoint_id';
        var elem = WebRtcPlayEndpoint(
            endpoint_id, 'local://$room_id/$builder_id/publish');
        return Tuple2(id, elem);
      }).toList();

      recv_endpoints.forEach((element) async {
        await control_client.create(element.item1, element.item2);
      });
    }

    var jason = Jason();
    var room = jason.initRoom();
    await room.disableVideo(MediaSourceKind.Display);

    var member = builder.build(room, send_state, recv_state);

    jasons.addAll({member.id: jason});
    members.addAll({member.id: member});
  }

  /// Joins a [Member] with the provided ID to the `Room` created for this
  /// [FlutterWidgetTesterWorld].
  Future<void> join_room(String member_id) async {
    await members[member_id]!.join_room(room_id);
  }

  /// Closes a [`Room`] of the provided [`Member`].
  void close_room(String member_id) {
    var jason = jasons[member_id]!;
    var member = members[member_id]!;
    var room = member.room;
    jason.closeRoom(room);
  }

  /// Waits for `OnLeave` Control API callback for the provided [Member] ID.
  Future<void> wait_for_on_leave(String member_id, String reason) async {
    while (true) {
      var callbacks = await get_callbacks(member_id);
      var events = callbacks
          .where((element) =>
              element.fid.contains(member_id) && element.event is OnLeave)
          .map((e) => e.event as OnLeave);
      if (events.isNotEmpty) {
        var ev = events.first;
        expect(ev.reason.name, reason);
        break;
      }
      await Future.delayed(Duration(milliseconds: 100));
    }
  }

  /// Waits for the [Member] `Room` being closed.
  Future<RoomCloseReason> wait_for_on_close(String member_id) async {
    var member = members[member_id]!;
    return member.connection_store.close_reason.future;
  }

  /// Deletes a Control API element of a [WebRtcPublishEndpoint] with the
  /// provided ID.
  Future<void> delete_publish_endpoint(String member_id) async {
    var resp = await control_client.delete('$room_id/$member_id/publish');
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Deletes a Control API element of a [WebRtcPlayEndpoint] with the
  /// provided ID.
  Future<void> delete_play_endpoint(
      String member_id, String partner_member_id) async {
    var play_endpoint_id = 'play-$partner_member_id';

    var resp =
        await control_client.delete('$room_id/$member_id/$play_endpoint_id');
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Deletes a Control API element of the [Member] with the provided ID.
  Future<void> delete_member_element(String member_id) async {
    var resp = await control_client.delete('$room_id/$member_id');
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Deletes a Control API element of the `Room` with the provided ID.
  Future<void> delete_room_element() async {
    var resp = await control_client.delete(room_id);
    if (resp.statusCode != 200) {
      throw resp.body;
    }
  }

  /// Returns [Room] created for this [FlutterWidgetTesterWorld].
  Future<Room> get_spec() async {
    var resp = await control_client.get(room_id);
    var json_room = (json.decode(resp.body) as Map<String, dynamic>)['element'];
    var room = Room.fromJson(json_room);
    return room;
  }

  /// Returns all [CallbackItem]s sent by Control API for this [FlutterWidgetTesterWorld]'s
  /// `Room`.
  Future<List<CallbackItem>> get_callbacks(String member_id) async {
    var cbs = await control_client.callbacks();
    return (json.decode(cbs.body) as List)
        .map((item) => CallbackItem.fromJson(item))
        .where((element) => element.fid.contains(room_id))
        .toList();
  }

  /// Applies provided [Room] spec to the `Room` created for this
  /// [FlutterWidgetTesterWorld].
  Future<void> apply(Room room) async {
    await control_client.apply(room_id, room);
  }

  /// Waits until a [Member] with the provided ID will connect with his
  /// responders.
  Future<void> wait_for_interconnection(String member_id) async {
    var interconnected_members = members.entries
        .where((element) =>
            element.value.is_joined &&
            element.value.id != member_id &&
            (element.value.is_recv || element.value.is_send))
        .toList();
    var member = members[member_id]!;
    for (var i = 0; i < interconnected_members.length; ++i) {
      var element = interconnected_members[i];
      var temp = member.count_of_tracks_between_members(element.value);
      var send_count = temp.item1;
      var recv_count = temp.item2;

      var other_member = members[element.key]!;

      for(var i = 0; i < other_member.connection_store.local_tracks.length; ++i) {
        var element = other_member.connection_store.local_tracks[i];
        print('M:' +
            other_member.id +
            ' ' +
            element.getTrack().id() +
            ' ' +
            element.kind().name +
            ' ' +
            element.mediaSourceKind().name +
            ' ' +
            (await element.getTrack().state()).toString());
      }

      for(var i = 0; i < member.connection_store.local_tracks.length; ++i) {
        var element = member.connection_store.local_tracks[i];
        print('M:' +
            member.id +
            ' ' +
            element.getTrack().id() +
            ' ' +
            element.kind().name +
            ' ' +
            element.mediaSourceKind().name +
            ' ' +
            (await element.getTrack().state()).toString());
      }

      await member.wait_for_connect(element.key);
      print('42 WAIT TRACK ' + element.key);
      await member.wait_for_track_count(element.key, recv_count);

      await other_member.wait_for_connect(member.id);
      print('42 WAIT TRACK ' + member.id);
      await other_member.wait_for_track_count(member.id, send_count);
    }
  }

  /// Creates [WebRtcPublishEndpoint]s and [WebRtcPlayEndpoint]s for the
  /// provided [MembersPair].
  Future<void> interconnect_members(MembersPair pair) async {
    if (pair.left.publish_endpoint() != null) {
      var publish_endpoint = pair.left.publish_endpoint()!;
      var left_member = members[pair.left.id]!;
      if (publish_endpoint.audio_settings.publish_policy !=
          PublishPolicy.Disabled) {
        left_member.update_send_media_state(MediaKind.Audio, null, true);
      }
      if (publish_endpoint.video_settings.publish_policy !=
          PublishPolicy.Disabled) {
        left_member.update_send_media_state(MediaKind.Video, null, true);
      }
      try {
        await control_client.create(
            '$room_id/' + pair.left.id + '/publish', publish_endpoint);
      } catch (e) {
        if (!e
            .toString()
            .contains('Endpoint with provided FID already exists')) {
          rethrow;
        }
      }
    }

    if (pair.right.publish_endpoint() != null) {
      var publish_endpoint = pair.right.publish_endpoint()!;
      var right_member = members[pair.right.id]!;
      if (publish_endpoint.audio_settings.publish_policy !=
          PublishPolicy.Disabled) {
        right_member.update_send_media_state(MediaKind.Audio, null, true);
      }
      if (publish_endpoint.video_settings.publish_policy !=
          PublishPolicy.Disabled) {
        right_member.update_send_media_state(MediaKind.Video, null, true);
      }

      try {
        await control_client.create(
            '$room_id/' + pair.right.id + '/publish', publish_endpoint);
      } catch (e) {
        if (!e
            .toString()
            .contains('Endpoint with provided FID already exists')) {
          rethrow;
        }
      }
    }

    if (pair.left.play_endpoint_for(room_id, pair.right) != null) {
      var publish_endpoint = pair.left.play_endpoint_for(room_id, pair.right)!;
      var left_member = members[pair.left.id]!;

      await left_member.update_recv_media_state(MediaKind.Video, null, true);
      await left_member.update_recv_media_state(MediaKind.Audio, null, true);

      await control_client.create(
          '$room_id/' + pair.left.id + '/' + publish_endpoint.id,
          publish_endpoint);
    }

    if (pair.right.play_endpoint_for(room_id, pair.left) != null) {
      var publish_endpoint = pair.right.play_endpoint_for(room_id, pair.left)!;
      var right_member = members[pair.right.id]!;

      await right_member.update_recv_media_state(MediaKind.Video, null, true);
      await right_member.update_recv_media_state(MediaKind.Audio, null, true);

      await control_client.create(
          '$room_id/' + pair.right.id + '/' + publish_endpoint.id,
          publish_endpoint);
    }

    {
      var left_member = members[pair.left.id]!;
      left_member.is_send = pair.left.is_send();
      left_member.is_recv = pair.right.recv;
    }

    {
      var right_member = members[pair.right.id]!;
      right_member.is_send = pair.right.is_send();
      right_member.is_recv = pair.right.recv;
    }
  }

  /// Creates [WebRtcPublishEndpoint]s and [WebRtcPlayEndpoint]s for the
  /// provided [MembersPair] using an `Apply` method of Control API.
  Future<void> interconnect_members_via_apply(MembersPair pair) async {
    var spec = await get_spec();
    if (spec.pipeline.containsKey(pair.left.id)) {
      var member = spec.pipeline[pair.left.id]!;
      member.pipeline.addAll({'publish': pair.left.publish_endpoint()!});

      var play_endpoint = pair.left.play_endpoint_for(room_id, pair.right)!;
      member.pipeline.addAll({play_endpoint.id: play_endpoint});
    }

    if (spec.pipeline.containsKey(pair.right.id)) {
      var member = spec.pipeline[pair.right.id]!;

      member.pipeline.addAll({'publish': pair.right.publish_endpoint()!});

      var play_endpoint = pair.right.play_endpoint_for(room_id, pair.left)!;
      member.pipeline.addAll({play_endpoint.id: play_endpoint});
    }

    await apply(spec);
  }

  /// Waits for `OnJoin` Control API callback for the provided [Member] ID.
  Future<void> wait_for_on_join(String member_id) async {
    while (true) {
      var callbacks = await get_callbacks(member_id);
      var on_join_found = callbacks
          .where((element) => element.fid.contains(member_id))
          .any((element) => element.event is OnJoin);
      if (on_join_found) {
        break;
      }
      await Future.delayed(Duration(milliseconds: 50));
    }
  }
}

/// [Member]s pairing configuration.
///
/// Based on this configuration [FlutterWidgetTesterWorld]
/// can dynamically create [Endpoint]s for this [Member]s.
class MembersPair {
  /// First [`PairedMember`] in a pair.
  PairedMember left;

  /// Second [`PairedMember`] in a pair.
  PairedMember right;

  /// Creates a new [MembersPair].
  MembersPair(this.left, this.right);
}

/// [Endpoint]s configuration of a [Member].
class PairedMember {
  /// Unique ID of this [PairedMember].
  String id;

  /// Audio settings to be sent by this [PairedMember].
  AudioSettings? send_audio;

  /// Video settings to be sent by this [PairedMember].
  VideoSettings? send_video;

  /// Indicator whether this is a receiving configuration, rather than
  /// publishing.
  bool recv = false;

  /// Creates a new [PairedMember].
  PairedMember(this.id, this.send_audio, this.send_video, this.recv);

  /// Indicates whether this [PairedMember] should publish media.
  bool is_send() {
    return send_audio != null || send_video != null;
  }

  /// Returns a [WebRtcPublishEndpoint] for this [PairedMember] if
  /// publishing is enabled.
  WebRtcPublishEndpoint? publish_endpoint() {
    WebRtcPublishEndpoint? res;
    if (is_send()) {
      res = WebRtcPublishEndpoint('publish', P2pMode.Always);
      if (send_audio == null) {
        res.audio_settings = AudioSettings(PublishPolicy.Disabled);
      } else {
        res.audio_settings = send_audio!;
      }

      if (send_video == null) {
        res.video_settings = VideoSettings(PublishPolicy.Disabled);
      } else {
        res.video_settings = send_video!;
      }
    }
    return res;
  }

  /// Returns a [WebRtcPlayEndpoint] for this [PairedMember] which
  /// will receive media from the provided [PairedMember] if receiving is
  /// enabled.
  WebRtcPlayEndpoint? play_endpoint_for(
      String room_id, PairedMember publisher) {
    if (recv) {
      var res = WebRtcPlayEndpoint('play-' + publisher.id,
          'local://$room_id/' + publisher.id + '/publish');
      return res;
    }
    return null;
  }
}
