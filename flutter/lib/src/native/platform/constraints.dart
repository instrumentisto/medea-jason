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
  try {
    val as Map<dynamic, dynamic>;
    var videoConst = val['video'] as Map<dynamic, dynamic>;
    var optional = videoConst['optional'] as Map<dynamic, dynamic>;
    var deviceId;
    if (optional['sourceId'] != null) {
      deviceId = optional['sourceId'];
    }
    var facingMode;
    if (optional['facingMode'] != null) {
      if (optional['facingMode'] == 'user') {
        facingMode = FacingMode.user;
      } else {
        facingMode = FacingMode.environment;
      }
    }
    cons.video.optional = VideoConstraints(deviceId, facingMode);
  } catch (e) {
    print("Exception fuck: " + e.toString());
  }
}
