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
