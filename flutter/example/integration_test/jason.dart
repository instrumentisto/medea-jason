import 'dart:async';
import 'dart:ffi';

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_jason/src/interface/track_kinds.dart';
import 'package:medea_jason/src/native/audio_track_constraints.dart';
import 'package:medea_jason/src/native/device_video_track_constraints.dart';
import 'package:medea_jason/src/native/display_video_track_constraints.dart';
import 'package:medea_jason/src/native/ffi/exceptions.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/nullable_pointer.dart';
import 'package:medea_jason/src/native/ffi/result.dart';
import 'package:medea_jason/src/native/input_device_info.dart';
import 'package:medea_jason/src/native/jason.dart';
import 'package:medea_jason/src/native/local_media_track.dart';
import 'package:medea_jason/src/native/media_stream_settings.dart';
import 'package:medea_jason/src/native/room_handle.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Jason', (WidgetTester tester) async {
    var jason = Jason();
    var room = jason.initRoom();

    expect(() => jason.mediaManager(), returnsNormally);
    expect(() => jason.closeRoom(room), returnsNormally);
    expect(() => jason.closeRoom(room), throwsStateError);
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

    expect(
        (devices.first as NativeInputDeviceInfo).ptr.getInnerPtr(),
        isNot(
            equals((devices.last as NativeInputDeviceInfo).ptr.getInnerPtr())));
    expect(
        (tracks.first as NativeLocalMediaTrack).ptr.getInnerPtr(),
        isNot(
            equals((tracks.last as NativeLocalMediaTrack).ptr.getInnerPtr())));

    expect(devices.first.deviceId(), equals('InputDeviceInfo.device_id'));
    expect(devices.first.groupId(), equals('InputDeviceInfo.group_id'));
    expect(devices.first.kind(), equals(MediaKind.Audio));
    expect(devices.first.label(), equals('InputDeviceInfo.label'));

    devices.first.free();
    expect(() => devices.first.label(), throwsStateError);

    expect(tracks.first.kind(), equals(MediaKind.Video));
    expect(tracks.first.mediaSourceKind(), equals(MediaSourceKind.Display));

    tracks.first.free();
    expect(() => tracks.first.kind(), throwsStateError);

    expect(
        () => returnsLocalMediaInitException('Dart err cause1').unwrap(),
        throwsA(predicate((e) =>
            e is LocalMediaInitException &&
            e.kind == LocalMediaInitExceptionKind.GetUserMediaFailed &&
            e.cause == 'Dart err cause1' &&
            e.nativeStackTrace.contains('at src'))));

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
            e.kind == LocalMediaInitExceptionKind.GetDisplayMediaFailed &&
            e.cause == 'Dart err cause2' &&
            e.nativeStackTrace.contains('at src')));

    expect(
        () => returnsEnumerateDevicesException('Dart err cause3').unwrap(),
        throwsA(predicate((e) =>
            e is EnumerateDevicesException &&
            e.cause == 'Dart err cause3' &&
            e.nativeStackTrace.contains('at src'))));

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
            e.cause == 'Dart err cause4' &&
            e.nativeStackTrace.contains('at src')));
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
    expect(() => constraints.deviceId('deviceId'), throwsStateError);

    var constraints2 = DeviceVideoTrackConstraints();
    var settings = MediaStreamSettings();
    constraints2.deviceId('deviceId');
    settings.deviceVideo(constraints2);
    expect(() => constraints2.deviceId('deviceId'), throwsStateError);
  });

  testWidgets('DisplayVideoTrackConstraints', (WidgetTester tester) async {
    var constraints = DisplayVideoTrackConstraints();
    constraints.free();
    expect(() => constraints.free(), throwsStateError);

    var constraints2 = DisplayVideoTrackConstraints();
    var settings = MediaStreamSettings();
    settings.displayVideo(constraints2);
    expect(() => settings.displayVideo(constraints2), throwsStateError);
  });

  testWidgets('AudioTrackConstraints', (WidgetTester tester) async {
    var constraints = AudioTrackConstraints();
    constraints.deviceId('deviceId');
    constraints.free();
    expect(() => constraints.deviceId('deviceId'), throwsStateError);

    var constraints2 = AudioTrackConstraints();
    var settings = MediaStreamSettings();
    constraints2.deviceId('deviceId');
    settings.audio(constraints2);
    expect(() => constraints2.deviceId('deviceId'), throwsStateError);
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

    expect(() => room.onNewConnection((_) {}), throwsStateError);
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
    expect(() => reason.isErr(), throwsStateError);
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
            e.message == 'ConnectionHandle is in detached state')));
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

    expect(track.enabled(), equals(true));
    expect(track.muted(), equals(false));
    expect(track.kind(), equals(MediaKind.Video));
    expect(track.mediaSourceKind(), equals(MediaSourceKind.Device));

    var allFired = List<Completer>.generate(5, (_) => Completer());
    track.onEnabled(() {
      allFired[0].complete();
    });
    track.onDisabled(() {
      allFired[1].complete();
    });
    track.onMuted(() {
      allFired[2].complete();
    });
    track.onUnmuted(() {
      allFired[3].complete();
    });
    track.onStopped(() {
      allFired[4].complete();
    });

    await Future.wait(allFired.map((e) => e.future))
        .timeout(Duration(seconds: 1));

    track.free();
    expect(() => track.kind(), throwsStateError);
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
    await room.disableRemoteVideo();

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
            e is MediaStateTransitionException &&
            e.message == 'SimpleTracksRequest should have at least one track' &&
            e.nativeStackTrace.contains('at src')));
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
            e.kind == RpcClientExceptionKind.ConnectionLost &&
            e.cause == 'Dart err cause1' &&
            e.message == 'RpcClientException::ConnectionLost' &&
            e.nativeStackTrace.contains('at src'))));

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
            e.kind == RpcClientExceptionKind.SessionFinished &&
            e.message == 'RpcClientException::SessionFinished' &&
            e.cause == 'Dart err cause2' &&
            e.nativeStackTrace.contains('at src')));
  });

  final returnsInputDevicePtr =
      dl.lookupFunction<ForeignValue Function(), ForeignValue Function()>(
          'returns_input_device_info_ptr');

  testWidgets('ForeignValue Rust => Dart', (WidgetTester tester) async {
    final returnsNone =
        dl.lookupFunction<ForeignValue Function(), ForeignValue Function()>(
            'returns_none');
    final returnsHandlePtr = dl.lookupFunction<ForeignValue Function(Handle),
        ForeignValue Function(Object?)>('returns_handle_ptr');
    final returnsString =
        dl.lookupFunction<ForeignValue Function(), ForeignValue Function()>(
            'returns_string');
    final returnsInt =
        dl.lookupFunction<ForeignValue Function(), ForeignValue Function()>(
            'returns_int');

    expect(returnsNone().toDart(), equals(null));

    var inputDevice = NativeInputDeviceInfo(
        NullablePointer(returnsInputDevicePtr().toDart()));
    expect(inputDevice.deviceId(), equals('InputDeviceInfo.device_id'));
    inputDevice.free();

    expect(returnsHandlePtr('asd').toDart(), equals('asd'));
    expect(returnsHandlePtr(111).toDart(), equals(111));
    expect(returnsHandlePtr(null).toDart(), equals(null));

    expect(returnsString().toDart(), equals('QWERTY'));

    expect(returnsInt().toDart(), equals(333));
  });

  testWidgets('ForeignValue Dart => Rust', (WidgetTester tester) async {
    final acceptsNone = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_none');
    final acceptsPtr = dl.lookupFunction<Void Function(ForeignValue),
        void Function(ForeignValue)>('accepts_input_device_info_pointer');
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
    final _muteVideo = dl.lookupFunction<Handle Function(Pointer, ForeignValue),
        Object Function(Pointer, ForeignValue)>('RoomHandle__mute_video');

    var jason = Jason();
    var room = jason.initRoom();

    var err;
    var arg = ForeignValue.fromInt(123);
    try {
      await (_muteVideo((room as NativeRoomHandle).ptr.getInnerPtr(), arg.ref)
          as Future);
    } catch (e) {
      err = e as ArgumentError;
    } finally {
      arg.free();
    }
    expect(err.invalidValue, equals(123));
    expect(err.name, 'kind');
  });

  testWidgets('Primitive arguments Callback validation',
      (WidgetTester widgetTester) async {
    final intListener = dl.lookupFunction<Handle Function(ForeignValue),
        Object Function(ForeignValue)>('test__callback_listener__int');
    final stringListener = dl.lookupFunction<Handle Function(ForeignValue),
        Object Function(ForeignValue)>('test__callback_listener__string');
    final optionalIntListener = dl.lookupFunction<Handle Function(ForeignValue),
        Object Function(ForeignValue)>('test__callback_listener__optional_int');
    final optionalStringListener = dl.lookupFunction<
        Handle Function(ForeignValue),
        Object Function(
            ForeignValue)>('test__callback_listener__optional_string');

    var intVal = ForeignValue.fromInt(45);
    var stringVal = ForeignValue.fromString('test string');
    var noneVal = ForeignValue.none();

    (intListener(intVal.ref) as Function)(45);
    (stringListener(stringVal.ref) as Function)('test string');
    (optionalIntListener(intVal.ref) as Function)(45);
    (optionalIntListener(noneVal.ref) as Function)(null);
    (optionalStringListener(stringVal.ref) as Function)('test string');
    (optionalStringListener(noneVal.ref) as Function)(null);

    intVal.free();
    stringVal.free();
    noneVal.free();
  });

  testWidgets('DartHandle argument Callback validation',
      (WidgetTester widgetTester) async {
    dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
            'register__test__test_callback_handle_function')(
        Pointer.fromFunction<Void Function(Handle)>(testObjMutator));
    final dartHandleListener =
        dl.lookupFunction<Handle Function(), Object Function()>(
            'test__callback_listener__dart_handle');

    var obj = TestObj();

    (dartHandleListener() as Function)(obj);
    expect(obj.x, equals(45));
  });

  testWidgets('FutureResolver primitives', (WidgetTester widgetTester) async {
    final intResolver =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'test__dart_future_resolver__int');
    final stringResolver =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'test__dart_future_resolver__string');

    var intVal = await (intResolver(
        Future.delayed(Duration(milliseconds: 500), () async {
      return 45;
    })) as Future);
    var stringVal = await (stringResolver(
        Future.delayed(Duration(milliseconds: 500), () async {
      return 'test string';
    })) as Future);

    expect(intVal as int, equals(45));
    expect(stringVal as String, 'test string');
  });

  testWidgets('DartHandle argument Future validation',
      (WidgetTester widgetTester) async {
    dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
            'register__test__dart_future_resolver_handle_fn')(
        Pointer.fromFunction<Void Function(Handle)>(testObjMutator));

    final handleResolver =
        dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
            'test__dart_future_resolver__handle');

    var testObj = TestObj();
    var fut = Future.delayed(Duration(milliseconds: 500), () async {
      return testObj;
    });
    await (handleResolver(fut) as Future);
    expect(testObj.x, equals(45));
  });

  // testWidgets('FallibleFutureResolver catches exceptions', (WidgetTester widgetTester) async {
  //   final fallibleFutureCatchesException =
  //     dl.lookupFunction<Handle Function(Handle), Object Function(Object)>(
  //         'test__fallible_dart_future_resolver__fails');
  //
  //   try {
  //     var fut = Future.delayed(Duration(milliseconds: 2000), () async {
  //       throw Exception('Just testing exceptions');
  //     });
  //     var res = await (fallibleFutureCatchesException(fut) as Future);
  //     expect(res as int, equals(1));
  //   } catch (e) {
  //     print('Catched in test case');
  //   }
  //
  // });
}

class TestObj {
  int x = 0;
}

void testObjMutator(Object o) {
  (o as TestObj).x = 45;
}
