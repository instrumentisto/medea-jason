library jason;

import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

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

class Jason implements base.Jason {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.JasonHandle opaque;

  /// Constructs a new [Jason] backed by the Rust struct behind the provided
  /// [frb.Jason].
  static Future<Jason> init() async {
    const stem = 'medea_jason';

    var lib = await loadExternalLibrary(const ExternalLibraryLoaderConfig(
        stem: stem, ioDirectory: '', webPrefix: ''));

    await RustLib.init(externalLibrary: lib);

    var jason = Jason._();
    jason.opaque = frb.JasonHandle();

    RustHandlesStorage().insertHandle(jason);

    print('-------------------------------------------------------------');
    var port =
        ((RustLib.instance.api) as BaseApiImpl).portManager.dartHandlerPort;
    print(port);
    print('-------------------------------------------------------------');
    // RustLib.instance.portManager;

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
