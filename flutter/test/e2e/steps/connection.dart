import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';

import '../api/endpoint.dart';
import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/custom_world.dart';

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

// #[then(regex = r"^(\S+) receives connection with (\S+)$")]
// async fn then_member_receives_connection(
//     world: &mut World,
//     id: String,
//     responder_id: String,
// ) {
//     let member = world.get_member(&id).unwrap();
//     member
//         .connections()
//         .wait_for_connection(responder_id.clone())
//         .await
//         .unwrap();
// }
