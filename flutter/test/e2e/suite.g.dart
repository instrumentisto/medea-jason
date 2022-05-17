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
      'Apply method of Control API:',
      <String>[],
      () {
        runScenario(
          'Remove member with `Apply` method',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API removes Alice with `Apply` method',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Bob\'s connection with Alice closes',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'Apply method of Control API',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Interconnect members with `Apply` method',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API interconnects Alice and Bob with `Apply` method',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice receives connection with Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob receives connection with Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            'Apply method of Control API',
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
