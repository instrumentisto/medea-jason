import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';

import '../world/custom_world.dart';

List<StepDefinitionGeneric> steps() {
  return [
    then_on_close_fires,
    when_jason_object_disposes,
    when_room_closed_by_client,
    when_member_joins_room,
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
    await context.world.wait_for_interconnection(id);
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
