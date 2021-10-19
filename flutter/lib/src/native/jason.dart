library jason;

import 'dart:ffi';
import 'dart:io';

import '../interface/jason.dart' as base;
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import 'ffi/callback.dart' as callback;
import 'ffi/completer.dart' as completer;
import 'ffi/exceptions.dart' as exceptions;
import 'ffi/executor.dart';
import 'ffi/nullable_pointer.dart';
import 'media_manager.dart';
import 'room_handle.dart';

typedef _new_C = Pointer Function();
typedef _new_Dart = Pointer Function();

typedef _mediaManager_C = Pointer Function(Pointer);
typedef _mediaManager_Dart = Pointer Function(Pointer);

typedef _closeRoom_C = Void Function(Pointer, Pointer);
typedef _closeRoom_Dart = void Function(Pointer, Pointer);

typedef _initRoom_C = Pointer Function(Pointer);
typedef _initRoom_Dart = Pointer Function(Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final DynamicLibrary dl = _dl_load();

/// [Executor] that drives Rust futures.
///
/// Instantiated in the [_dl_load()] function, and must not be touched ever
/// after that.
var executor;

final _new = dl.lookupFunction<_new_C, _new_Dart>('Jason__new');

final _media_manager = dl.lookupFunction<_mediaManager_C, _mediaManager_Dart>(
    'Jason__media_manager');

final _initRoom =
    dl.lookupFunction<_initRoom_C, _initRoom_Dart>('Jason__init_room');

final _close_room =
    dl.lookupFunction<_closeRoom_C, _closeRoom_Dart>('Jason__close_room');

final _free = dl.lookupFunction<_free_C, _free_Dart>('Jason__free');

DynamicLibrary _dl_load() {
  if (!Platform.isAndroid) {
    throw UnsupportedError('This platform is not supported.');
  }
  if (NativeApi.majorVersion != 2) {
    // If the DartVM we're running on does not have the same major version as
    // this file was compiled against, refuse to initialize: the symbols are not
    // compatible.
    throw 'You are running unsupported NativeApi version.';
  }

  var dl = DynamicLibrary.open('libmedea_jason.so');

  var initResult = dl.lookupFunction<
      IntPtr Function(Pointer<Void>),
      int Function(
          Pointer<Void>)>('init_dart_api_dl')(NativeApi.initializeApiDLData);

  if (initResult != 0) {
    throw 'Failed to initialize Dart API. Code: $initResult';
  }
  callback.registerFunctions(dl);
  completer.registerFunctions(dl);
  exceptions.registerFunctions(dl);

  executor = Executor(dl);

  return dl;
}

class Jason extends base.Jason {
  /// [Pointer] to the Rust struct backing this object.
  final NullablePointer ptr = NullablePointer(_new());

  @override
  MediaManagerHandle mediaManager() {
    return NativeMediaManagerHandle(
        NullablePointer(_media_manager(ptr.getInnerPtr())));
  }

  @override
  RoomHandle initRoom() {
    return NativeRoomHandle(NullablePointer(_initRoom(ptr.getInnerPtr())));
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    _close_room(
        ptr.getInnerPtr(), (room as NativeRoomHandle).ptr.getInnerPtr());
    room.ptr.free();
  }

  @override
  @moveSemantics
  void free() {
    _free(ptr.getInnerPtr());
    ptr.free();
  }
}
