import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:flutter_gherkin/flutter_gherkin_with_driver.dart';
import 'package:gherkin/gherkin.dart';

import 'conf.dart';
import 'hooks/reset_hook.dart';
import 'steps/connection.dart';
import 'steps/control_api.dart';
import 'steps/teest.dart';
import 'world/custom_world.dart';

part 'suite.g.dart';

var gg = FlutterTestConfiguration()
  ..stepDefinitions = [
    fillField2,
    fillField1,
    then_connection_closes,
    then_member_receives_connection,
    when_control_api_removes_member,
    when_control_api_removes_room,
    when_interconnects_kind,
    when_control_api_removes_member_via_apply,
    when_control_api_interconnects_via_apply,
  ]
  ..hooks = []
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
  ..defaultTimeout = const Duration(seconds: 30)
  ..customStepParameterDefinitions = []
  ..createWorld = (config) => Future.sync(() async {
        var world = CustomWorld();
        await world.control_client.create(world.room_id,
            {'kind': 'Room', 'id': world.room_id, 'pipeline': {}});
        return world;
      });

/// Entry point of E2E testing.
@GherkinTestSuite(featurePaths: ['./test/e2e/features/apply.feature'])
void main() {
  executeTestSuite(
    gg,
    (World world) async {
      init_VAR();
    },
  );
}
