import 'dart:ffi';

import 'callback_listener.dart' as callback_listener;

void registerFunctions(DynamicLibrary dl) {
  callback_listener.registerFunctions(dl);
}
