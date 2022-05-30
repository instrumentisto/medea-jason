import 'dart:async';
import 'dart:collection';
import 'dart:convert';

import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:tuple/tuple.dart';

import '../api/callback.dart';
import '../api/endpoint.dart';
import '../api/member.dart';
import '../api/room.dart';
import '../control.dart';
import 'member.dart';
import 'package:uuid/uuid.dart';


class CustomWorld extends FlutterWidgetTesterWorld {
  late String room_id;
  late MyClient control_client;
  var members = HashMap<String, Member>();
  var jasons = HashMap<String, Jason>();

  CustomWorld() {
    var uuid = Uuid();
    room_id = uuid.v4();
    control_client = MyClient('http://127.0.0.1:8000'); // todo ENV
  }

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

      var publish = WebRtcPublishEndpoint('publish', P2pMode.Always);
      pipeline.addAll({'publish': Endpoint(publish)});
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
          var play = WebRtcPlayEndpoint(endpoint_id, 'local://$room_id/$id/publish');
          pipeline.addAll({endpoint_id: Endpoint(play)});
        }
      });
    }

    //todo
    var mem = ApiMember();
    mem.id = builder_id;
    mem.pipeline = pipeline;
    // mem.credentials = ApiCredentials().toJson();
    mem.credentials = {'plain': 'test'};
    mem.on_join = 'grpc://127.0.0.1:9099';
    mem.on_leave = 'grpc://127.0.0.1:9099';
    var jmem = mem.toJson();
    jmem.addAll({'kind': 'Member'});
    await control_client.create('$room_id/$builder_id', jmem);

    if (builder.is_send) {
      var recv_endpoints =
          members.entries.where((element) => element.value.is_recv).map((e) {
        var endpoint_id = 'play-$builder_id';
        var m_id = e.value.id;
        var id = '$room_id/$m_id/$endpoint_id';
        var elem = WebRtcPlayEndpoint(endpoint_id, 'local://$room_id/$builder_id/publish');
        return Tuple2(id, elem);
      }).toList();

      recv_endpoints.forEach((element) async {
        var gg = <String, dynamic>{};
        gg.addAll(element.item2.toJson());
        await control_client.create(element.item1, gg);
      });
    }

    var jason = Jason();
    var room = jason.initRoom();
    await room.disableVideo(MediaSourceKind.Display);


    var member = builder.build(room, send_state, recv_state);

    jasons.addAll({member.id: jason});
    members.addAll({member.id: member});
  }

  // todo error handle
  Future<void> join_room(String member_id) async {
    await members[member_id]!.join_room(room_id);
  }

  void close_room(String member_id) {
    var jason = jasons[member_id]!;
    var member = members[member_id]!;
    var room = member.room;
    jason.closeRoom(room);
  }

  Future<void> wait_for_on_leave(String member_id, String reason) async {
    while(true) {
      var callbacks = await get_callbacks(member_id);
      var events = callbacks.where((element) => element.fid.contains(member_id) && element.event.data is OnLeave).map((e) => e.event.data as OnLeave);
      if(events.isNotEmpty) {
        var ev = events.first;
        if(ev.reason.name == reason) {
          break;
        } else {
          throw 'not eq';
        }
      }
      await Future.delayed(Duration(milliseconds: 100));
    }
  }

  Future<RoomCloseReason> wait_for_on_close(String member_id) async {
    var member = members[member_id]!;
    return member.close_reason.future;
  }

  Future<void> delete_publish_endpoint(String member_id) async {
    var resp = await control_client.delete('$room_id/$member_id/publish');
  }

  Future<void> delete_play_endpoint(
      String member_id, String partner_member_id) async {
    var play_endpoint_id = 'play-$partner_member_id';

    var resp =
        await control_client.delete('$room_id/$member_id/$play_endpoint_id');
    
    // todo error check
  }

  Future<void> delete_member_element(String member_id) async {
    var resp = await control_client.delete('$room_id/$member_id');
    // todo error check
  }

  Future<void> delete_room_element() async {
    var resp = await control_client.delete(room_id);
    // todo error check
  }

  Future<Room> get_spec() async {
    var resp = await control_client.get(room_id);
    Map<String, dynamic> resp2 = json.decode(resp.body);
    var room = Room();
    room.id = resp2['element']['id'];
    room.pipeline = Room.fromPipe(resp2['element']['pipeline']);
    return room;
  }

// todo
  Future<List<CallbackItem>> get_callbacks(String member_id) async {
    var cbs = await control_client.callbacks();
    return (json.decode(cbs.body) as List)
        .map((data) => CallbackItem.fromJson(data)).where((element) => element.fid.contains(room_id))
        .toList();
  }

  Future<void> apply(Room el) async {
    await control_client.apply(room_id, el.toJson());
  }

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

      await member.wait_for_connect(element.key);
      await member.wait_for_track_count(element.key, recv_count);

      var other_member = members[element.key]!;
      await other_member.wait_for_connect(member.id);
      await other_member.wait_for_track_count(member.id, send_count);
    }
  }






    //     if let Some(play_endpoint) =
    //         pair.right.play_endpoint_for(&self.room_id, &pair.left)
    //     {
    //         let right_member = self.members.get_mut(&pair.right.id).unwrap();

    //         right_member.update_recv_media_state(
    //             Some(MediaKind::Video),
    //             None,
    //             true,
    //         );
    //         right_member.update_recv_media_state(
    //             Some(MediaKind::Audio),
    //             None,
    //             true,
    //         );

    //         self.control_client
    //             .create(
    //                 &control_api_path!(
    //                     self.room_id,
    //                     pair.right.id,
    //                     play_endpoint.id
    //                 ),
    //                 play_endpoint.into(),
    //             )
    //             .await?;
    //     }
  /// Creates `WebRtcPublishEndpoint`s and `WebRtcPlayEndpoint`s for the
  /// provided [`MembersPair`] using an `Apply` method of Control API.
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
      await control_client.create(
          '$room_id/' + pair.left.id + '/publish', publish_endpoint);
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
      await control_client.create(
          '$room_id/' + pair.right.id + '/publish', publish_endpoint);
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

  Future<void> interconnect_members_via_apply(MembersPair pair) async {
    var spec = await get_spec();
    if (spec.pipeline.containsKey(pair.left.id)) {
      var member = spec.pipeline[pair.left.id]!;
      member.pipeline.addAll({'publish': Endpoint(pair.left.publish_endpoint()!)});

      var play_endpoint = pair.left.play_endpoint_for(room_id, pair.right)!;
      member.pipeline.addAll({play_endpoint.id: Endpoint(play_endpoint)});
    }

    if (spec.pipeline.containsKey(pair.right.id)) {
      var member = spec.pipeline[pair.right.id]!;

      member.pipeline.addAll({'publish': Endpoint(pair.right.publish_endpoint()!)});

      var play_endpoint = pair.right.play_endpoint_for(room_id, pair.left)!;
      member.pipeline.addAll({play_endpoint.id: Endpoint(play_endpoint)});
    }

    await apply(spec);
  }

  Future<void> wait_for_on_join(String member_id) async {
    while (true) {
      var callbacks = await get_callbacks(member_id);
      var on_join_found = callbacks
          .where((element) => element.fid.contains(member_id))
          .any((element) {
        return CallbackEvent.toJson(element.event)['type'] == 'OnJoin';
      });
      if (on_join_found) {
        break;
      }
      await Future.delayed(Duration(milliseconds: 50));
    }
  }
}

/// [Session] with some additional info about a [User] it represents.
class CustomUser {
  CustomUser(this.session, this.userId, this.userNum);

  /// [Session] of this [CustomUser].
  final String session;

  /// [UserId] of this [CustomUser].
  final String userId;

  /// [UserNum] of this [CustomUser].
  final String userNum;

  /// ID of a [Chat]-dialog with the authenticated [MyUser].
  String? dialog;
}

class MembersPair {
  PairedMember left;
  PairedMember right;

  MembersPair(this.left, this.right);
}

class PairedMember {
  late String id;
  AudioSettings? send_audio;
  VideoSettings? send_video;
  bool recv = false;

  PairedMember(this.id, this.send_audio, this.send_video, this.recv);

  bool is_send() {
    return send_audio != null || send_video != null;
  }

  WebRtcPublishEndpoint? publish_endpoint() {
    WebRtcPublishEndpoint? res = null;
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

  WebRtcPlayEndpoint? play_endpoint_for(
      String room_id, PairedMember publisher) {
    if (recv) {
      var res = WebRtcPlayEndpoint('play-' + publisher.id, 'local://$room_id/' + publisher.id + '/publish');
      return res;
    }
    return null;
  }
}
