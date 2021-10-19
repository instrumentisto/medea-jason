import 'dart:ffi';

import 'callback_listener.dart' as callback_listener;
import 'future_resolver.dart' as future_resolver;

/// Registers functions needed for platform utils working.
void registerFunctions(DynamicLibrary dl) {
  callback_listener.registerFunctions(dl);
  future_resolver.registerFunctions(dl);
}
