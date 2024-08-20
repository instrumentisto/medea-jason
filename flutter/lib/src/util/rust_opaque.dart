import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Wrapper around a [FrbOpaque] that can be `null` once disposed.
///
/// Accessing the opaque type after it's disposed will throw a [StateError].
class RustOpaque<T extends RustOpaqueInterface> {
  T _opaque;

  /// Constructs a [RustOpaque] value from the provided [FrbOpaque] value.
  RustOpaque(this._opaque);

  /// Returns the underlying [FrbOpaque].
  ///
  /// Throws a [StateError] if the underlying [FrbOpaque] has been freed.
  T get inner {
    if (_opaque.isDisposed) {
      throw StateError('`RustOpaque` cannot be used after dispose');
    }
    return _opaque;
  }

  /// Indicates whether this [FrbOpaque] is disposed.
  bool get isDisposed => _opaque.isDisposed;

  /// Sets the inner opaque value to the provided one.
  set inner(T value) {
    _opaque = value;
  }

  /// Disposes the underlying [FrbOpaque].
  void dispose() {
    _opaque.dispose();
  }
}
