import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:flutter_gherkin/flutter_gherkin_with_driver.dart';
import 'package:gherkin/gherkin.dart';

import 'hooks/reset_hook.dart';
import 'world/custom_world.dart';

part 'suite.g.dart';

/// Entry point of E2E testing.
@GherkinTestSuite(featurePaths: ['./test/e2e/features/apply.feature'])
void main() {
  executeTestSuite(
    FlutterTestConfiguration.DEFAULT([])
      ..reporters = [
        StdoutReporter(MessageLevel.error)
          ..setWriteLineFn(print)
          ..setWriteFn(print),
        ProgressReporter()
          ..setWriteLineFn(print)
          ..setWriteFn(print),
        TestRunSummaryReporter()
          ..setWriteLineFn(print)
          ..setWriteFn(print),
        JsonReporter(
          writeReport: (_, __) => Future<void>.value(),
        ),
      ],
    (World world) async {},
  );
}
