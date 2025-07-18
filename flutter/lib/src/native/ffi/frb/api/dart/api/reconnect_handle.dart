// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../../frb_generated.dart';
import '../api.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ReconnectHandle>>
abstract class ReconnectHandle implements RustOpaqueInterface, ForeignClass {
  /// Constructs a [`ForeignClass`] from the given raw pointer via
  /// [`Box::from_raw()`].
  ///
  /// # Safety
  ///
  /// Same as for [`Box::from_raw()`].
  static ReconnectHandle fromPtr({required int ptr}) => RustLib.instance.api
      .crateApiDartApiReconnectHandleReconnectHandleFromPtr(ptr: ptr);

  /// Tries to reconnect a [`Room`] in a loop with a growing backoff delay.
  ///
  /// The first attempt will be performed immediately, and the second attempt
  /// will be performed after `starting_delay_ms`.
  ///
  /// Delay between reconnection attempts won't be greater than
  /// `max_delay_ms`.
  ///
  /// After each reconnection attempt, delay between reconnections will be
  /// multiplied by the given `multiplier` until it reaches `max_delay_ms`.
  ///
  /// If `multiplier` is a negative number then it will be considered as
  /// `0.0`. This might cause a busy loop, so it's not recommended.
  ///
  /// Max elapsed time can be limited with an optional `max_elapsed_time_ms`
  /// argument.
  ///
  /// If the [`Room`] is already reconnecting then new reconnection attempt
  /// won't be performed. Instead, it will wait for the first reconnection
  /// attempt result and use it here.
  Object reconnectWithBackoff({
    required int startingDelay,
    required double multiplier,
    required int maxDelay,
    int? maxElapsedTimeMs,
  });

  /// Tries to reconnect a [`Room`] after the provided delay in milliseconds.
  ///
  /// If the [`Room`] is already reconnecting then new reconnection attempt
  /// won't be performed. Instead, it will wait for the first reconnection
  /// attempt result and use it here.
  Object reconnectWithDelay({required int delayMs});
}
