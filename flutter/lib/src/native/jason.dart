library jason;

import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import '../interface/jason.dart' as base;
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
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
  final el = Platform.isIOS
      ? ExternalLibrary.process(iKnowHowToUseIt: true)
      : ExternalLibrary.open(path);
  final dl = el.ffiDynamicLibrary;

  var initResult = dl.lookupFunction<IntPtr Function(Pointer<Void>),
          int Function(Pointer<Void>)>('init_jason_dart_api_dl')(
      NativeApi.initializeApiDLData);

  if (initResult != 0) {
    throw 'Failed to initialize Dart API. Code: $initResult';
  }

  callback.registerFunctions(dl);
  completer.registerFunctions(dl);
  exceptions.registerFunctions(dl);
  future.registerFunctions(dl);
  function.registerFunctions(dl);
  platform_utils_registerer.registerFunctions(dl);
  list.registerFunctions(dl);
  map.registerFunctions(dl);
  native_string.registerFunctions(dl);

  executor = Executor(dl);

  return el;
}

class Jason implements base.Jason {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.Jason opaque;

  /// Creates a new instance of [Jason].
  static Future<Jason> init() async {
    // Init medea_flutter_webrtc.
    await initFfiBridge();
    if (!RustLib.instance.initialized) {
      await RustLib.init(externalLibrary: el);

      frb.onPanic(cb: (msg) async {
        msg as String;
        await RustHandlesStorage().freeAll();
        if (_onPanicCallback != null) {
          _onPanicCallback!(msg);
        }
      });
    }

    var jason = Jason._();
    var port =
        // ignore: invalid_use_of_internal_member
        ((RustLib.instance.api) as BaseApiImpl).portManager.dartHandlerPort;
    jason.opaque = frb.Jason(dartHandlerPort: port);
    RustHandlesStorage().insertHandle(jason);

    return jason;
  }

  Jason._();

  @override
  MediaManagerHandle mediaManager() {
    return NativeMediaManagerHandle(opaque.jasonMediaManager());
  }

  @override
  RoomHandle initRoom() {
    return NativeRoomHandle(opaque.jasonInitRoom());
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    room as NativeRoomHandle;

    opaque.jasonCloseRoom(roomToDelete: room.opaque);
    room.opaque.dispose();
  }

  @override
  @moveSemantics
  void free() {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);
      opaque.jasonDispose();
      opaque.dispose();
    }
  }
}
