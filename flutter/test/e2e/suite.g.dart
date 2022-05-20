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
      'Enable remote media:',
      <String>[],
      () {
        runScenario(
          '`RemoteMediaTrack.on_enabled()` fires when video is enabled',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Given joined member Alice with disabled video playing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice enables remote video',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_enabled` callback fires 1 time on Alice\'s remote device video track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'Enable remote media',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          '`RemoteMediaTrack.on_enabled()` fires when audio is enabled',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Given joined member Alice with disabled audio playing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice enables remote audio',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_enabled` callback fires 1 time on Alice\'s remote audio track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          '`RemoteMediaTrack.on_enabled()` doesn\'t fire when track is created',
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
              'When Bob joins the room',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_enabled` callback fires 0 times on Alice\'s remote audio track from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And `on_enabled` callback fires 0 times on Bob\'s remote audio track from Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And `on_enabled` callback fires 0 times on Alice\'s remote device video track from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And `on_enabled` callback fires 0 times on Bob\'s remote device video track from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Remote member enables video',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And joined member Bob with disabled video publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob enables video and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_enabled` callback fires 1 time on Alice\'s remote device video track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Remote member enables audio',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And joined member Bob with disabled audio publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob enables audio and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_enabled` callback fires 1 time on Alice\'s remote audio track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            'Enable remote media',
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
