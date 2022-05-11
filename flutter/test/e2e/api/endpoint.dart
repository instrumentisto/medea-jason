import 'package:medea_jason/medea_jason.dart';

class Endpoint {
  late String type;
  late WebRtcPublishEndpoint? data1;
  late WebRtcPlayEndpoint? data2;

  Map<String, dynamic> toJson() {
    if (type == 'WebRtcPublishEndpoint') {
      return {
        type: {data1!.toString()}
      };
    }
    return {
      type: {data2!.toString()}
    };
  }
}

class WebRtcPlayEndpoint {
  late String id; // skip deser
  late String src;
  late bool force_relay; // default
}

class WebRtcPublishEndpoint {
  late String id;
  late P2pMode p2p;
  late bool force_relay; // default
  late Object audio_settings; // default
  late Object video_settings; // default

}

enum P2pMode {
  Always,
  Never,
  IfPossible,
}

enum PublishPolicy {
  /// Publish this media type if it possible.
  Optional,

  /// Don't start call if this media type can't be published.
  Required,

  /// Media type __must__ not be published.
  ///
  /// Media server will not try to initialize publishing.
  Disabled,
}
