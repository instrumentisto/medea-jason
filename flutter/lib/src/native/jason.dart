library jason;

import 'dart:ffi';
import 'dart:io';

import '../interface/jason.dart' as base;
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/callback.dart' as callback;
import 'ffi/completer.dart' as completer;
import 'ffi/exception.dart' as exceptions;
import 'ffi/executor.dart';
import 'ffi/function.dart' as function;
import 'ffi/future.dart' as future;
import 'ffi/jason_api.g.dart' as frb;
import 'ffi/list.dart' as list;
import 'ffi/map.dart' as map;
import 'ffi/native_string.dart' as native_string;
import 'media_manager.dart';
import 'platform/functions_registerer.dart' as platform_utils_registerer;
import 'room_handle.dart';

/// Bindings to the Rust side API.
frb.MedeaJason api = _initApi();
DynamicLibrary dl = _dlLoad();

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

DynamicLibrary _dlLoad() {
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
  final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
  late final dl = Platform.isIOS
      ? DynamicLibrary.process()
      : Platform.isMacOS
          ? DynamicLibrary.executable()
          : DynamicLibrary.open(path);

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

  return dl;
}

frb.MedeaJason _initApi() {
  var api = frb.MedeaJasonImpl(dl);
  api.onPanic(cb: (msg) async {
    msg as String;
    print("-------------------------------");
    print(msg);
    print("-------------------------------");
    await RustHandlesStorage().freeAll();
    if (_onPanicCallback != null) {
      _onPanicCallback!(msg);
    }
  });
  return api;
}

class Jason implements base.Jason {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final RustOpaque<frb.Jason> opaque = RustOpaque(api.jasonNew());

  /// Constructs a new [Jason] backed by the Rust struct behind the provided
  /// [frb.Jason].
  Jason() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  MediaManagerHandle mediaManager() {
    return NativeMediaManagerHandle(
        api.jasonMediaManager(jason: opaque.innerOpaque));
  }

  @override
  RoomHandle initRoom() {
    return NativeRoomHandle(api.jasonInitRoom(jason: opaque.innerOpaque));
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    api.jasonCloseRoom(
        jason: opaque.innerOpaque,
        roomToDelete: (room as NativeRoomHandle).opaque.moveOpaque);
  }

  @override
  @moveSemantics
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);
      api.jasonDispose(jason: opaque.moveOpaque);
    }
  }
}
