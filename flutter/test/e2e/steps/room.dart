import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';
import 'package:tuple/tuple.dart';

import '../world/custom_world.dart';

StepDefinitionGeneric then_on_close_fires = then2<String, String, CustomWorld>(
  RegExp(r"(Alice|Bob|Carol)'s `on_close` room's callback fires with `(.+)`"),
  (id, expect_reason, context) async {
    var reason = await context.world.wait_for_on_close(id);
    expect(reason.reason(), expect_reason);
  },
);

StepDefinitionGeneric when_member_joins_room = when1<String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) joins the room'),
  (id, context) async {
    await context.world.join_room(id);
    await context.world.wait_for_interconnection(id);
  },
);

StepDefinitionGeneric when_room_closed_by_client = when1<String, CustomWorld>(
  RegExp(r"(Alice|Bob|Carol)'s room closed by client"),
  (id, context) async {
    context.world.close_room(id);
  },
);

StepDefinitionGeneric when_jason_object_disposes = when1<String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) disposes Jason object'),
  (id, context) async {
    context.world.jasons[id]!.free();
  },
);

StepDefinitionGeneric given_member_gum_will_error = given2 <String, String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) `getUserMedia\(\)` (audio |video |)errors'),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    Tuple2<bool,bool> gumSetting;
    if (kind.isEmpty) {
      gumSetting = Tuple2(true,true);
    } else {
      gumSetting = Tuple2(kind.contains('audio'), kind.contains('video'));
    }
    member.get_user_media_mock(gumSetting.item1, gumSetting.item2);
  },
);

