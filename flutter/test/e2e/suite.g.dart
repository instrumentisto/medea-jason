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
    testFeature1();
    testFeature2();
    testFeature3();
    testFeature4();
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
          onAfter: null,
        );

        runScenario(
          '`OnJoin` callback fires on interconnection with `Apply` method',
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
              'Then Control API sends `OnJoin` callback for member Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          '`Room.on_close()` fires when room is removed with `Apply` method',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
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
              'Then Alice\'s `on_close` room\'s callback fires with `Evicted` reason',
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

  void testFeature1() {
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

  void testFeature2() {
    runFeature(
      'Delete endpoint:',
      <String>[],
      () {
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
              'When Control API deletes Alice\'s publish endpoint',
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
          onBefore: () async => onBeforeRunFeature(
            'Delete endpoint',
            <String>[],
          ),
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
              'When Control API deletes Alice\'s play endpoint with Bob',
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
              'When Control API deletes Alice\'s publish endpoint',
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
          'Publishing continues when WebRtcPlayEndpoint is deleted',
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
              'Then Bob has 2 live remote tracks from Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Publishing continues when partner\'s WebRtcPublishEndpoint is deleted',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API deletes Alice\'s publish endpoint',
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
            'Delete endpoint',
          ),
        );
      },
    );
  }

  void testFeature3() {
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

  void testFeature4() {
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
              'Given room with joined members Alice and Bob and Carol',
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
              'Given room with joined members Alice and Bob and Carol',
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
              'Given room with joined members Alice and Bob and Carol',
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
              'Given room with joined members Alice and Bob and Carol',
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
              'Given room with joined members Alice and Bob and Carol',
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
              'Given room with joined members Alice and Bob and Carol',
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
              'Given room with joined members Alice and Bob and Carol',
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
              'Given room with joined members Alice and Bob and Carol',
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
