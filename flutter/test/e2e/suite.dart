import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:flutter_gherkin/flutter_gherkin_with_driver.dart';
import 'package:gherkin/gherkin.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

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

final TestConfigs = FlutterTestConfiguration()
  ..stepDefinitions = control_api.steps() +
      connection.steps() +
      room.steps() +
      track.steps() +
      media_state.steps() +
      websocket.steps() +
      given.steps()
  ..reporters = [
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
  ]
  ..defaultTimeout = const Duration(seconds: 120)
  ..customStepParameterDefinitions = []
  ..createWorld = (config) => Future.sync(() async {
        await webrtc.enableFakeMedia();
        if (old_world != null) {
          var vl = old_world!.jasons.values.toList();
          for (var i = 0; i < vl.length; ++i) {
            var value = vl[i];
            value.free();
          }
        }
        var world = CustomWorld();
        old_world = world;
        await world.control_client
            .create(world.room_id, Room(world.room_id, {}));
        return world;
      });

// TODO: Enable all tests in #71
// @GherkinTestSuite(featurePaths: [FEATURES_PATH])
@GherkinTestSuite(featurePaths: [
  '../e2e/tests/features/apply.feature',
  '../e2e/tests/features/create_endpoint.feature',
  '../e2e/tests/features/delete_endpoint.feature',
  '../e2e/tests/features/disable_remote_media.feature',
  '../e2e/tests/features/enable_remote_media.feature',
  // // '../e2e/tests/features/get_user_media.feature',
  '../e2e/tests/features/local_tracks_create.feature',
  '../e2e/tests/features/media_direction.feature',
  // '../e2e/tests/features/media_disable.feature',
  '../e2e/tests/features/media_mute.feature',
  '../e2e/tests/features/on_join.feature',
  '../e2e/tests/features/on_leave.feature',
  '../e2e/tests/features/on_new_connection_fires.feature',
  '../e2e/tests/features/remote_connection_close.feature',
  '../e2e/tests/features/room_close.feature',
  '../e2e/tests/features/room_join.feature',
  // '../e2e/tests/features/state_synchronization.feature',
])
Future<void> main() async {
  executeTestSuite(
    TestConfigs,
    (World world) async {},
  );
}
