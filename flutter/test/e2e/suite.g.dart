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
      '`OnLeave` callback of Control API:',
      <String>[],
      () {
        runScenario(
          'Member closes room',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice\'s room closed by client',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Control API sends `OnLeave` callback with `Disconnected` reason for member Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            '`OnLeave` callback of Control API',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Member\'s Jason object disposed',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disposes Jason object',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Control API sends `OnLeave` callback with `Disconnected` reason for member Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member deleted by Control API',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API removes member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Control API doesn\'t send `OnLeave` callback for member Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Member\'s room deleted by Control API',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API removes the room',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Control API doesn\'t send `OnLeave` callback for member Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            '`OnLeave` callback of Control API',
          ),
        );
      },
    );
  }

  void testFeature1() {
    runFeature(
      'Remote Connection closing:',
      <String>[],
      () {
        runScenario(
          'Connection closes when member is deleted by Control API',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API removes member Bob',
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
          onBefore: () async => onBeforeRunFeature(
            'Remote Connection closing',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Connection closes when other member disposes Jason',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob disposes Jason object',
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
          'Connection closes when other member closes Room',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Bob\'s room closed by client',
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
          onAfter: () async => onAfterRunFeature(
            'Remote Connection closing',
          ),
        );
      },
    );
  }

  void testFeature2() {
    runFeature(
      'Room closing:',
      <String>[],
      () {
        runScenario(
          '`Room.on_close()` fires when `Jason.close_room()` is invoked',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice\'s room closed by client',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s `on_close` room\'s callback fires with `RoomClosed` reason',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: () async => onBeforeRunFeature(
            'Room closing',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          '`Room.on_close()` fires when `Jason.dispose()` is invoked',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice disposes Jason object',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice\'s `on_close` room\'s callback fires with `RoomClosed` reason',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          '`Room.on_close()` fires when member is removed by Control API',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API removes member Alice',
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
          onAfter: null,
        );

        runScenario(
          '`Room.on_close()` fires when room is removed by Control API',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined member Alice',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Control API removes the room',
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
            'Room closing',
          ),
        );
      },
    );
  }

  void testFeature3() {
    runFeature(
      'Room joining:',
      <String>[],
      () {
        runScenario(
          'Member joined',
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
            'Room joining',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Member joined with disabled media',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with member Alice with disabled media publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And joined member Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice joins the room',
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
          'Member without endpoints joined',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with member Alice with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And joined member Bob with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice joins the room',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice doesn\'t receive connection with Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob doesn\'t receive connection with Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Third member joined',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And member Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Carol joins the room',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice receives connection with Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob receives connection with Carol',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            'Room joining',
          ),
        );
      },
    );
  }

  void testFeature4() {
    runFeature(
      '`on_new_connection` callback:',
      <String>[],
      () {
        runScenario(
          'Member joined with enabled media',
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
            '`on_new_connection` callback',
            <String>[],
          ),
          onAfter: null,
        );

        runScenario(
          'Member joined with disabled media',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with member Alice with disabled media publishing',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And joined member Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice joins the room',
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
          'Member joined without WebRTC endpoints',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with member Alice with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And joined member Bob with no WebRTC endpoints',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Alice joins the room',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice doesn\'t receive connection with Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob doesn\'t receive connection with Alice',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: null,
        );

        runScenario(
          'Third member joined',
          <String>[],
          (TestDependencies dependencies) async {
            await runStep(
              'Given room with joined members Alice and Bob',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And member Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'When Carol joins the room',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'Then Alice receives connection with Carol',
              <String>[],
              null,
              dependencies,
            );

            await runStep(
              'And Bob receives connection with Carol',
              <String>[],
              null,
              dependencies,
            );
          },
          onBefore: null,
          onAfter: () async => onAfterRunFeature(
            '`on_new_connection` callback',
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
