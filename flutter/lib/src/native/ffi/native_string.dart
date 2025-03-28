import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '../../util/move_semantic.dart';
import '../jason.dart';

typedef _FreeC = Void Function(Pointer<Utf8>);
typedef _FreeDart = void Function(Pointer<Utf8>);

/// Frees [String] returned from Rust.
final _free = dl.lookupFunction<_FreeC, _FreeDart>('String_free');

extension RustStringPointer on Pointer<Utf8> {
  /// Converts this [RustStringPointer] to a Dart's [String].
  @moveSemantics
  String nativeStringToDartString() {
    try {
      return toDartString();
    } finally {
      _free(this);
    }
  }
}

/// Registers functions required for supporting Dart allocated strings in Rust.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
    'register_free_dart_native_string',
  )(Pointer.fromFunction<Void Function(Pointer<Utf8>)>(_freeDartNativeString));
}

/// Releases native heap memory for the given [string].
void _freeDartNativeString(Pointer<Utf8> string) {
  calloc.free(string);
}
