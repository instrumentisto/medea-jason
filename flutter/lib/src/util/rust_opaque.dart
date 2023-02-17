import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

/// Wrapper around a [FrbOpaque] that can be `null` once disposed.
///
/// Accessing the opaque type after it's disposed will throw a [StateError].
class RustOpaque<T extends FrbOpaque> {
  T _opaque;

  /// Constructs a [RustOpaque] value from the provided [FrbOpaque] value.
  RustOpaque(this._opaque);

  /// Returns the underlying [FrbOpaque].
  ///
  /// Throws a [StateError] if the underlying [FrbOpaque] has been freed.
  T get innerOpaque {
    if (_opaque.isStale()) {
      throw StateError('`RustOpaque` cannot be used after dispose');
    }
    return _opaque;
  }

  /// Returns the underlying [FrbOpaque] with move semantics.
  ///
  /// Throws a [StateError] if the underlying [FrbOpaque] has been freed.
  T get moveOpaque {
    if (_opaque.isStale()) {
      throw StateError('`RustOpaque` cannot be used after dispose');
    }
    _opaque.move = true;
    return _opaque;
  }

  /// Sets the inner opaque value to the provided one.
  set innerOpaque(T value) {
    _opaque = value;
  }

  /// Indicates whether this [FrbOpaque] is disposed.
  bool isStale() {
    return _opaque.isStale();
  }

  /// Disposes the underlying [FrbOpaque].
  void dispose() {
    _opaque.dispose();
  }
}
