import 'dart:collection';

import 'move_semantic.dart';

/// Abstraction of a handle to an object allocated in the Rust side.
abstract class PlatformHandle {}

/// A [PlatformHandle] with an async destructor.
abstract class AsyncPlatformHandle implements PlatformHandle {
  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  Future<void> free();
}

/// A [PlatformHandle] with a sync destructor.
abstract class SyncPlatformHandle implements PlatformHandle {
  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}

/// Store for all the Rust handles created and returned from the Rust side.
class RustHandlesStorage {
  static final RustHandlesStorage _singleton = RustHandlesStorage._internal();

  /// All handles given for the Dart side from the Rust side.
  final HashSet<PlatformHandle> _handles = HashSet();

  /// Indicator whether this [RustHandlesStorage] frees all the [_handles].
  bool _isFreeingAll = false;

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
    if (!_isFreeingAll) {
      _handles.remove(handle);
    }
  }

  /// Disposes all the Rust handles registered in this [RustHandlesStorage].
  Future<void> freeAll() async {
    _isFreeingAll = true;
    for (var h in _handles.toList()) {
      if (h is AsyncPlatformHandle) {
        await h.free();
      } else {
        h as SyncPlatformHandle;
        h.free();
      }
    }
    _handles.clear();
    _isFreeingAll = false;
  }
}
