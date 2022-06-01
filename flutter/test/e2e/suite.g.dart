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
          'Member disables video during call',
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
              'Then Alice\'s device video remote track from Bob is disabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s audio remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'Media enabling/disabling',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Member disables audio during call',
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
              'Then Alice\'s audio remote track from Bob is disabled',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s device video remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables video before call',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And member Bob with disabled video publishing',
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
              'Then Alice doesn\'t have device video remote track from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s audio remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member disables audio before call',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And member Bob with disabled audio publishing',
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
              'Then Alice doesn\'t have audio remote track from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Alice\'s device video remote track from Bob is enabled',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member enables audio during call',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And member Bob with disabled audio publishing',
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
              'And Bob enables audio and awaits it completes',
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
          'Member enables video during call',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And member Bob with disabled video publishing',
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
              'And Bob enables video and awaits it completes',
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
          'Local track is dropped when video is disabled',
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
              'Then Bob\'s device video local track is stopped',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Local track is dropped when audio is disabled',
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
              'Then Bob\'s audio local track is stopped',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
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
