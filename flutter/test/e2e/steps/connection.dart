import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';

import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

List<StepDefinitionGeneric> steps() {
  return [
    then_connection_closes,
    then_member_receives_connection,
    when_connection_changes_remote_media_state,
    then_member_doesnt_receive_connection,
  ];
}

StepDefinitionGeneric then_connection_closes =
    then2<String, String, CustomWorld>(
  r"(\S+)'s connection with (\S+) closes$",
  (id, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_close(partner_id);
  },
);

StepDefinitionGeneric then_member_receives_connection =
    then2<String, String, CustomWorld>(
  r'(\S+) receives connection with (\S+)$',
  (id, responder_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(responder_id);
  },
);

StepDefinitionGeneric then_member_doesnt_receive_connection =
    then2<String, String, CustomWorld>(
  r"(\S+) doesn't receive connection with (\S+)$",
  (id, responder_id, context) async {
    var member = context.world.members[id]!;
    expect(member.connection_store.connections[responder_id], null);
  },
);

StepDefinitionGeneric when_connection_changes_remote_media_state =
    when4<String, String, String, String, CustomWorld>(
  r'(\S+) (enables|disables) (audio|video) receiving from (\S+)',
  (id, action, kind, partner_id, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parse_media_kind(kind);

    var connect = member.connection_store.connections[partner_id]!;
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
