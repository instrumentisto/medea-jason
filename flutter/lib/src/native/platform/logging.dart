import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'logging.g.dart' as bridge;

/// Registers logging-related Dart callbacks so Rust can notify about log level
/// changes.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl, setWebrtcLogLevel: _setWebrtcLogLevel);
}

/// Propagates desired logging level to
/// [medea_flutter_webrtc](https://github.com/instrumentisto/medea-flutter-webrtc).
Future<void> Function() _setWebrtcLogLevel(int level) {
  return () => webrtc.setLogLevel(webrtc.LogLevel.values[level]);
}
