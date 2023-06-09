import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';
import 'package:tuple/tuple.dart';

import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';

List<StepDefinitionGeneric> steps() {
  return [
    givenMemberGumWillError,
    thenOnCloseFires,
    thenRoomFailedLocalStreamFires,
    whenJasonObjectDisposes,
    whenRoomClosedByClient,
    whenMemberEnablesViaLocalMediaSettings,
    whenMemberJoinsRoom
  ];
}

StepDefinitionGeneric thenOnCloseFires = then2<String, String, CustomWorld>(
  RegExp(r"(\S+)'s `on_close` room's callback fires with `(\S+)` reason$"),
  (id, expectReason, context) async {
    var reason = await context.world.waitForOnClose(id);
    expect(reason.reason(), expectReason);
  },
);

StepDefinitionGeneric whenMemberJoinsRoom = when1<String, CustomWorld>(
  RegExp(r'(\S+) joins the room$'),
  (id, context) async {
    await context.world.joinRoom(id);
  },
);

StepDefinitionGeneric whenRoomClosedByClient = when1<String, CustomWorld>(
  RegExp(r"(\S+)'s room closed by client$"),
  (id, context) async {
    context.world.closeRoom(id);
  },
);

StepDefinitionGeneric whenJasonObjectDisposes = when1<String, CustomWorld>(
  RegExp(r'(\S+) disposes Jason object$'),
  (id, context) async {
    context.world.jasons[id]!.free();
  },
);

StepDefinitionGeneric givenMemberGumWillError =
    given2<String, String, CustomWorld>(
  RegExp(r"(\S+)'s `getUserMedia\(\)` (audio |video |)errors$"),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    Tuple2<bool, bool> gumSetting;
    if (kind.isEmpty) {
      gumSetting = const Tuple2(true, true);
    } else {
      gumSetting = Tuple2(kind.contains('audio'), kind.contains('video'));
    }
    member.getUserMediaMock(gumSetting.item1, gumSetting.item2);
  },
);

StepDefinitionGeneric thenRoomFailedLocalStreamFires =
    then2<String, String, CustomWorld>(
  RegExp(r"(\S+)'s `Room.on_failed_local_stream\(\)` fires {int} time(:?s)?$"),
  (id, times, context) async {
    var member = context.world.members[id]!;
    var timesParse = int.parse(times);
    await member.waitFailedLocalStreamCount(timesParse);
  },
);

StepDefinitionGeneric whenMemberEnablesViaLocalMediaSettings =
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
