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
      'Media muting:',
      <String>[],
      () {
        runScenario(
          '`RemoteTrack.on_muted()` and `RemoteTrack.on_unmuted()` callbacks fire when video is muted/unmuted',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob mutes video and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_muted` callback fires 1 time on Alice\'s remote device video track from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob unmutes video and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_unmuted` callback fires 1 time on Alice\'s remote device video track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'Media muting',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          '`RemoteTrack.on_muted()` and `RemoteTrack.on_unmuted()` callbacks fire when audio is muted/unmuted',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob mutes audio and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_muted` callback fires 1 time on Alice\'s remote audio track from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob unmutes audio and awaits it completes',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then `on_unmuted` callback fires 1 time on Alice\'s remote audio track from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            'Media muting',
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
