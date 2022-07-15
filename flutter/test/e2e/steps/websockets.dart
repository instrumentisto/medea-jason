import 'package:gherkin/gherkin.dart';

import '../world/custom_world.dart';

StepDefinitionGeneric ws_connection_loss = when1<String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) loses WS connection'),
  (id, context) async {
    await context.world.connection_loss(id);
    var member = context.world.members[id]!;
    await member.wait_connection_lost();
  },
);

StepDefinitionGeneric ws_connection_restore = when1<String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) restores WS connection'),
  (id, context) async {
    await context.world.disable_connection_loss(id);
    var member = context.world.members[id]!;
    await member.rh!.reconnectWithBackoff(100, 2.0, 1000, 5000);
    member.rh = null;
  },
);

StepDefinitionGeneric connection_is_lost = when1<String, CustomWorld>(
  RegExp(r"(Alice|Bob|Carol)'s WS connection is lost"),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.wait_connection_lost();
  },
);
