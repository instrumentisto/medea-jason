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

CustomWorld? old_world;
Future<void> clear_world() async {
  MockMediaDevices.resetGUM();

  if (old_world != null) {
    old_world!.jasons.values.forEach((element) {
      element.free();
    });

    var members = old_world!.members.values;
    for (var member in members) {
      await member.forget_local_tracks();
    }
  }
}

final TestConfigs = FlutterTestConfiguration(
    stepDefinitions: control_api.steps() +
        connection.steps() +
        room.steps() +
        track.steps() +
        media_state.steps() +
        websocket.steps() +
        given.steps(),
    createWorld: (config) => Future.sync(() async {
          await clear_world();
          await webrtc.enableFakeMedia();

          var world = CustomWorld();
          old_world = world;
          await world.control_client
              .create(world.room_id, Room(world.room_id, {}));
          return world;
        }),
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
  executeTestSuite(configuration: TestConfigs, appMainFunction: (_) async {});
}
