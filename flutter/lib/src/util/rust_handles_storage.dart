import 'dart:collection';

import 'move_semantic.dart';

/// Abstraction of a handle to an object allocated in the Rust side.
abstract class PlatformHandle {
  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}

/// Store for all the Rust handles created and returned from the Rust side.
class RustHandlesStorage {
  static final RustHandlesStorage _singleton = RustHandlesStorage._internal();

  /// All handles given for the Dart side from the Rust side.
  final HashSet<PlatformHandle> _handles = HashSet();

  factory RustHandlesStorage() {
    return _singleton;
  }

  RustHandlesStorage._internal();

  /// Insert the provided [handle] to this [RustHandlesStorage].
  void insertHandle(PlatformHandle handle) {
    _handles.add(handle);
  }

  /// Removes the provided [handle] from this [RustHandlesStorage].
  void removeHandle(PlatformHandle handle) {
    _handles.remove(handle);
  }

  /// Disposes all the Rust handles registered in this [RustHandlesStorage].
  void freeAll() {
    _handles.forEach((h) {
      h.free();
    });
    _handles.clear();
  }
}
