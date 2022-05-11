import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:gherkin/gherkin.dart';

/// [Hook] resetting the [Hive] and [Get] states after a test.
class ResetAppHook extends Hook {
  @override
  int get priority => 1;

  @override
  Future<void> onBeforeScenario(
    TestConfiguration config,
    String scenario,
    Iterable<Tag> tags,
  ) async {
    FocusManager.instance.primaryFocus?.unfocus();


    await Future.delayed(Duration.zero);
  }

  @override
  Future<void> onAfterScenario(
    TestConfiguration config,
    String scenario,
    Iterable<Tag> tags,
  ) =>
      onBeforeScenario(config, scenario, tags);
}
