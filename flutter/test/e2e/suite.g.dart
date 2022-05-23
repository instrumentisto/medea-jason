// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'suite.dart';

// **************************************************************************
// GherkinSuiteTestGenerator
// **************************************************************************

class _CustomGherkinIntegrationTestRunner extends GherkinIntegrationTestRunner {
  _CustomGherkinIntegrationTestRunner(
    TestConfiguration configuration,
    Future<void> Function(World) appMainFunction,
  ) : super(configuration, appMainFunction);

  @override
  void onRun() {
    testFeature0();
  }

  void testFeature0() {
    runFeature(
      'Media enabling/disabling:',
      <String>[],
      () {
        runScenario(
          'Member starts enabling video and instantly disables it',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob\'s `getUserMedia()` request has added latency',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob disables video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob frees all local tracks',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob enables video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob disables video and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Bob is disabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'Media enabling/disabling',
            <String>[],
          ),
          onAfter: () async => onAfterRunFeature(
            'Media enabling/disabling',
          ),
        );
      },
    );
  }
}

void executeTestSuite(
  TestConfiguration configuration,
  Future<void> Function(World) appMainFunction,
) {
  _CustomGherkinIntegrationTestRunner(configuration, appMainFunction).run();
}
