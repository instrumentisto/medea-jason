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
      'State synchronization:',
      <String>[],
      () {
        runScenario(
          '`RoomHandle.on_connection_loss()` fires when WS connection lost',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice loses WS connection',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s WS connection is lost',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'State synchronization',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Remote track disable works while disconnect',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice loses WS connection',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob disables audio and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice restores WS connection',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Bob is disabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Local track disable works while disconnect',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice loses WS connection',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice disables audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice restores WS connection',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Bob\'s audio remote track from Alice is disabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            'State synchronization',
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
