import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';

import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';

Future<void> new_given_member( joined,
    first_member_id, second_member_id, third_member_id, webrtc, context) async {

  print('CCCCCCCCCCCCC' + webrtc);
  var member_builder = MyBuilder(first_member_id, true, true);
  var member_builder2 = MyBuilder(second_member_id, true, true);

  await context.world.create_member(member_builder);
  await context.world.create_member(member_builder2);
  print('CCCCCCCCCCCCC0' + webrtc);

  if (joined == 'joined ') {
  print('CCCCCCCCCCCCC1' + webrtc);

    await context.world.join_room(first_member_id);
    await context.world.wait_for_interconnection(first_member_id);
  print('CCCCCCCCCCCCC2' + webrtc);


    await context.world.join_room(second_member_id);
    await context.world.wait_for_interconnection(second_member_id);
  print('CCCCCCCCCCCCC3' + webrtc);

  }
}

StepDefinitionGeneric fillField1 = given3<String, String, String, CustomWorld>(
  RegExp(
      r'room with (joined |)member (Alice|Bob|Carol)( with no (play |publish |)WebRTC endpoints|)'),
  (joined, first_member_id, webrtc, context) async {
    await new_given_member(joined, first_member_id, '', '', webrtc, context);
  },
);

StepDefinitionGeneric fillField2 = given4<String, String, String, String, CustomWorld>(
  RegExp(
      r'room with (joined |)member (Alice|Bob|Carol) and (Alice|Bob|Carol)( with no WebRTC endpoints|)'),
  (joined, first_member_id, second_member_id, webrtc, context) async {
    await new_given_member(joined,
        first_member_id, second_member_id, '', webrtc, context);
  },
);
