import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/nullable_pointer.dart';
import 'package:medea_jason/src/native/ffi/result.dart';
import 'package:medea_jason/src/native/room_handle.dart';
import 'package:medea_jason/src/native/media_device_info.dart';
import 'package:medea_jason/src/native/local_media_track.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Jason', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();

    expect(() => jason.mediaManager(), returnsNormally);
    expect(() => jason.closeRoom(room), returnsNormally);
    expect(() => jason.closeRoom(room), throwsA(isA<FfiException>()));
  });

  testWidgets('MediaManager', (WidgetTester tester) async {
    final returnsLocalMediaInitException =
        dl.lookupFunction<Result Function(Handle), Result Function(Object)>(
            'returns_local_media_init_exception');
    final returnsFutureWithLocalMediaInitException =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'returns_future_with_local_media_init_exception');
    final returnsEnumerateDevicesException =
        dl.lookupFunction<Result Function(Handle), Result Function(Object)>(
            'returns_enumerate_devices_exception');
    final returnsFutureWithEnumerateDevicesException =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'returns_future_enumerate_devices_exception');

    var jason = Jason();
    var mediaManager = jason.mediaManager();

    var devices = await mediaManager.enumerateDevices();
    var tracks = await mediaManager.initLocalTracks(MediaStreamSettings());

    expect(devices.length, equals(3));
    expect(tracks.length, equals(3));

    expect((devices.first as NativeMediaDeviceInfo).opaque,
        isNot(equals((devices.last as NativeMediaDeviceInfo).opaque)));
    expect((tracks.first as NativeLocalMediaTrack).opaque,
        isNot(equals((tracks.last as NativeLocalMediaTrack).opaque)));

    expect(devices.first.deviceId(), equals('MediaDeviceInfo.device_id'));
    expect(devices.first.groupId(), equals('MediaDeviceInfo.group_id'));
    expect(devices.first.kind(), equals(MediaDeviceKind.audioinput));
    expect(devices.first.label(), equals('MediaDeviceInfo.label'));

    devices.first.free();
    expect(() => devices.first.label(), throwsA(isA<FfiException>()));

    expect(tracks.first.kind(), equals(MediaKind.Video));
    expect(tracks.first.mediaSourceKind(), equals(MediaSourceKind.Display));

    tracks.first.free();
    expect(() => tracks.first.kind(), throwsA(isA<FfiException>()));

    expect(
        () => returnsLocalMediaInitException('Dart err cause1').unwrap(),
        throwsA(predicate((e) =>
            e is LocalMediaInitException &&
            e.kind() == LocalMediaInitExceptionKind.GetUserMediaAudioFailed &&
            e.cause() == 'Dart err cause1' &&
            e.trace().contains('at src'))));

    var err;
    try {
      await (returnsFutureWithLocalMediaInitException('Dart err cause2')
          as Future);
    } catch (e) {
      err = e as LocalMediaInitException;
    }
    expect(
        err,
        predicate((e) =>
            e is LocalMediaInitException &&
            e.kind() == LocalMediaInitExceptionKind.GetDisplayMediaFailed &&
            e.cause() == 'Dart err cause2' &&
            e.trace().contains('at src')));

    expect(
        () => returnsEnumerateDevicesException('Dart err cause3').unwrap(),
        throwsA(predicate((e) =>
            e is EnumerateDevicesException &&
            e.cause() == 'Dart err cause3' &&
            e.trace().contains('at src'))));

    var err2;
    try {
      await (returnsFutureWithEnumerateDevicesException('Dart err cause4')
          as Future);
    } catch (e) {
      err2 = e as EnumerateDevicesException;
    }
    expect(
        err2,
        predicate((e) =>
            e is EnumerateDevicesException &&
            e.cause() == 'Dart err cause4' &&
            e.trace().contains('at src')));
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

  testWidgets('DisplayVideoTrackConstraints', (WidgetTester tester) async {
    var constraints = DisplayVideoTrackConstraints();
    constraints.free();

    var constraints2 = DisplayVideoTrackConstraints();
    var settings = MediaStreamSettings();
    settings.displayVideo(constraints2);
    expect(() => settings.displayVideo(constraints2),
        throwsA(isA<FfiException>()));
  });

  testWidgets('AudioTrackConstraints', (WidgetTester tester) async {
    var constraints = AudioTrackConstraints();
    constraints.deviceId('deviceId');
    constraints.free();
    expect(
        () => constraints.deviceId('deviceId'), throwsA(isA<FfiException>()));

    var constraints2 = AudioTrackConstraints();
    var settings = MediaStreamSettings();
    constraints2.deviceId('deviceId');
    settings.audio(constraints2);
    expect(
        () => constraints2.deviceId('deviceId'), throwsA(isA<FfiException>()));
  });

  testWidgets('RoomHandle', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();

    var allFired = List<Completer>.generate(4, (_) => Completer());

    room.onClose((reason) {
      allFired[0].complete();
    });

    room.onConnectionLoss((reconnectHandle) {
      allFired[1].complete();
    });

    room.onLocalTrack((localTrack) {
      allFired[2].complete();
    });

    room.onNewConnection((connection) {
      allFired[3].complete();
    });

    await Future.wait(allFired.map((e) => e.future))
        .timeout(Duration(seconds: 1));

    room.free();

    expect(() => room.onNewConnection((_) {}), throwsA(isA<FfiException>()));
  });

  testWidgets('RoomCloseReason', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();
    var reasonFut = Completer<RoomCloseReason>();

    room.onClose((reason) {
      reasonFut.complete(reason);
    });

    var reason = await reasonFut.future.timeout(Duration(seconds: 1));

    expect(reason.reason(), equals('RpcClientUnexpectedlyDropped'));
    expect(reason.isClosedByServer(), equals(false));
    expect(reason.isErr(), equals(true));
    reason.free();
    expect(() => reason.isErr(), throwsA(isA<FfiException>()));
  });

  testWidgets('ConnectionHandle', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();

    var connFut = Completer<ConnectionHandle>();
    room.onNewConnection((conn) {
      connFut.complete(conn);
    });
    var conn = await connFut.future;

    expect(
        () => conn.getRemoteMemberId(),
        throwsA(predicate((e) =>
            e is StateError &&
            e.message == '`ConnectionHandle` is in detached state')));
    var allFired = List<Completer>.generate(2, (_) => Completer());
    conn.onQualityScoreUpdate((score) {
      allFired[0].complete(score);
    });
    conn.onClose(() {
      allFired[1].complete();
    });

    var res = await Future.wait(allFired.map((e) => e.future))
        .timeout(Duration(seconds: 1));
    expect(res[0], 4);
  });

  testWidgets('ConnectionHandle', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();

    var connFut = Completer<ConnectionHandle>();
    room.onNewConnection((conn) {
      connFut.complete(conn);
    });
    var conn = await connFut.future;

    var trackFut = Completer<RemoteMediaTrack>();
    conn.onRemoteTrackAdded((remoteTrack) {
      trackFut.complete(remoteTrack);
    });

    var track = await trackFut.future;

    expect(track.muted(), equals(false));
    expect(track.kind(), equals(MediaKind.Video));
    expect(track.mediaSourceKind(), equals(MediaSourceKind.Device));

    var allFired = List<Completer>.generate(3, (_) => Completer());
    track.onMuted(() {
      allFired[0].complete();
    });
    track.onUnmuted(() {
      allFired[1].complete();
    });
    track.onStopped(() {
      allFired[2].complete();
    });
    track.onMediaDirectionChanged((direction) {
      expect(direction, TrackMediaDirection.SendRecv);
      allFired[3].complete();
    });

    await Future.wait(allFired.map((e) => e.future))
        .timeout(Duration(seconds: 1));

    track.free();
    expect(() => track.kind(), throwsA(isA<FfiException>()));
  });

  testWidgets('RoomHandle', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();

    await room.join('wss://example.com/room/Alice?token=777');
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

  testWidgets('ReconnectHandle', (WidgetTester tester) async {
    final returnsRpcClientException =
        dl.lookupFunction<Result Function(Handle), Result Function(Object)>(
            'returns_rpc_client_exception');
    final returnsFutureWithRpcClientException =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'returns_future_rpc_client_exception');

    var jason = Jason();
    var room = jason.initRoom();

    var handleFut = Completer<ReconnectHandle>();
    room.onConnectionLoss((reconnectHandle) {
      handleFut.complete(reconnectHandle);
    });
    var handle = await handleFut.future;

    await handle.reconnectWithDelay(155);
    await handle.reconnectWithBackoff(1, 2, 3);

    var exception;
    try {
      await handle.reconnectWithDelay(-1);
    } catch (e) {
      exception = e;
    }
    expect(exception, isArgumentError);

    var exception2;
    try {
      await handle.reconnectWithBackoff(-1, 2, 3, 145);
    } catch (e) {
      exception2 = e;
    }
    expect(exception2, isArgumentError);

    var exception3;
    try {
      await handle.reconnectWithBackoff(1, 2, -3, 333);
    } catch (e) {
      exception3 = e;
    }
    expect(exception3, isArgumentError);
    var argumentError = exception3 as ArgumentError;
    expect(argumentError.invalidValue, equals(-3));
    expect(argumentError.name, 'maxDelay');
    expect(argumentError.message, 'Expected u32');

    var exception4;
    try {
      await handle.reconnectWithBackoff(1, 2, 3, -4);
    } catch (e) {
      exception4 = e;
    }
    expect(exception4, isArgumentError);
    var argumentError2 = exception4 as ArgumentError;
    expect(argumentError2.invalidValue, equals(-4));
    expect(argumentError2.name, 'maxElapsedTimeMs');
    expect(argumentError2.message, 'Expected u32');

    expect(
        () => returnsRpcClientException('Dart err cause1').unwrap(),
        throwsA(predicate((e) =>
            e is RpcClientException &&
            e.kind() == RpcClientExceptionKind.ConnectionLost &&
            e.cause() == 'Dart err cause1' &&
            e.message() == 'RpcClientException::ConnectionLost' &&
            e.trace().contains('at src'))));

    var exception5;
    try {
      await (returnsFutureWithRpcClientException('Dart err cause2') as Future);
    } catch (e) {
      exception5 = e;
    }
    expect(
        exception5,
        predicate((e) =>
            e is RpcClientException &&
            e.kind() == RpcClientExceptionKind.SessionFinished &&
            e.message() == 'RpcClientException::SessionFinished' &&
            e.cause() == 'Dart err cause2' &&
            e.trace().contains('at src')));
  });

  final returnsInputDevicePtr =
      dl.lookupFunction<ForeignValue Function(), ForeignValue Function()>(
          'returns_media_device_info_ptr');

  testWidgets('ForeignValue', (WidgetTester tester) async {
    Pointer rustPtr1 = returnsInputDevicePtr().toDart();

    var fvN = ForeignValue.none();
    var fvI = ForeignValue.fromInt(145);
    var fvS = ForeignValue.fromString('my string');
    var fvH = ForeignValue.fromHandle(TestObj(333));
    var fvR = ForeignValue.fromPtr(NullablePointer(rustPtr1));

    expect(fvN.ref.toDart(), null);
    expect(fvI.ref.toDart(), 145);
    expect(fvS.ref.toDart(), 'my string');
    expect((fvH.ref.toDart() as TestObj).val, 333);
    expect((fvR.ref.toDart() as Pointer).address, rustPtr1.address);

    fvN.free();
    fvI.free();
    fvS.free();
    fvH.free();
    fvR.free();

    Pointer rustPtr2 = returnsInputDevicePtr().toDart();

    var fvN2 = ForeignValue.fromDart(null);
    var fvI2 = ForeignValue.fromDart(555);
    var fvS2 = ForeignValue.fromDart('my string');
    var fvH2 = ForeignValue.fromDart(TestObj(666));
    var fvR2 = ForeignValue.fromDart(NullablePointer(rustPtr2));

    expect(fvN2.ref.toDart(), null);
    expect(fvI2.ref.toDart(), 555);
    expect(fvS2.ref.toDart(), 'my string');
    expect((fvH2.ref.toDart() as TestObj).val, 666);
    expect((fvR2.ref.toDart() as Pointer).address, rustPtr2.address);

    fvN2.free();
    fvI2.free();
    fvS2.free();
    fvH2.free();
    fvR2.free();
  });

  testWidgets('ForeignValue Dart => Rust', (WidgetTester tester) async {
    final acceptsNone = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_none');
    final acceptsPtr = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_media_device_info_pointer');
    final acceptsString = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_string');
    final acceptsInt = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_int');

    var none = ForeignValue.none();
    var ptr =
        ForeignValue.fromPtr(NullablePointer(returnsInputDevicePtr().toDart()));
    var str = ForeignValue.fromString('my string');
    var num = ForeignValue.fromInt(235);

    acceptsNone(none.ref);
    acceptsPtr(ptr.ref);
    acceptsString(str.ref);
    acceptsInt(num.ref);

    none.free();
    ptr.free();
    str.free();
    num.free();
  });

  testWidgets('ForeignValue Dart => Rust', (WidgetTester tester) async {
    final acceptsNone = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_none');
    final acceptsPtr = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_media_device_info_pointer');
    final acceptsString = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_string');
    final acceptsInt = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_int');

    var none = ForeignValue.none();
    var ptr =
        ForeignValue.fromPtr(NullablePointer(returnsInputDevicePtr().toDart()));
    var str = ForeignValue.fromString('my string');
    var num = ForeignValue.fromInt(235);

    acceptsNone(none.ref);
    acceptsPtr(ptr.ref);
    acceptsString(str.ref);
    acceptsInt(num.ref);

    none.free();
    ptr.free();
    str.free();
    num.free();
  });

  testWidgets('Complex arguments validation', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();
    var err;
    var arg = 123;

    try {
      await (api.roomHandleMuteVideo(
          roomHandle: (room as NativeRoomHandle).opaque,
          sourceKind: arg) as Future);
    } on FfiException catch (e) {
      err = objectFromAnyhow(e) as ArgumentError;
    }
    expect(err.invalidValue, equals(123));
    expect(err.name, 'kind');
  });

  testWidgets('Primitive arguments Callback validation',
      (WidgetTester widgetTester) async {
    final intListener = dl.lookupFunction<Handle Function(ForeignValue),
        Object Function(ForeignValue)>('test_callback_listener_int');
    final stringListener = dl.lookupFunction<Handle Function(ForeignValue),
        Object Function(ForeignValue)>('test_callback_listener_string');
    final optionalIntListener = dl.lookupFunction<Handle Function(ForeignValue),
        Object Function(ForeignValue)>('test_callback_listener_optional_int');
    final optionalStringListener = dl.lookupFunction<
        Handle Function(ForeignValue),
        Object Function(
            ForeignValue)>('test_callback_listener_optional_string');

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

  testWidgets('DartHandle argument Callback validation',
      (WidgetTester widgetTester) async {
    dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
            'register__test__test_callback_handle_function')(
        Pointer.fromFunction<Void Function(Handle)>(testObjMutator));
    final dartHandleListener =
        dl.lookupFunction<Handle Function(), Object Function()>(
            'test_callback_listener_dart_handle');

    var obj = TestObj(0);

    (dartHandleListener() as Function)(obj);
    expect(obj.val, equals(45));
  });

  testWidgets('FutureResolver primitives', (WidgetTester widgetTester) async {
    final intResolver =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'test__future_from_dart__int');
    final stringResolver =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'test__future_from_dart__string');

    var intVal = await (intResolver(
        () => Future.delayed(Duration(milliseconds: 500), () async {
              return 45;
            })) as Future);
    var stringVal = await (stringResolver(
        () => Future.delayed(Duration(milliseconds: 500), () async {
              return 'test string';
            })) as Future);

    expect(intVal as int, equals(45));
    expect(stringVal as String, 'test string');
  });

  testWidgets('DartHandle argument Future validation',
      (WidgetTester widgetTester) async {
    dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
            'register__test__future_from_dart_handle_fn')(
        Pointer.fromFunction<Void Function(Handle)>(testObjMutator));

    final handleResolver =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'test__future_from_dart__handle');

    var testObj = TestObj(0);
    var fut = () => Future.delayed(Duration(milliseconds: 500), () async {
          return testObj;
        });
    await (handleResolver(fut) as Future);
    expect(testObj.val, equals(45));
  });

  testWidgets('FutureResolver catches exceptions',
      (WidgetTester widgetTester) async {
    final futureCatchesException =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'test__future_from_dart__fails');

    var fut = () => Future.delayed(Duration(milliseconds: 500), () async {
          throw Exception('Test Exception');
        });
    var res = await (futureCatchesException(fut) as Future);
    expect(res as int, equals(1));
  });

  testWidgets('Panic catcher fires callback and frees Handles',
      (WidgetTester widgetTester) async {
    final firePanic =
        dl.lookupFunction<Void Function(), void Function()>('fire_panic');
    final jason = Jason();
    var completer = Completer();
    onPanic((msg) => completer.complete(msg));
    try {
      firePanic();
    } catch (e) {
      var res = await completer.future;
      expect(res as String, contains('panicked at'));
      expect(jason.opaque.isStale(), true);
      return;
    }
    throw Exception('Exception not fired on panic');
  });

  testWidgets('Volume settings', (WidgetTester widgetTester) async {
    var jason = Jason();
    var media = jason.mediaManager();

    expect(await media.microphoneVolumeIsAvailable(), true);
    expect(await media.microphoneVolume(), 50);
    expect(() async => await media.setMicrophoneVolume(100), returnsNormally);
  });

  testWidgets('Enumerate displays', (WidgetTester widgetTester) async {
    var shouldWork = Platform.isLinux || Platform.isMacOS || Platform.isWindows;

    var jason = Jason();
    var media = jason.mediaManager();

    if (shouldWork) {
      var displays = await media.enumerateDisplays();

      expect(displays.length, 1);
      expect(displays[0].deviceId(), 'device_id');
      expect(displays[0].title(), 'title');
    } else {
      var err;
      try {
        await media.enumerateDisplays();
      } catch (e) {
        err = e;
      }
      expect(err is UnsupportedError, true);
    }
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
