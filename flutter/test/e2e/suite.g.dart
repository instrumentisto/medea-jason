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
      'Create endpoint:',
      <String>[],
      () {
        runScenario(
          'New endpoint creates new connections',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API interconnects Alice and Bob',
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
          onBefore: () async => onBeforeRunFeature(
            'Create endpoint',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Only one member publishes all',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API starts Alice\'s media publishing to Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice doesn\'t have remote tracks from Bob',
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
          'Only one member publishes audio',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API starts Alice\'s audio publishing to Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice doesn\'t have remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob has audio remote track from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Only one member publishes video',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API starts Alice\'s video publishing to Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice doesn\'t have remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob has video remote track from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'WebRtcPlayEndpoint removed and recreated',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API deletes Alice\'s play endpoint with Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Control API starts Bob\'s media publishing to Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice has 2 live remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Endpoints removed and recreated',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API deletes Bob\'s publish endpoint',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Control API starts Bob\'s media publishing to Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice has 2 live remote tracks from Bob',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            'Create endpoint',
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
