library jason;

import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import '../interface/jason.dart' as base;
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart' as util;
import '/src/util/rust_handles_storage.dart';
import 'ffi/callback.dart' as callback;
import 'ffi/completer.dart' as completer;
import 'ffi/exception.dart' as exceptions;
import 'ffi/executor.dart';
import 'ffi/frb/frb.dart' as frb;
import 'ffi/frb/frb_generated.dart';
import 'ffi/function.dart' as function;
import 'ffi/future.dart' as future;
import 'ffi/list.dart' as list;
import 'ffi/map.dart' as map;
import 'ffi/native_string.dart' as native_string;
import 'media_manager.dart';
import 'platform/functions_registerer.dart' as platform_utils_registerer;
import 'room_handle.dart';

/// Bindings to the Rust side API.
final ExternalLibrary el = _dlLoad();
final DynamicLibrary dl = el.ffiDynamicLibrary;

/// [Executor] that drives Rust futures.
///
/// Instantiated in the [_dl_load()] function, and must not be touched ever
/// after that.
late Executor executor;

/// Callback to be fired whenever Rust code panics.
void Function(String)? _onPanicCallback;

/// Sets callback to be fired whenever Rust code panics.
///
/// Once this callback is called, all the old handles returned from Rust SHOULD
/// NOT be used.
void onPanic(void Function(String)? cb) {
  _onPanicCallback = cb;
}

ExternalLibrary _dlLoad() {
  print("dlload start");
  if (!(Platform.isAndroid ||
      Platform.isLinux ||
      Platform.isWindows ||
      Platform.isMacOS ||
      Platform.isIOS)) {
    throw UnsupportedError('This platform is not supported.');
  }
  if (NativeApi.majorVersion != 2) {
    // If the DartVM we're running on does not have the same major version as
    // this file was compiled against, refuse to initialize: the symbols are not
    // compatible.
    throw 'You are running unsupported NativeApi version.';
  }

  const base = 'medea_jason';
  final path = Platform.isWindows
      ? '$base.dll'
      : Platform.isLinux || Platform.isAndroid
          ? 'lib$base.so'
          : 'lib$base.dylib';
  print("load library 1");
  final el = Platform.isIOS
      ? ExternalLibrary.process(iKnowHowToUseIt: true)
      : ExternalLibrary.open(path);
  print("load library 2");
  final dl = el.ffiDynamicLibrary;

  print("dlload::init 1");
  var initResult = dl.lookupFunction<IntPtr Function(Pointer<Void>),
          int Function(Pointer<Void>)>('init_jason_dart_api_dl')(
      NativeApi.initializeApiDLData);
  print("dlload::init 2");

  if (initResult != 0) {
    throw 'Failed to initialize Dart API. Code: $initResult';
  }

  print("dlload registerFunctions 1");
  callback.registerFunctions(dl);
  print("dlload registerFunctions 2");
  completer.registerFunctions(dl);
  print("dlload registerFunctions 3");
  exceptions.registerFunctions(dl);
  print("dlload registerFunctions 4");
  future.registerFunctions(dl);
  print("dlload registerFunctions 5");
  function.registerFunctions(dl);
  print("dlload registerFunctions 6");
  platform_utils_registerer.registerFunctions(dl);
  print("dlload registerFunctions 7");
  list.registerFunctions(dl);
  print("dlload registerFunctions 8");
  map.registerFunctions(dl);
  print("dlload registerFunctions 9");
  native_string.registerFunctions(dl);
  print("dlload registerFunctions 10");

  executor = Executor(dl);
  print("dlload::Executor OK");

  print("dlload end");
  return el;
}

class Jason implements base.Jason {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late util.RustOpaque<frb.Jason> opaque;

  /// Creates a new instance of [Jason].
  static Future<Jason> init() async {
    print("Jason::init 1");
    // Init `medea_flutter_webrtc`.
    await initFfiBridge();
    print("Jason::init 2");
    if (!RustLib.instance.initialized) {
      await RustLib.init(externalLibrary: el);
      print("Jason::init 3");

      var port =
          // ignore: invalid_use_of_internal_member
          ((RustLib.instance.api) as BaseApiImpl).portManager.dartHandlerPort;
      print("Jason::init 4");

      frb.onPanic(cb: (msg) async {
        msg as String;
        await RustHandlesStorage().freeAll();
        if (_onPanicCallback != null) {
          _onPanicCallback!(msg);
        }
      });
      print("Jason::init 5");
      frb.setDartOpaqueMessagePort(dartHandlerPort: port);
    }
    print("Jason::init 6");

    var jason = Jason._();
    print("Jason::init 7");

    jason.opaque = util.RustOpaque(frb.Jason());
    print("Jason::init 8");
    RustHandlesStorage().insertHandle(jason);
    print("Jason::init 9");

    return jason;
  }

  Jason._();

  @override
  MediaManagerHandle mediaManager() {
    return NativeMediaManagerHandle(opaque.inner.jasonMediaManager());
  }

  @override
  RoomHandle initRoom() {
    return NativeRoomHandle(opaque.inner.jasonInitRoom());
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    room as NativeRoomHandle;

    opaque.inner.jasonCloseRoom(roomToDelete: room.opaque.inner);
    room.opaque.dispose();
  }

  @override
  @moveSemantics
  void free() {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);
      opaque.inner.jasonDispose();
      opaque.dispose();
    }
  }
}
