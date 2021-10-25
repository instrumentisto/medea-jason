import 'dart:ffi';

import 'object.dart' as object;

/// Registers functions needed for platform utils working.
void registerFunctions(DynamicLibrary dl) {
  object.registerFunctions(dl);
}
