import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';

import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

List<StepDefinitionGeneric> steps() {
  return [
    thenConnectionCloses,
    thenMemberReceivesConnection,
    whenConnectionChangesRemoteMediaState,
    thenMemberDoesntReceiveConnection,
  ];
}

StepDefinitionGeneric thenConnectionCloses = then2<String, String, CustomWorld>(
  r"(\S+)'s connection with (\S+) closes$",
  (id, partnerId, context) async {
    throw (42);
    var member = context.world.members[id]!;
    await member.waitForClose(partnerId);
  },
);

StepDefinitionGeneric thenMemberReceivesConnection =
    then2<String, String, CustomWorld>(
  r'(\S+) receives connection with (\S+)$',
  (id, responderId, context) async {
    var member = context.world.members[id]!;
    await member.waitForConnect(responderId);
  },
);

StepDefinitionGeneric thenMemberDoesntReceiveConnection =
    then2<String, String, CustomWorld>(
  r"(\S+) doesn't receive connection with (\S+)$",
  (id, responderId, context) async {
    var member = context.world.members[id]!;
    expect(member.connectionStore.connections[responderId], null);
  },
);

StepDefinitionGeneric whenConnectionChangesRemoteMediaState =
    when4<String, String, String, String, CustomWorld>(
  r'(\S+) (enables|disables) (audio|video) receiving from (\S+)',
  (id, action, kind, partnerId, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parseMediaKind(kind);

    var connect = member.connectionStore.connections[partnerId]!;
    if (action == 'enables') {
      if (parsedKind.item1 == MediaKind.Audio) {
        await connect.enableRemoteAudio();
      } else {
        await connect.enableRemoteVideo();
      }
    } else {
      if (parsedKind.item1 == MediaKind.Audio) {
        await connect.disableRemoteAudio();
      } else {
        await connect.disableRemoteVideo();
      }
    }
  },
);
