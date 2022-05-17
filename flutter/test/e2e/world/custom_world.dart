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
import '../conf.dart';
import '../control.dart';
import 'member.dart';
import 'package:uuid/uuid.dart';


/// [FlutterWidgetTesterWorld] storing a custom state during a single test.
class CustomWorld extends FlutterWidgetTesterWorld {
  late String room_id;
  late MyClient control_client;
  late HashMap<String, MyMember> members;
  late HashMap<String, Jason> jasons;

  CustomWorld() {
    var uuid = Uuid();
    room_id = uuid.v4();
    control_client = MyClient('http://127.0.0.1:8000');
    members = HashMap();
    jasons = HashMap();
  }

  Future<void> create_member(MyBuilder builder) async {
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

      //todo
      var gg = WebRtcPublishEndpoint();
      gg.id = 'publish';
      gg.p2p = P2pMode.Always;
      gg.force_relay = false;
      gg.audio_settings = AudioSettings(PublishPolicy.Optional);
      gg.video_settings = VideoSettings(PublishPolicy.Optional);
      var ep = Endpoint();
      ep.data = gg;
      pipeline.addAll({'publish': ep}); // todo
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
          var gg2 = WebRtcPlayEndpoint();
          gg2.id = endpoint_id;
          gg2.src = 'local://$room_id/$id/publish';
          gg2.force_relay = false;
                var ep = Endpoint();
      ep.data = gg2;
          pipeline.addAll({endpoint_id: ep});
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
    jmem.addAll({'kind' : 'Member'});
    await control_client.create('$room_id/$builder_id', jmem);

    if (builder.is_send) {
      var recv_endpoints =
          members.entries.where((element) => element.value.is_recv).map((e) {
        var endpoint_id = 'play-$builder_id';
        var m_id = e.value.id;
        var id = '$room_id/$m_id/$endpoint_id';
        //todo
        var elem = WebRtcPlayEndpoint();
        elem.id = endpoint_id;
        elem.src = 'local://$room_id/$builder_id/publish';
        elem.force_relay = false;
        return Tuple2(id, elem);
      }).toList();


// WebRtcPlayEndpoint(WebRtcPlayEndpoint { id: "play-Bob", src: "local://98528b22-8edd-4121-bb13-e5f670ae9778/Bob/publish", force_relay: false })
// "{\"kind\":\"WebRtcPlayEndpoint\",\"id\":\"play-Bob\",\"src\":\"local://11fc9734-f690-49a5-9436-1f7c2a9f6ce3/Bob/publish\",\"force_relay\":false}"
//  {\"kind\":\"WebRtcPlayEndpoint\",\"id\":\"play-Bob\",\"src\":\"local://d5515fab-5868-4fb0-87f0-c7ee03cf3cb7/Bob/publish\",\"force_relay\":false}
      recv_endpoints.forEach((element) async {
        Map<String, dynamic> gg = {};
        gg.addAll(element.item2.toJson());
        await control_client.create(element.item1, gg);
      });
    }

    var jason = Jason();
    var room = jason.initRoom();

    var member = builder.build(room, send_state, recv_state);


    jasons.addAll({member.id: jason});
    members.addAll({member.id: member});
  }

  // todo error handle
  Future<void> join_room(String member_id) async {
    await members[member_id]!.join_room(room_id);
  }

  Future<void> close_room(String member_id) async {
    var jason = jasons[member_id];
    var member = members[member_id];
    var room = member?.room;
    jason?.closeRoom(room!);
  }

  Future<void> wait_for_on_close(String member_id) async {
    var member = members[member_id];
    //???
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
    room.pipeline =  Room.fromPipe(resp2['element']['pipeline']);
    return room;
  }

// todo
  Future<List<CallbackEvent>> get_callbacks(String member_id) async {
    var cbs = await control_client.callbacks();
    //todo
    return List.empty();
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
    for (var i = 0; i <interconnected_members.length; ++i) {
      var element = interconnected_members[i];
      var temp = member.count_of_tracks_between_members(element.value);
      var send_count = temp.item1;
      var recv_count = temp.item2;

      await member.wait_for_connect(element.key);

      // todo 
      //     conn.tracks_store()
      //         .await?
      //         .wait_for_count(recv_count)
      //         .await?;

      var other_member =  members[element.key]!;
      await other_member.wait_for_connect(member.id);


              //     partner_conn
        //         .tracks_store()
        //         .await?
        //         .wait_for_count(send_count)
        //         .await?;
        // }

    }

  }
        /// Creates `WebRtcPublishEndpoint`s and `WebRtcPlayEndpoint`s for the
    /// provided [`MembersPair`] using an `Apply` method of Control API.
    Future<void> interconnect_members_via_apply(MembersPair pair) async {
      var spec = await get_spec();
      if (spec.pipeline.containsKey(pair.left.id)) {
        var ep = Endpoint();
        ep.data = pair.left.publish_endpoint()!;
        var member = spec.pipeline[pair.left.id]!; // pair.left.id
        member.pipeline.addAll({'publish' : ep});


        var play_endpoint = pair.left.play_endpoint_for(room_id, pair.right)!;
        var ep2 = Endpoint();
        ep2.data = play_endpoint;
        member.pipeline.addAll({play_endpoint.id : ep2});
      }

      if (spec.pipeline.containsKey(pair.right.id)) {
        var member = spec.pipeline[pair.right.id]!; // pair.left.id
        var ep = Endpoint();
        ep.data = pair.right.publish_endpoint()!;
        member.pipeline.addAll({'publish' : ep});
        
        var play_endpoint = pair.right.play_endpoint_for(room_id, pair.right)!;
                var ep2 = Endpoint();
        ep2.data = play_endpoint;

        member.pipeline.addAll({play_endpoint.id : ep2});
      }

      await apply(spec);

    }

    Future<void> wait_for_on_join(String member_id) async {
      while (true) {
        var callbacks = await get_callbacks(member_id);
        // callbacks.where((element) => element.)
      }
    }

    //     /// Waits for `OnJoin` Control API callback for the provided [`Member`] ID.
    // pub async fn wait_for_on_join(&mut self, member_id: String) {
    //     let mut interval = interval(Duration::from_millis(50));
    //     loop {
    //         let callbacks = self.get_callbacks().await;
    //         let on_join_found = callbacks
    //             .into_iter()
    //             .filter(|e| e.fid.contains(&member_id))
    //             .any(|e| matches!(e.event, CallbackEvent::OnJoin(_)));
    //         if on_join_found {
    //             break;
    //         }
    //         interval.tick().await;
    //     }
    // }

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

// impl PairedMember {
//     /// Indicates whether this [`PairedMember`] should publish media.
//     fn is_send(&self) -> bool {
//         self.send_audio.is_some() || self.send_video.is_some()
//     }

  bool is_send() {
    return send_audio != null || send_video != null;
  }
  
  WebRtcPublishEndpoint? publish_endpoint() {
    WebRtcPublishEndpoint? res = null;
    if (is_send()) {
      res = WebRtcPublishEndpoint();
      res.id = 'publish';
      res.p2p = P2pMode.Always;
      res.force_relay = false;
      if(send_audio == null) {
        res.audio_settings = AudioSettings(PublishPolicy.Disabled);
      }
      else {
        res.audio_settings = send_audio!;
      }

      if(send_video == null) {
        res.video_settings = VideoSettings(PublishPolicy.Disabled);
      }
      else {
        res.video_settings = send_video!;
      }

    }
    return res;
  }

  WebRtcPlayEndpoint? play_endpoint_for(String room_id, PairedMember publisher) {
    if (recv) {
      var res = WebRtcPlayEndpoint();
      res.id = 'play-' + publisher.id;
      res.force_relay = false;
      res.src = 'local://$room_id/' + publisher.id + '/publish';
      return res;
    }
    return null;
  }
}