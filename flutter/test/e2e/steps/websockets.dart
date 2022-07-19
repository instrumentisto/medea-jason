import 'package:gherkin/gherkin.dart';

import '../world/custom_world.dart';

StepDefinitionGeneric ws_connection_loss = when1<String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) loses WS connection'),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.connection_loss();
  },
);

StepDefinitionGeneric ws_connection_restore = when1<String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) restores WS connection'),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.reconnect();
  },
);

StepDefinitionGeneric connection_is_lost = when1<String, CustomWorld>(
  RegExp(r"(Alice|Bob|Carol)'s WS connection is lost"),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.wait_connection_lost();
  },
);
