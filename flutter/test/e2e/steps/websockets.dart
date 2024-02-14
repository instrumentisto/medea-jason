import 'package:gherkin/gherkin.dart';

import '../world/custom_world.dart';

List<StepDefinitionGeneric> steps() {
  return [
    wsConnectionLoss,
    wsConnectionRestore,
    connectionIsLost,
  ];
}

StepDefinitionGeneric wsConnectionLoss = when1<String, CustomWorld>(
  RegExp(r'(\S+) loses WS connection$'),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.connectionLoss();
  },
);

StepDefinitionGeneric wsConnectionRestore = when1<String, CustomWorld>(
  RegExp(r'(\S+) restores WS connection$'),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.reconnect();
  },
);

StepDefinitionGeneric connectionIsLost = when1<String, CustomWorld>(
  RegExp(r"(\S+)'s WS connection is lost$"),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.waitConnectionLost();
  },
);
