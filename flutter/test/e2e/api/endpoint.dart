import 'package:medea_jason/medea_jason.dart';
import 'package:json_annotation/json_annotation.dart';

part 'endpoint.g.dart';

@JsonSerializable()
class Endpoint {
  late dynamic data;
    static Endpoint fromJ(Map<String, dynamic> json) {
    try {
      var res = Endpoint();
      res.data = WebRtcPlayEndpoint.fromJson(json);
      return res;
    } catch (e) {
      var res = Endpoint();
      res.data = WebRtcPublishEndpoint.fromJson(json);
      return res;
    }
  }

  Endpoint();
  factory Endpoint.fromJson(Map<String, dynamic> json) {
    try {
      var res = Endpoint();
      res.data = WebRtcPlayEndpoint.fromJson(json);
      return res;
    } catch (e) {
      var res = Endpoint();
      res.data = WebRtcPublishEndpoint.fromJson(json);
      return res;
    }
  }

  Map<String, dynamic> toJson() {
    return data.toJson();
  }
}

@JsonSerializable()
class WebRtcPlayEndpoint {
  late String id; // skip deser
  late String src;
  bool force_relay = false; // default

  WebRtcPlayEndpoint();
  factory WebRtcPlayEndpoint.fromJson(Map<String, dynamic> json) => _$WebRtcPlayEndpointFromJson(json);

  Map<String, dynamic> toJson() {
    var gg = _$WebRtcPlayEndpointToJson(this);
    gg.addAll({'kind':'WebRtcPlayEndpoint'});
    return gg;
  }
}

@JsonSerializable()
class WebRtcPublishEndpoint {
  late String id;
  late P2pMode p2p;
  late bool force_relay = false; // default
  AudioSettings audio_settings = AudioSettings(PublishPolicy.Optional); // default
  VideoSettings video_settings = VideoSettings(PublishPolicy.Optional); // default
  WebRtcPublishEndpoint();
  factory WebRtcPublishEndpoint.fromJson(Map<String, dynamic> json) => _$WebRtcPublishEndpointFromJson(json);

  Map<String, dynamic> toJson() {
    var gg = _$WebRtcPublishEndpointToJson(this) ;
    gg.addAll({'kind':'WebRtcPublishEndpoint'});
    gg['audio_settings'] = audio_settings.toJson();
    gg['video_settings'] = video_settings.toJson();
    return gg;
    }
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

@JsonSerializable()
class AudioSettings {
  PublishPolicy publish_policy = PublishPolicy.Optional;
  AudioSettings(this.publish_policy);
  factory AudioSettings.fromJson(Map<String, dynamic> json) => _$AudioSettingsFromJson(json);
  Map<String, dynamic> toJson() => _$AudioSettingsToJson(this);
}

@JsonSerializable()
class VideoSettings {
  PublishPolicy publish_policy = PublishPolicy.Optional;
  VideoSettings(this.publish_policy);
  factory VideoSettings.fromJson(Map<String, dynamic> json) => _$VideoSettingsFromJson(json);
  Map<String, dynamic> toJson() => _$VideoSettingsToJson(this);

}