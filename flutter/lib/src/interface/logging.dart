import 'enums.dart' show LogLevel;

import '../native/jason.dart'
    if (dart.library.js_interop) '../web/jason.dart'
    as impl;

/// Handle for managing logging settings in the whole library and all its
/// subsystems
final class Logging {
  Logging._();

  /// Sets the global maximum [LogLevel].
  ///
  /// This can be called at any time in runtime to change the current level.
  static Future<void> setLogLevel(LogLevel level) => impl.setLogLevel(level);
}
