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
          onAfter: null,
        );

        runScenario(
          'Disable/enable works fine while disconnect',
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
              'And Alice enables audio',
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
              'Then Bob\'s audio remote track from Alice is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Audio endpoint added while disconnected',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
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
              'And Control API interconnects audio of Alice and Bob',
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
              'Then Alice has audio remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob has audio remote tracks from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Video endpoint added while disconnected',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
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
              'And Control API interconnects video of Alice and Bob',
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
              'Then Alice has video remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob has video remote tracks from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'New endpoint creates new tracks',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
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
              'And Control API interconnects Alice and Bob',
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
              'Then Alice has audio and video remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob has audio and video remote tracks from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'New member joins while disconnected',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And member Bob',
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
              'And Bob joins the room',
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
          onAfter: null,
        );

        runScenario(
          '`Connection.on_close()` fires when other member leaves while disconnected',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
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
              'And Bob\'s room closed by client',
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
              'Then Alice\'s connection with Bob closes',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          '`Connection.on_close()` fires when other member is deleted by Control API while disconnected',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
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
              'And Control API removes member Bob',
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
              'Then Alice\'s connection with Bob closes',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Control API deletes WebRtcPublishEndpoint',
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
              'And Control API deletes Alice\'s publish endpoint',
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
              'Then Bob has 2 stopped remote tracks from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Control API deletes WebRtcPlayEndpoint',
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
              'And Control API deletes Alice\'s play endpoint with Bob',
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
              'Then Alice has 2 stopped remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Control API deletes all endpoints',
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
              'And Control API deletes Alice\'s publish endpoint',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Control API deletes Alice\'s play endpoint with Bob',
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
              'Then Alice\'s connection with Bob closes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob\'s connection with Alice closes',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Create and delete endpoints while disconnected',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
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
              'And Control API starts Alice\'s audio publishing to Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Control API deletes Alice\'s publish endpoint',
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
              'Then Alice doesn\'t have live local tracks',
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
