import 'dart:ffi';

import 'callback_listener.dart' as callback_listener;
import 'future_resolver.dart' as future_resolver;
import 'error.dart' as error;

/// Registers functions needed for platform utils working.
void registerFunctions(DynamicLibrary dl) {
  callback_listener.registerFunctions(dl);
  future_resolver.registerFunctions(dl);
  error.registerFunctions(dl);
}
