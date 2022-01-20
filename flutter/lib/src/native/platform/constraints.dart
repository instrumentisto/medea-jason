import 'dart:ffi';
import 'constraints.g.dart' as bridge;

import 'package:flutter_webrtc/src/model/constraints.dart';

/// Registers functions allowing Rust to operate Dart [MediaStreamConstraints].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_new),
    audio: Pointer.fromFunction(_setAudio),
    video: Pointer.fromFunction(_setVideo),
  );
}

/// Returns empty [MediaStreamConstraints].
Object _new() {
  return Constraints();
}

// TODO(evdokimovs): fix audio constraints setting
/// Sets [MediaStreamConstraints.audio] for the provided [cons].
void _setAudio(Constraints cons, Object val) {
  // cons.audio = val;
}

// TODO(evdokimovs): fix video constraints setting
/// Sets [MediaStreamConstraints.video] for the provided [cons].
void _setVideo(Constraints cons, Object val) {
  // cons.video = val;
}
