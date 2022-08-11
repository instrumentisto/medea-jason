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
      'Remote media disabling:',
      <String>[],
      () {
        runScenario(
          'Remote video track stops when disabled',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s remote device video track from Bob disables',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'Remote media disabling',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Remote audio track stops when disabled',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s remote audio track from Bob disables',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          '`RemoteTrack.on_disabled()` fires when audio is disabled',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_disabled` callback fires 1 time on Alice\'s remote audio track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          '`RemoteTrack.on_disabled()` fires when video is disabled',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_disabled` callback fires 1 time on Alice\'s remote device video track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Remote member disables video',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob disables video and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_disabled` callback fires 1 time on Alice\'s remote device video track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Remote member disables audio',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob disables audio and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_disabled` callback fires 1 time on Alice\'s remote audio track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables audio receiving from concrete `Connection`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled video publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables audio receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s remote audio track from Bob disables',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s audio remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables video receiving from concrete `Connection`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled audio publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables video receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s remote device video track from Bob disables',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s device video remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member enables video receiving from concrete `Connection`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables video receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables video receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member enables audio receiving from concrete `Connection`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables audio receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables audio receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables remote audio via `Room` and enables concrete `Connection`\'s remote audio',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables audio receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s audio remote track from Carol is disabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables remote video via `Room` and enables concrete `Connection`\'s remote video',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled audio publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables video receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s device video remote track from Carol is disabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables remote video from `Connection` and enables remote video via `Room`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and with disabled audio publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables video receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables remote video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables remote audio from `Connection` and enables remote audio via `Room`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled video publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables audio receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables remote audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables all `Connection`s audio receiving and enables it via `Room`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled video publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables audio receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice disables audio receiving from Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables remote audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables all `Connection`s video receiving and enables it via `Room`',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled audio publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables video receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice disables video receiving from Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables remote video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables remote video via `Room` and enables all `Connection`s remote videos',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled audio publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables video receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables video receiving from Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s device video remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s device video remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables remote audio via `Room` and enables all `Connection`s remote audios',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob and Carol with disabled video publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disables remote audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables audio receiving from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice enables audio receiving from Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s audio remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s audio remote track from Carol is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            'Remote media disabling',
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
