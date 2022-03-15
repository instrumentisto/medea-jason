import 'move_semantic.dart';

abstract class FreeableHandle {
  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}

/// Store for the all Rust handles created returned from Rust.
class RustHandlesStorage {
  static final RustHandlesStorage _singleton = RustHandlesStorage._internal();

  /// All handles given for the Dart side from the Rust side.
  List<dynamic> _handles = [];

  factory RustHandlesStorage() {
    return _singleton;
  }

  RustHandlesStorage._internal();

  /// Insert provided [handle] to this [RustHandlesStorage].
  void insertHandle(dynamic handle) {
    _handles.add(handle);
  }

  /// Disposes all Rust handles registered in this [RustHandlesStorage].
  void freeAll() {
    _handles.forEach((h) {
      h as FreeableHandle;
      h.free();
    });
    _handles = [];
  }
}
