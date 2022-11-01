library jason;

import 'dart:ffi';
import 'dart:io';

import '../interface/jason.dart' as base;
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'ffi/callback.dart' as callback;
import 'ffi/completer.dart' as completer;
import 'ffi/exception.dart' as exceptions;
import 'ffi/executor.dart';
import 'ffi/function.dart' as function;
import 'ffi/future.dart' as future;
import 'ffi/list.dart' as list;
import 'ffi/map.dart' as map;
import 'ffi/native_string.dart' as native_string;
import 'media_manager.dart';
import 'platform/functions_registerer.dart' as platform_utils_registerer;
import 'room_handle.dart';

typedef _cast_C = Handle Function(IntPtr);
typedef _cast_Dart = Object Function(int);

typedef _cast_handle_C = IntPtr Function(Handle);
typedef _cast_handle_Dart = int Function(Object);

typedef _cast_fn_handle_C = IntPtr Function(Handle);
typedef _cast_fn_handle_Dart = int Function(void Function(Pointer));

typedef _onPanic_C = Void Function(Handle);
typedef _onPanic_Dart = void Function(Object);

late api.ApiApiImpl impl_api = _dl_load();
late DynamicLibrary dl;

//todo
Future<dynamic> rust2dart(api.MyDartFuture future) {
  var res = _cast(impl_api.dartFutureToUsize(handle: future)) as Future;
  future.dispose();
  return res;
}

Object rust2dart2(api.DartHandle handle) {
  var res = _cast(impl_api.opaqueToUsize(handle: handle));
  handle.dispose();
  return res;
}

//todo
api.DartHandle handle2rust(Object obj) {
  return impl_api.dartHandleToOpaque(handle: _cast_handle(obj));
}

/// [Executor] that drives Rust futures.
///
/// Instantiated in the [_dl_load()] function, and must not be touched ever
/// after that.
var executor;

final _cast = dl.lookupFunction<_cast_C, _cast_Dart>('int2handle');
final _cast_handle =
    dl.lookupFunction<_cast_handle_C, _cast_handle_Dart>('handle2int');

/// Callback to be fired whenever Rust code panics.
void Function(String)? _onPanicCallback;

/// Sets callback to be fired whenever Rust code panics.
///
/// Once this callback is called, all the old handles returned from Rust SHOULD
/// NOT be used.
void onPanic(void Function(String)? cb) {
  _onPanicCallback = cb;
}

api.ApiApiImpl _dl_load() {
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
  late final _dl = Platform.isIOS
      ? DynamicLibrary.process()
      : Platform.isMacOS
          ? DynamicLibrary.executable()
          : DynamicLibrary.open(path);

  var impl_api = api.ApiApiImpl(_dl);

  var initResult = _dl.lookupFunction<
      IntPtr Function(Pointer<Void>),
      int Function(
          Pointer<Void>)>('init_dart_api_dl')(NativeApi.initializeApiDLData);

  if (initResult != 0) {
    throw 'Failed to initialize Dart API. Code: $initResult';
  }
  callback.registerFunctions(_dl);
  completer.registerFunctions(_dl);
  exceptions.registerFunctions(_dl);
  future.registerFunctions(_dl);
  function.registerFunctions(_dl);
  platform_utils_registerer.registerFunctions(_dl);
  list.registerFunctions(_dl);
  map.registerFunctions(_dl);
  native_string.registerFunctions(_dl);

  executor = Executor(_dl);

  final _onPanic = _dl.lookupFunction<_onPanic_C, _onPanic_Dart>('on_panic');
  _onPanic((msg) {
    msg as String;
    RustHandlesStorage().freeAll();
    if (_onPanicCallback != null) {
      _onPanicCallback!(msg);
    }
  });
  dl = _dl;
  return impl_api;
}

class Jason extends base.Jason {
  /// [Pointer] to the Rust struct backing this object.
  final api.RefCellOptionJason opaque = impl_api.jasonNew();

  Jason() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  MediaManagerHandle mediaManager() {
    return NativeMediaManagerHandle.opaque(
        impl_api.jasonMediaManager(jason: opaque));
  }

  @override
  RoomHandle initRoom() {
    return NativeRoomHandle.opaque(impl_api.jasonInitRoom(jason: opaque));
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    impl_api.jasonCloseRoom(
        jason: opaque, roomToDelete: (room as NativeRoomHandle).opaque);
  }

  @override
  @moveSemantics
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);
      impl_api.jasonDispose(jason: opaque);

      opaque.dispose();
    }
  }
}
