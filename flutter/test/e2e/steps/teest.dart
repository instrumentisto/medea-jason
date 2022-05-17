import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';

import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';

Future<void> new_given_member(
      first_member_id,
      second_member_id,
      third_member_id,
      context) async {
        // context.world.members.remove('hide');

        var member_builder = MyBuilder(first_member_id, true, true);
        var member_builder2 = MyBuilder(second_member_id, true, true);


       await context.world.create_member(member_builder);
       await context.world.create_member(member_builder2);
       if (true) {
         await context.world.join_room(first_member_id);
         await context.world.wait_for_interconnection(first_member_id);

        await context.world.join_room(second_member_id);
         await context.world.wait_for_interconnection(second_member_id);
       }

      }

StepDefinitionGeneric fillField1 = given1<String, CustomWorld>(
  RegExp(
      r'room with joined member (Alice|Bob|Carol)'),
  (
      first_member_id,
      context) async {
        await new_given_member(first_member_id, '', '', context);
      },
);

StepDefinitionGeneric fillField2 = given2<String, String, CustomWorld>(
  RegExp(
      r'room with joined member (Alice|Bob|Carol) and (Alice|Bob|Carol)'),
  (
      first_member_id,
      second_member_id,
      context) async {
        await new_given_member(first_member_id, second_member_id, '', context);
      },
);

