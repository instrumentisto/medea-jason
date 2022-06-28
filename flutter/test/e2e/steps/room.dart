import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';

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
