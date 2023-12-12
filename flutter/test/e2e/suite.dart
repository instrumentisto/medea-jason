// ignore_for_file: avoid_print

import 'dart:io';

import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:flutter_gherkin/flutter_gherkin_with_driver.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/platform/media_devices.dart';
import 'api/room.dart';
import 'steps/connection.dart' as connection;
import 'steps/control_api.dart' as control_api;
import 'steps/given.dart' as given;
import 'steps/media_state.dart' as media_state;
import 'steps/room.dart' as room;
import 'steps/track.dart' as track;
import 'steps/websockets.dart' as websocket;
import 'world/custom_world.dart';

part 'suite.g.dart';

CustomWorld? oldWorld;
Future<void> clearWorld() async {
  MockMediaDevices.resetGUM();

  if (oldWorld != null) {
    for (var element in oldWorld!.jasons.values) {
      element.free();
    }

    var members = oldWorld!.members.values;
    for (var member in members) {
      await member.forgetLocalTracks();
    }
  }
}

/// [Hook] calling `exit(1)` if any tests in the run have failed, to make sure
/// that test run process exits with an error code.
class ExitOnFailureHook implements Hook {
  bool fail = false;

  @override
  Future<void> onAfterRun(TestConfiguration config) async {
    if (fail) {
      exit(1);
    }
  }

  @override
  Future<void> onAfterScenario(
      TestConfiguration config, String scenario, Iterable<Tag> tags,
      {bool passed = true}) async {
    fail = fail || !passed;
  }

  @override
  Future<void> onAfterScenarioWorldCreated(
      World world, String scenario, Iterable<Tag> tags) async {}

  @override
  Future<void> onAfterStep(
      World world, String step, StepResult stepResult) async {}

  @override
  Future<void> onBeforeRun(TestConfiguration config) async {}

  @override
  Future<void> onBeforeScenario(
      TestConfiguration config, String scenario, Iterable<Tag> tags) async {}

  @override
  Future<void> onBeforeStep(World world, String step) async {}

  @override
  int get priority => 999;
}

final testConfigs = FlutterTestConfiguration(
    stepDefinitions: control_api.steps() +
        connection.steps() +
        room.steps() +
        track.steps() +
        media_state.steps() +
        websocket.steps() +
        given.steps(),
    createWorld: (config) => Future.sync(() async {
          await clearWorld();
          await webrtc.enableFakeMedia();

          var world = CustomWorld();
          oldWorld = world;
          await world.controlClient
              .create(world.roomId, Room(world.roomId, {}));
          return world;
        }),
    defaultTimeout: const Duration(seconds: 30),
    tagExpression: 'not @${isSfu ? 'mesh' : 'sfu'}',
    hooks: [
      ExitOnFailureHook()
    ],
    reporters: [
      StdoutReporter(MessageLevel.verbose)
        ..setWriteLineFn(print)
        ..setWriteFn(print),
      ProgressReporter()
        ..setWriteLineFn(print)
        ..setWriteFn(print),
      TestRunSummaryReporter()
        ..setWriteLineFn(print)
        ..setWriteFn(print),
      FlutterDriverReporter(logInfoMessages: true),
    ]);

@GherkinTestSuite(featurePaths: ['../e2e/tests/features/**'])
void main() {
  executeTestSuite(configuration: testConfigs, appMainFunction: (_) async {});
}
