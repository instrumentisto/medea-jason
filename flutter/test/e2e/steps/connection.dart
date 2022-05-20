import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';
import 'package:medea_jason/medea_jason.dart';

import '../api/endpoint.dart';
import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

StepDefinitionGeneric then_connection_closes =
    then2<String, String, CustomWorld>(
  r"(Alice|Bob|Carol)'s connection with (Alice|Bob|Carol) closes",
  (id, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_close(partner_id);
  },
);

StepDefinitionGeneric then_member_receives_connection =
    then2<String, String, CustomWorld>(
  r'(Alice|Bob|Carol) receives connection with (Alice|Bob|Carol)',
  (id, responder_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(responder_id);
  },
);

StepDefinitionGeneric then_member_doesnt_receive_connection =
    then2<String, String, CustomWorld>(
  r"(Alice|Bob|Carol) doesn't receive connection with (Alice|Bob|Carol)",
  (id, responder_id, context) async {
    var member = context.world.members[id]!;
    if (member.connection_store.connects[responder_id] != null ) {
      throw 'not null';
    }
  },
);

// #[then(regex = r"^(\S+) doesn't receive connection with (\S+)$")]
// async fn (
//     world: &mut World,
//     id: String,
//     responder_id: String,
// ) {
//     let member = world.get_member(&id).unwrap();
//     assert!(member
//         .connections()
//         .get(responder_id)
//         .await
//         .unwrap()
//         .is_none());
// }

StepDefinitionGeneric when_connection_changes_remote_media_state =
    when4<String, String, String, String, CustomWorld>(
  r'(Alice|Bob|Carol) (enables|disables) (audio|video) receiving from (Alice|Bob|Carol)',
  (id, action, kind, partner_id, context) async {
    var member = context.world.members[id]!;
    var kind_ = parse_media_kind(kind);

    var conn = member.connection_store.connects[partner_id]!;
    if (action == 'enables') {
      if (kind_.item1 == MediaKind.Audio) {
        await conn.enableRemoteAudio();
      } else {
        await conn.enableRemoteVideo();
      }
    } else {
      if (kind_.item1 == MediaKind.Audio) {
        await conn.disableRemoteAudio();
      } else {
        await conn.disableRemoteVideo();
      }
    }
  },
);
