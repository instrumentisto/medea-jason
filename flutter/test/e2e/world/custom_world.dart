import 'dart:collection';

import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:tuple/tuple.dart';

import '../api/endpoint.dart';
import '../api/member.dart';
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
    control_client = MyClient(envVars['CLIENT_API_ADDR']!);
    members = HashMap();
    jasons = HashMap();
  }

  void create_member(MyBuilder builder) async {
    var pipeline = HashMap<String, Object>();
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
      gg.audio_settings; //default
      gg.video_settings; //default
      pipeline.addAll({'publish': gg}); // todo
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
          // todo
          var gg2 = WebRtcPlayEndpoint();
          gg2.id = endpoint_id;
          gg2.src = 'local://$room_id/$id/publish';
          gg2.force_relay = false;
          pipeline.addAll({endpoint_id: gg2});
        }
      });
    }
    //todo
    var mem = ApiMember();
    mem.id = builder_id;
    mem.pipeline = pipeline;
    mem.credentials = ApiCredentials();
    mem.credentials!.type = 'Plain';
    mem.credentials!.data = 'test';
    mem.on_join = 'grpc://127.0.0.1:9099';
    mem.on_leave = 'grpc://127.0.0.1:9099';

    await control_client.create('$room_id/$builder_id', mem);
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

      recv_endpoints.forEach((element) async {
        await control_client.create(element.item1, element.item2);
      });
    }

    var jason = Jason();
    var room = jason.initRoom();
    var member = builder.build(room, send_state, recv_state);
    jasons.addAll({member.id: jason});
    members.addAll({member.id: member});
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
