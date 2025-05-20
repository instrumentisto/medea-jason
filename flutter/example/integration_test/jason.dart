import 'dart:async';
import 'dart:ffi';
import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/media_device_details.dart';
import 'package:medea_jason/src/native/local_media_track.dart';
import 'package:medea_jason/src/native/platform/rtc_stats.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;
import 'package:device_info_plus/device_info_plus.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    if (Platform.isLinux || Platform.isWindows || Platform.isMacOS) {
      await webrtc.initFfiBridge();
      await webrtc.enableFakeMedia();
    }
    if (Platform.isAndroid) {
      DeviceInfoPlugin deviceInfoPlugin = DeviceInfoPlugin();
      final androidInfo = await deviceInfoPlugin.androidInfo;
      if (androidInfo.version.sdkInt < 25) {
        // Wait for `adb` grant permissions. Emulator running an old SDK won't
        // grant permissions if the UI prompt is already up. So we wait for a
        // little bit to let it run before calling any camera/mic functions
        // that will trigger the prompt.
        await Future.delayed(Duration(seconds: 5));
      }
    }
  });

  testWidgets('MediaManager', (WidgetTester tester) async {
    var jason = await Jason.init();
    var mediaManager = jason.mediaManager();

    var devices = await mediaManager.enumerateDevices();

    var settings = MediaStreamSettings();
    settings.audio(AudioTrackConstraints());
    settings.deviceVideo(DeviceVideoTrackConstraints());
    var tracks = await mediaManager.initLocalTracks(settings);

    expect(devices.length, greaterThanOrEqualTo(2));
    expect(tracks.length, equals(2));

    expect(await tracks[0].state(), webrtc.MediaStreamTrackState.live);
    expect(await tracks[1].state(), webrtc.MediaStreamTrackState.live);
    expect(
      (devices.first as NativeMediaDeviceDetails),
      isNot(equals((devices.last as NativeMediaDeviceDetails))),
    );
    expect(
      (tracks.first as NativeLocalMediaTrack).opaque,
      isNot(equals((tracks.last as NativeLocalMediaTrack).opaque)),
    );

    var video = tracks.where((element) => element.kind() == MediaKind.video);
    expect(video.isNotEmpty, isTrue);
    expect(video.first.mediaSourceKind(), equals(MediaSourceKind.device));

    await tracks.first.free();
    expect(() => tracks.first.kind(), throwsA(isA<StateError>()));

    if (Platform.isIOS) {
      // iOS simulator has no camera.
      return;
    }
    var videoDevice = devices.firstWhere(
      (d) => d.kind() == MediaDeviceKind.videoInput,
    );

    if (!Platform.isAndroid) {
      expect(videoDevice.deviceId(), equals('fake camera id'));
      expect(videoDevice.groupId(), isNull);
      expect(videoDevice.label(), equals('fake camera'));
    }

    videoDevice.free();
  });

  testWidgets('DeviceVideoTrackConstraints', (WidgetTester tester) async {
    var constraints = DeviceVideoTrackConstraints();
    constraints.deviceId('deviceId');
    constraints.exactFacingMode(FacingMode.user);
    constraints.idealFacingMode(FacingMode.right);
    constraints.exactHeight(444);
    constraints.idealHeight(111);
    constraints.heightInRange(55, 66);
    constraints.exactWidth(444);
    constraints.idealWidth(111);
    constraints.widthInRange(55, 66);

    expect(() => constraints.exactHeight(-1), throwsArgumentError);
    expect(() => constraints.idealHeight(-1), throwsArgumentError);
    expect(() => constraints.exactHeight(1 << 32 + 1), throwsArgumentError);
    expect(() => constraints.heightInRange(-1, 200), throwsArgumentError);
    expect(() => constraints.heightInRange(200, -1), throwsArgumentError);

    expect(() => constraints.exactWidth(-1), throwsArgumentError);
    expect(() => constraints.idealWidth(-1), throwsArgumentError);
    expect(() => constraints.exactWidth(1 << 32 + 1), throwsArgumentError);
    expect(() => constraints.widthInRange(-1, 200), throwsArgumentError);
    expect(() => constraints.widthInRange(200, -1), throwsArgumentError);

    var constraints2 = DeviceVideoTrackConstraints();
    var settings = MediaStreamSettings();
    constraints2.deviceId('deviceId');
    settings.deviceVideo(constraints2);
  });

  testWidgets('RoomHandle', (WidgetTester tester) async {
    var jason = await Jason.init();
    var room = jason.initRoom();
    room.onFailedLocalMedia((_) {});
    room.onConnectionLoss((_) {});

    await room.setLocalMediaSettings(MediaStreamSettings(), true, false);
    await room.muteAudio();
    await room.unmuteAudio();
    await room.muteVideo();
    await room.unmuteVideo(MediaSourceKind.display);
    await room.disableVideo(MediaSourceKind.display);
    await room.enableVideo(MediaSourceKind.device);
    await room.disableAudio();
    await room.enableAudio();
    await room.disableRemoteAudio();
    await room.enableRemoteAudio();
    await room.disableRemoteVideo(MediaSourceKind.device);

    dynamic formatExc;
    try {
      await room.join('obviously bad url');
    } catch (e) {
      formatExc = e;
    }
    expect(
      formatExc,
      allOf(
        predicate(
          (e) =>
              e is FormatException &&
              e.message.contains('relative URL without a base'),
        ),
      ),
    );

    jason.closeRoom(room);
    jason.free();
    room.free();
  });

  testWidgets('Primitive arguments Callback validation', (
    WidgetTester widgetTester,
  ) async {
    final intListener = dl.lookupFunction<
      Handle Function(ForeignValue),
      Object Function(ForeignValue)
    >('test_callback_listener_int');
    final stringListener = dl.lookupFunction<
      Handle Function(ForeignValue),
      Object Function(ForeignValue)
    >('test_callback_listener_string');
    final optionalIntListener = dl.lookupFunction<
      Handle Function(ForeignValue),
      Object Function(ForeignValue)
    >('test_callback_listener_optional_int');
    final optionalStringListener = dl.lookupFunction<
      Handle Function(ForeignValue),
      Object Function(ForeignValue)
    >('test_callback_listener_optional_string');

    var intVal = ForeignValue.fromInt(45);
    var stringVal = ForeignValue.fromString('test string');
    var stringVal2 = ForeignValue.fromString('test string');
    var noneVal = ForeignValue.none();

    (intListener(intVal.ref) as Function)(45);
    (stringListener(stringVal.ref) as Function)('test string');
    (optionalIntListener(intVal.ref) as Function)(45);
    (optionalIntListener(noneVal.ref) as Function)(null);
    (optionalStringListener(stringVal2.ref) as Function)('test string');
    (optionalStringListener(noneVal.ref) as Function)(null);

    intVal.free();
    stringVal.free();
    stringVal2.free();
    noneVal.free();
  });

  testWidgets('DartHandle argument Callback validation', (
    WidgetTester widgetTester,
  ) async {
    dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register__test__test_callback_handle_function',
    )(Pointer.fromFunction<Void Function(Handle)>(testObjMutator));
    final dartHandleListener = dl
        .lookupFunction<Handle Function(), Object Function()>(
          'test_callback_listener_dart_handle',
        );

    var obj = TestObj(0);

    (dartHandleListener() as Function)(obj);
    expect(obj.val, equals(45));
  });

  testWidgets('FutureResolver primitives', (WidgetTester widgetTester) async {
    final intResolver = dl
        .lookupFunction<Handle Function(Handle), Object Function(Object)>(
          'test__future_from_dart__int',
        );
    final stringResolver = dl
        .lookupFunction<Handle Function(Handle), Object Function(Object)>(
          'test__future_from_dart__string',
        );

    var intVal =
        await (intResolver(
              () => Future.delayed(const Duration(milliseconds: 500), () async {
                return 45;
              }),
            )
            as Future);
    var stringVal =
        await (stringResolver(
              () => Future.delayed(const Duration(milliseconds: 500), () async {
                return 'test string';
              }),
            )
            as Future);

    expect(intVal as int, equals(45));
    expect(stringVal as String, 'test string');
  });

  testWidgets('GetStats() works', (WidgetTester widgetTester) async {
    final testRtcStatsParse = dl.lookupFunction<
      Uint64 Function(ForeignValue),
      int Function(ForeignValue)
    >('test_rtc_stats_parse');

    var pc1 = await webrtc.PeerConnection.create(
      webrtc.IceTransportType.all,
      [],
    );
    var pc2 = await webrtc.PeerConnection.create(
      webrtc.IceTransportType.all,
      [],
    );

    pc1.onIceCandidate((candidate) async {
      if (!pc2.closed) {
        await pc2.addIceCandidate(candidate);
      }
    });

    pc2.onIceCandidate((candidate) async {
      if (!pc1.closed) {
        await pc1.addIceCandidate(candidate);
      }
    });
    var tVideo = await pc1.addTransceiver(
      webrtc.MediaKind.video,
      webrtc.RtpTransceiverInit(webrtc.TransceiverDirection.sendRecv),
    );
    var tAudio = await pc1.addTransceiver(
      webrtc.MediaKind.audio,
      webrtc.RtpTransceiverInit(webrtc.TransceiverDirection.sendRecv),
    );

    var offer = await pc1.createOffer();
    await pc1.setLocalDescription(offer);
    await pc2.setRemoteDescription(offer);

    var answer = await pc2.createAnswer();
    await pc2.setLocalDescription(answer);
    await pc1.setRemoteDescription(answer);

    var senderStats = await pc1.getStats();
    var receiverStats = await pc2.getStats();

    var senderStatsJson = jsonEncode(
      senderStats.map((stat) => stat.toMap()).toList(),
    );
    var receiverStatsJson = jsonEncode(
      receiverStats.map((stat) => stat.toMap()).toList(),
    );

    var senderStatsString = ForeignValue.fromString(senderStatsJson);
    var receiverStatsString = ForeignValue.fromString(receiverStatsJson);

    expect(
      testRtcStatsParse(senderStatsString.ref),
      equals(senderStats.length),
    );
    expect(
      testRtcStatsParse(receiverStatsString.ref),
      equals(receiverStats.length),
    );

    await pc1.close();
    await pc2.close();
    await tVideo.dispose();
    await tAudio.dispose();

    senderStatsString.free();
    receiverStatsString.free();
  });

  testWidgets('DartHandle argument Future validation', (
    WidgetTester widgetTester,
  ) async {
    dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register__test__future_from_dart_handle_fn',
    )(Pointer.fromFunction<Void Function(Handle)>(testObjMutator));

    final handleResolver = dl
        .lookupFunction<Handle Function(Handle), Object Function(Object)>(
          'test__future_from_dart__handle',
        );

    var testObj = TestObj(0);
    fut() => Future.delayed(const Duration(milliseconds: 500), () async {
      return testObj;
    });
    await (handleResolver(fut) as Future);
    expect(testObj.val, equals(45));
  });

  testWidgets('FutureResolver catches exceptions', (
    WidgetTester widgetTester,
  ) async {
    final futureCatchesException = dl
        .lookupFunction<Handle Function(Handle), Object Function(Object)>(
          'test__future_from_dart__fails',
        );

    fut() => Future.delayed(const Duration(milliseconds: 500), () async {
      throw Exception('Test Exception');
    });
    var res = await (futureCatchesException(fut) as Future);
    expect(res as int, equals(1));
  });

  testWidgets('Panic catcher fires callback and frees Handles', (
    WidgetTester widgetTester,
  ) async {
    final firePanic = dl.lookupFunction<Void Function(), void Function()>(
      'fire_panic',
    );
    final jason = await Jason.init();
    var completer = Completer();
    onPanic((msg) => completer.complete(msg));
    try {
      firePanic();
    } catch (e) {
      var res = await completer.future;
      expect(res as String, contains('panicked at'));
      expect(jason.opaque.isDisposed, true);
      return;
    }
    throw Exception('Exception not fired on panic');
  });

  testWidgets('Enumerate displays', (WidgetTester widgetTester) async {
    var shouldWork = Platform.isLinux || Platform.isMacOS || Platform.isWindows;

    var jason = await Jason.init();
    var media = jason.mediaManager();

    if (!shouldWork) {
      dynamic err;
      try {
        await media.enumerateDisplays();
      } catch (e) {
        err = e;
      }
      expect(err is UnsupportedError, true);
    }
  });

  testWidgets('AudioProcessing with getUserMedia', (WidgetTester tester) async {
    if (Platform.isAndroid || Platform.isIOS) {
      // Audio processing is only supported on desktop.
      return;
    }

    var jason = await Jason.init();
    var mediaManager = jason.mediaManager();

    {
      // all enabled by default
      var settings = MediaStreamSettings();
      settings.audio(AudioTrackConstraints());
      var track = (await mediaManager.initLocalTracks(settings))[0];

      expect(track.isAudioProcessingAvailable(), isTrue);
      expect(await track.isNoiseSuppressionEnabled(), isTrue);
      expect(await track.isHighPassFilterEnabled(), isTrue);
      expect(await track.isAutoGainControlEnabled(), isTrue);
      expect(await track.isEchoCancellationEnabled(), isTrue);
      expect(
        await track.getNoiseSuppressionLevel(),
        NoiseSuppressionLevel.veryHigh,
      );

      await track.free();
    }

    {
      // disable via gum
      var audio = AudioTrackConstraints();
      audio.exactAutoGainControl(false);
      audio.exactEchoCancellation(false);
      audio.exactHighPassFilter(false);
      audio.exactNoiseSuppression(false);
      audio.noiseSuppressionLevel(NoiseSuppressionLevel.low);
      var settings = MediaStreamSettings();
      settings.audio(audio);
      var track = (await mediaManager.initLocalTracks(settings))[0];

      expect(track.isAudioProcessingAvailable(), isTrue);
      expect(await track.isNoiseSuppressionEnabled(), isFalse);
      expect(await track.isHighPassFilterEnabled(), isFalse);
      expect(await track.isAutoGainControlEnabled(), isFalse);
      expect(await track.isEchoCancellationEnabled(), isFalse);
      expect(await track.getNoiseSuppressionLevel(), NoiseSuppressionLevel.low);

      await track.free();
    }

    {
      // new track if different config
      var settings1 = MediaStreamSettings();
      settings1.audio(AudioTrackConstraints());
      var track1 = (await mediaManager.initLocalTracks(settings1))[0];

      expect(await track1.isNoiseSuppressionEnabled(), isTrue);

      var settings2 = MediaStreamSettings();
      var audio = AudioTrackConstraints();
      audio.exactNoiseSuppression(false);
      settings2.audio(audio);
      var track2 = (await mediaManager.initLocalTracks(settings2))[0];

      expect(await track2.isNoiseSuppressionEnabled(), isFalse);
      expect(track1.getTrack().id(), isNot(equals(track2.getTrack().id())));

      await track1.free();
      await track2.free();
    }

    {
      // same track if same config
      var settings1 = MediaStreamSettings();
      var audio1 = AudioTrackConstraints();
      audio1.exactNoiseSuppression(false);
      settings1.audio(audio1);
      var track1 = (await mediaManager.initLocalTracks(settings1))[0];

      expect(await track1.isNoiseSuppressionEnabled(), isFalse);

      var settings2 = MediaStreamSettings();
      var audio2 = AudioTrackConstraints();
      audio2.exactNoiseSuppression(false);
      settings2.audio(audio2);
      var track2 = (await mediaManager.initLocalTracks(settings2))[0];

      expect(await track2.isNoiseSuppressionEnabled(), isFalse);
      expect(track1.getTrack().id(), equals(track2.getTrack().id()));

      await track1.free();
      await track2.free();
    }

    {
      // same track if ideal config changed
      var settings1 = MediaStreamSettings();
      settings1.audio(AudioTrackConstraints());
      var track1 = (await mediaManager.initLocalTracks(settings1))[0];

      expect(await track1.isNoiseSuppressionEnabled(), isTrue);

      var settings2 = MediaStreamSettings();
      var audio = AudioTrackConstraints();
      audio.idealNoiseSuppression(false);
      settings2.audio(audio);
      var track2 = (await mediaManager.initLocalTracks(settings2))[0];

      expect(await track2.isNoiseSuppressionEnabled(), isTrue);
      expect(track1.getTrack().id(), equals(track2.getTrack().id()));

      await track1.free();
      await track2.free();
    }

    jason.free();
    mediaManager.free();
  });

  testWidgets('AudioProcessing in runtime', (WidgetTester tester) async {
    var jason = await Jason.init();
    var mediaManager = jason.mediaManager();

    if (Platform.isAndroid || Platform.isIOS) {
      // Audio processing is only supported on desktop.
      return;
    }

    var settings = MediaStreamSettings();
    settings.audio(AudioTrackConstraints());
    var track = (await mediaManager.initLocalTracks(settings))[0];

    expect(track.isAudioProcessingAvailable(), isTrue);

    expect(await track.isNoiseSuppressionEnabled(), isTrue);
    expect(await track.isHighPassFilterEnabled(), isTrue);
    expect(await track.isAutoGainControlEnabled(), isTrue);
    expect(await track.isEchoCancellationEnabled(), isTrue);
    expect(
      await track.getNoiseSuppressionLevel(),
      NoiseSuppressionLevel.veryHigh,
    );

    await track.setNoiseSuppressionEnabled(false);
    expect(await track.isNoiseSuppressionEnabled(), isFalse);

    await track.setHighPassFilterEnabled(false);
    expect(await track.isHighPassFilterEnabled(), isFalse);

    await track.setAutoGainControlEnabled(false);
    expect(await track.isAutoGainControlEnabled(), isFalse);

    await track.setEchoCancellationEnabled(false);
    expect(await track.isEchoCancellationEnabled(), isFalse);

    await track.setNoiseSuppressionLevel(NoiseSuppressionLevel.high);
    expect(await track.getNoiseSuppressionLevel(), NoiseSuppressionLevel.high);

    await track.setNoiseSuppressionLevel(NoiseSuppressionLevel.moderate);
    expect(
      await track.getNoiseSuppressionLevel(),
      NoiseSuppressionLevel.moderate,
    );

    await track.setNoiseSuppressionLevel(NoiseSuppressionLevel.low);
    expect(await track.getNoiseSuppressionLevel(), NoiseSuppressionLevel.low);

    await track.setNoiseSuppressionEnabled(true);
    expect(await track.isNoiseSuppressionEnabled(), isTrue);

    await track.setHighPassFilterEnabled(true);
    expect(await track.isHighPassFilterEnabled(), isTrue);

    await track.setAutoGainControlEnabled(true);
    expect(await track.isAutoGainControlEnabled(), isTrue);

    await track.setEchoCancellationEnabled(true);
    expect(await track.isEchoCancellationEnabled(), isTrue);

    await track.free();
    jason.free();
    mediaManager.free();
  });
}

class TestObj {
  TestObj(this.val);

  int val;
}

void testObjMutator(Object o) {
  o as TestObj;
  o.val = 45;
}
