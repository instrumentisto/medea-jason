import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<Utf8> Function(Object) id,
  required Pointer Function(Object) deviceId,
  required Pointer Function(Object) facingMode,
  required Pointer Function(Object) height,
  required Pointer Function(Object) width,
  required void Function(Object, int) setEnabled,
  required void Function(Object) stop,
  required int Function(Object) enabled,
  required int Function(Object) kind,
  required int Function(Object) readyState,
  required void Function(Object, Object) onEnded,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer)>('register_media_stream_track')(
    Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(id),
    Pointer.fromFunction<Pointer Function(Handle)>(deviceId),
    Pointer.fromFunction<Pointer Function(Handle)>(facingMode),
    Pointer.fromFunction<Pointer Function(Handle)>(height),
    Pointer.fromFunction<Pointer Function(Handle)>(width),
    Pointer.fromFunction<Void Function(Handle, Int32)>(setEnabled),
    Pointer.fromFunction<Void Function(Handle)>(stop),
    Pointer.fromFunction<Int64 Function(Handle)>(enabled, 0),
    Pointer.fromFunction<Int64 Function(Handle)>(kind, 0),
    Pointer.fromFunction<Int64 Function(Handle)>(readyState, 0),
    Pointer.fromFunction<Void Function(Handle, Handle)>(onEnded),
  );
}
