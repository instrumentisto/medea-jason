import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/result.dart';
import 'package:medea_jason/src/native/room_handle.dart';
import 'package:medea_jason/src/native/media_device_info.dart';
import 'package:medea_jason/src/native/local_media_track.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await webrtc.enableFakeMedia();
  });

  testWidgets('MediaManager', (WidgetTester tester) async {
    var jason = Jason();
    var mediaManager = jason.mediaManager();

    var devices = await mediaManager.enumerateDevices();

    var settings = MediaStreamSettings();
    settings.audio(AudioTrackConstraints());
    settings.deviceVideo(DeviceVideoTrackConstraints());
    var tracks = await mediaManager.initLocalTracks(settings);

    expect(devices.length, equals(2));
    expect(tracks.length, equals(2));

    expect((devices.first as NativeMediaDeviceInfo).opaque,
        isNot(equals((devices.last as NativeMediaDeviceInfo).opaque)));
    expect((tracks.first as NativeLocalMediaTrack).opaque,
        isNot(equals((tracks.last as NativeLocalMediaTrack).opaque)));

    expect(devices.first.deviceId(), equals('fake camera id'));
    expect(devices.first.groupId(), isNull);
    expect(devices.first.kind(), equals(MediaDeviceKind.videoinput));
    expect(devices.first.label(), equals('fake camera'));

    devices.first.free();
    expect(() => devices.first.label(), throwsA(isA<FfiException>()));

    expect(tracks.first.kind(), equals(MediaKind.Video));
    // expect(tracks.first.mediaSourceKind(), equals(MediaSourceKind.Display));

    // tracks.first.free();
    // expect(() => tracks.first.kind(), throwsA(isA<FfiException>()));
  });

  testWidgets('DeviceVideoTrackConstraints', (WidgetTester tester) async {
    var constraints = DeviceVideoTrackConstraints();
    constraints.deviceId('deviceId');
    constraints.exactFacingMode(FacingMode.User);
    constraints.idealFacingMode(FacingMode.Right);
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

    constraints.free();
    expect(
        () => constraints.deviceId('deviceId'), throwsA(isA<FfiException>()));

    var constraints2 = DeviceVideoTrackConstraints();
    var settings = MediaStreamSettings();
    constraints2.deviceId('deviceId');
    settings.deviceVideo(constraints2);
    expect(
        () => constraints2.deviceId('deviceId'), throwsA(isA<FfiException>()));
  });

  // testWidgets('ConnectionHandle', (WidgetTester tester) async {
  //   var jason = Jason();
  //   var room = jason.initRoom();

  //   var connFut = Completer<ConnectionHandle>();
  //   room.onNewConnection((conn) {
  //     connFut.complete(conn);
  //   });
  //   var conn = await connFut.future;

  //   expect(
  //       () => conn.getRemoteMemberId(),
  //       throwsA(predicate((e) =>
  //           e is StateError &&
  //           e.message == '`ConnectionHandle` is in detached state')));
  //   var allFired = List<Completer>.generate(2, (_) => Completer());
  //   conn.onQualityScoreUpdate((score) {
  //     allFired[0].complete(score);
  //   });
  //   conn.onClose(() {
  //     allFired[1].complete();
  //   });

  //   var res = await Future.wait(allFired.map((e) => e.future))
  //       .timeout(Duration(seconds: 1));
  //   expect(res[0], 4);
  // });
  //
  testWidgets('RoomHandle', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();

    // await room.join('wss://example.com/room/Alice?token=777');
    await room.setLocalMediaSettings(MediaStreamSettings(), true, false);
    await room.muteAudio();
    await room.unmuteAudio();
    await room.muteVideo();
    await room.unmuteVideo(MediaSourceKind.Display);
    await room.disableVideo(MediaSourceKind.Display);
    await room.enableVideo(MediaSourceKind.Device);
    await room.disableAudio();
    await room.enableAudio();
    await room.disableRemoteAudio();
    await room.enableRemoteAudio();
    await room.disableRemoteVideo(MediaSourceKind.Device);

    var stateErr;
    try {
      await room.enableRemoteVideo();
    } catch (e) {
      stateErr = e;
    }
    expect(
        stateErr,
        allOf(predicate((e) =>
            e is StateError &&
            e.message == 'RoomHandle is in detached state')));

    var formatExc;
    try {
      await room.join('obviously bad url');
    } catch (e) {
      formatExc = e;
    }
    expect(
        formatExc,
        allOf(predicate((e) =>
            e is FormatException &&
            e.message.contains('relative URL without a base'))));

    var localMediaErr = Completer<Object>();
    room.onFailedLocalMedia((err) {
      localMediaErr.complete(err);
    });
    var err = await localMediaErr.future;
    expect(
        err,
        predicate((e) =>
            e is InternalException &&
            e.message() ==
                'SimpleTracksRequest should have at least one track' &&
            e.trace().contains('at src')));
  });
  //
  // testWidgets('ReconnectHandle', (WidgetTester tester) async {
  //   var jason = Jason();
  //   var room = jason.initRoom();
  //
  //   var handleFut = Completer<ReconnectHandle>();
  //   room.onConnectionLoss((reconnectHandle) {
  //     handleFut.complete(reconnectHandle);
  //   });
  //   var handle = await handleFut.future;
  //
  //   await handle.reconnectWithDelay(155);
  //   await handle.reconnectWithBackoff(1, 2, 3);
  //
  //   var exception;
  //   try {
  //     await handle.reconnectWithDelay(-1);
  //   } catch (e) {
  //     exception = e;
  //   }
  //   expect(exception, isArgumentError);
  //
  //   var exception2;
  //   try {
  //     await handle.reconnectWithBackoff(-1, 2, 3, 145);
  //   } catch (e) {
  //     exception2 = e;
  //   }
  //   expect(exception2, isArgumentError);
  //
  //   var exception3;
  //   try {
  //     await handle.reconnectWithBackoff(1, 2, -3, 333);
  //   } catch (e) {
  //     exception3 = e;
  //   }
  //   expect(exception3, isArgumentError);
  //   var argumentError = exception3 as ArgumentError;
  //   expect(argumentError.invalidValue, equals(-3));
  //   expect(argumentError.name, 'maxDelay');
  //   expect(argumentError.message, 'Expected u32');
  //
  //   var exception4;
  //   try {
  //     await handle.reconnectWithBackoff(1, 2, 3, -4);
  //   } catch (e) {
  //     exception4 = e;
  //   }
  //   expect(exception4, isArgumentError);
  //   var argumentError2 = exception4 as ArgumentError;
  //   expect(argumentError2.invalidValue, equals(-4));
  //   expect(argumentError2.name, 'maxElapsedTimeMs');
  //   expect(argumentError2.message, 'Expected u32');
  // });
  //
  // testWidgets('Complex arguments validation', (WidgetTester tester) async {
  //   var jason = Jason();
  //   var room = jason.initRoom();
  //   var err;
  //   var arg = 123;
  //
  //   try {
  //     await (api.roomHandleMuteVideo(
  //         roomHandle: (room as NativeRoomHandle).opaque,
  //         sourceKind: arg) as Future);
  //   } on FfiException catch (e) {
  //     err = objectFromAnyhow(e) as ArgumentError;
  //   }
  //   expect(err.invalidValue, equals(123));
  //   expect(err.name, 'kind');
  // });
  //
  // testWidgets('Primitive arguments Callback validation',
  //     (WidgetTester widgetTester) async {
  //   final intListener = dl.lookupFunction<Handle Function(ForeignValue),
  //       Object Function(ForeignValue)>('test_callback_listener_int');
  //   final stringListener = dl.lookupFunction<Handle Function(ForeignValue),
  //       Object Function(ForeignValue)>('test_callback_listener_string');
  //   final optionalIntListener = dl.lookupFunction<Handle Function(ForeignValue),
  //       Object Function(ForeignValue)>('test_callback_listener_optional_int');
  //   final optionalStringListener = dl.lookupFunction<
  //       Handle Function(ForeignValue),
  //       Object Function(
  //           ForeignValue)>('test_callback_listener_optional_string');
  //
  //   var intVal = ForeignValue.fromInt(45);
  //   var stringVal = ForeignValue.fromString('test string');
  //   var stringVal2 = ForeignValue.fromString('test string');
  //   var noneVal = ForeignValue.none();
  //
  //   (intListener(intVal.ref) as Function)(45);
  //   (stringListener(stringVal.ref) as Function)('test string');
  //   (optionalIntListener(intVal.ref) as Function)(45);
  //   (optionalIntListener(noneVal.ref) as Function)(null);
  //   (optionalStringListener(stringVal2.ref) as Function)('test string');
  //   (optionalStringListener(noneVal.ref) as Function)(null);
  //
  //   intVal.free();
  //   stringVal.free();
  //   stringVal2.free();
  //   noneVal.free();
  // });
  //
  // testWidgets('DartHandle argument Callback validation',
  //     (WidgetTester widgetTester) async {
  //   dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
  //           'register__test__test_callback_handle_function')(
  //       Pointer.fromFunction<Void Function(Handle)>(testObjMutator));
  //   final dartHandleListener =
  //       dl.lookupFunction<Handle Function(), Object Function()>(
  //           'test_callback_listener_dart_handle');
  //
  //   var obj = TestObj(0);
  //
  //   (dartHandleListener() as Function)(obj);
  //   expect(obj.val, equals(45));
  // });
  //
  // testWidgets('FutureResolver primitives', (WidgetTester widgetTester) async {
  //   final intResolver =
  //       dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
  //           'test__future_from_dart__int');
  //   final stringResolver =
  //       dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
  //           'test__future_from_dart__string');
  //
  //   var intVal = await (intResolver(
  //       () => Future.delayed(Duration(milliseconds: 500), () async {
  //             return 45;
  //           })) as Future);
  //   var stringVal = await (stringResolver(
  //       () => Future.delayed(Duration(milliseconds: 500), () async {
  //             return 'test string';
  //           })) as Future);
  //
  //   expect(intVal as int, equals(45));
  //   expect(stringVal as String, 'test string');
  // });
  //
  // testWidgets('DartHandle argument Future validation',
  //     (WidgetTester widgetTester) async {
  //   dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
  //           'register__test__future_from_dart_handle_fn')(
  //       Pointer.fromFunction<Void Function(Handle)>(testObjMutator));
  //
  //   final handleResolver =
  //       dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
  //           'test__future_from_dart__handle');
  //
  //   var testObj = TestObj(0);
  //   var fut = () => Future.delayed(Duration(milliseconds: 500), () async {
  //         return testObj;
  //       });
  //   await (handleResolver(fut) as Future);
  //   expect(testObj.val, equals(45));
  // });
  //
  // testWidgets('FutureResolver catches exceptions',
  //     (WidgetTester widgetTester) async {
  //   final futureCatchesException =
  //       dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
  //           'test__future_from_dart__fails');
  //
  //   var fut = () => Future.delayed(Duration(milliseconds: 500), () async {
  //         throw Exception('Test Exception');
  //       });
  //   var res = await (futureCatchesException(fut) as Future);
  //   expect(res as int, equals(1));
  // });
  //
  // testWidgets('Panic catcher fires callback and frees Handles',
  //     (WidgetTester widgetTester) async {
  //   final firePanic =
  //       dl.lookupFunction<Void Function(), void Function()>('fire_panic');
  //   final jason = Jason();
  //   var completer = Completer();
  //   onPanic((msg) => completer.complete(msg));
  //   try {
  //     firePanic();
  //   } catch (e) {
  //     var res = await completer.future;
  //     expect(res as String, contains('panicked at'));
  //     expect(jason.opaque.isStale(), true);
  //     return;
  //   }
  //   throw Exception('Exception not fired on panic');
  // });
  //
//   testWidgets('Enumerate displays', (WidgetTester widgetTester) async {
//     var shouldWork = Platform.isLinux || Platform.isMacOS || Platform.isWindows;

//     var jason = Jason();
//     var media = jason.mediaManager();

//     if (!shouldWork) {
//       var displays = await media.enumerateDisplays();

//       expect(displays.length, 1);
//       expect(displays[0].deviceId(), 'device_id');
//       expect(displays[0].title(), 'title');
//     } else {
//       var err;
//       try {
//         await media.enumerateDisplays();
//       } catch (e) {
//         err = e;
//       }
//       expect(err is UnsupportedError, true);
//     }
//   });
}

class TestObj {
  TestObj(this.val);

  int val;
}

void testObjMutator(Object o) {
  o as TestObj;
  o.val = 45;
}
