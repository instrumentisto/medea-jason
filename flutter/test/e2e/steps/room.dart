import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:gherkin/gherkin.dart';

import '../api/endpoint.dart';
import '../api/room.dart';
import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

StepDefinitionGeneric then_on_close_fires = then2<String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s `on_close` room's callback fires with `(.+)`"),
  (id, expect_reason, context) async {
    var reason = await context.world.wait_for_on_close(id);
    if (reason.reason() != expect_reason) {
      throw 42;
    }
  },
);

StepDefinitionGeneric then_room_failed_local_stream_fires = then2 <String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s `Room.on_failed_local_stream\(\)` fires {int} time(s)"),
  (id, kind, context) async {
    int kind_ = int.parse(kind);  
    var member = context.world.members[id]!;
    var compl = Completer();
    member.room.onFailedLocalMedia((p0) {compl.complete();});
    await compl.future;
  },
);


StepDefinitionGeneric when_member_joins_room = when1 <String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) joins the room'),
  (id, context) async {
    await context.world.join_room(id);
  },
);

StepDefinitionGeneric when_room_closed_by_client = when1 <String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s room closed by client"),
  (id, context) async {
    context.world.close_room(id);
  },
);

StepDefinitionGeneric when_jason_object_disposes = when1 <String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) disposes Jason object'),
  (id, context) async {
    context.world.jasons[id]!.free();
  },
);


//todo
StepDefinitionGeneric given_member_gum_will_error = given2<String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s `getUserMedia\(\)` (audio |video |)errors"),
  (id, kind, context) async {
    throw 'Not found';
    // var member = context.world.members[id]!;
    // var caps = DeviceConstraints();
  
 

    // if (kind == '') {
    // caps.video.mandatory = DeviceVideoConstraints();
    // caps.video.mandatory!.width = 640;
    // caps.video.mandatory!.height = 480;
    // caps.video.mandatory!.fps = 30;
    //   caps.audio.mandatory = AudioConstraints();
    // } else {
    //   if (kind.contains('video')) {
    // caps.video.mandatory = DeviceVideoConstraints();
    // caps.video.mandatory!.width = 640;
    // caps.video.mandatory!.height = 480;
    // caps.video.mandatory!.fps = 30;
    //   }
    //   if (kind.contains('audio')) {
    //     caps.audio.mandatory = AudioConstraints();
    //   }
    // }

    // await getUserMedia(caps); // IDK
  },
);
