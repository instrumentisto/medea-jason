import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';
import 'package:tuple/tuple.dart';

import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';

List<StepDefinitionGeneric> steps() {
  return [
    given_member_gum_will_error,
    then_on_close_fires,
    then_room_failed_local_stream_fires,
    when_jason_object_disposes,
    when_room_closed_by_client,
    when_member_enables_via_local_media_settings,
    when_member_joins_room
  ];
}

StepDefinitionGeneric then_on_close_fires = then2<String, String, CustomWorld>(
  RegExp(r"(\S+)'s `on_close` room's callback fires with `(\S+)` reason$"),
  (id, expect_reason, context) async {
    var reason = await context.world.wait_for_on_close(id);
    expect(reason.reason(), expect_reason);
  },
);

StepDefinitionGeneric when_member_joins_room = when1<String, CustomWorld>(
  RegExp(r'(\S+) joins the room$'),
  (id, context) async {
    await context.world.join_room(id);
  },
);

StepDefinitionGeneric when_room_closed_by_client = when1<String, CustomWorld>(
  RegExp(r"(\S+)'s room closed by client$"),
  (id, context) async {
    context.world.close_room(id);
  },
);

StepDefinitionGeneric when_jason_object_disposes = when1<String, CustomWorld>(
  RegExp(r'(\S+) disposes Jason object$'),
  (id, context) async {
    context.world.jasons[id]!.free();
  },
);

StepDefinitionGeneric given_member_gum_will_error =
    given2<String, String, CustomWorld>(
  RegExp(r"(\S+)'s `getUserMedia\(\)` (audio |video |)errors$"),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    Tuple2<bool, bool> gumSetting;
    if (kind.isEmpty) {
      gumSetting = Tuple2(true, true);
    } else {
      gumSetting = Tuple2(kind.contains('audio'), kind.contains('video'));
    }
    member.get_user_media_mock(gumSetting.item1, gumSetting.item2);
  },
);

StepDefinitionGeneric then_room_failed_local_stream_fires =
    then2<String, String, CustomWorld>(
  RegExp(r"(\S+)'s `Room.on_failed_local_stream\(\)` fires {int} time(:?s)?$"),
  (id, times, context) async {
    var member = context.world.members[id]!;
    var times_parse = int.parse(times);
    await member.wait_failed_local_stream_count(times_parse);
  },
);

StepDefinitionGeneric when_member_enables_via_local_media_settings =
    then2<String, String, CustomWorld>(
  RegExp(r'(\S+) enables (video|audio|video and audio) in local '
      r'media settings'),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    var setting = MediaStreamSettings();
    if (kind.contains('video')) {
      setting.deviceVideo(DeviceVideoTrackConstraints());
    }
    if (kind.contains('audio')) {
      setting.audio(AudioTrackConstraints());
    }
    await member.room.setLocalMediaSettings(setting, true, true);
  },
);
