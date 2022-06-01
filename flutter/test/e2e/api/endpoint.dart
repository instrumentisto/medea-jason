import 'package:json_annotation/json_annotation.dart';

part 'endpoint.g.dart';

@JsonSerializable()
class Endpoint {
  Map<String, dynamic> toJson() => {};
  Endpoint();
  factory Endpoint.fromJson(Map<String, dynamic> json) {
    if (json.toString().contains('WebRtcPlayEndpoint')) {
      return WebRtcPlayEndpoint.fromJson(json);
    } else {
      return WebRtcPublishEndpoint.fromJson(json);
    }
  }
}

@JsonSerializable()
class WebRtcPlayEndpoint implements Endpoint {
  String id = ''; // skip deser
  String src;
  bool force_relay = false;

  WebRtcPlayEndpoint(this.id, this.src);

  factory WebRtcPlayEndpoint.fromJson(Map<String, dynamic> json) =>
      _$WebRtcPlayEndpointFromJson(json);

  @override
  Map<String, dynamic> toJson() {
    var json = _$WebRtcPlayEndpointToJson(this);
    json.addAll({'kind': 'WebRtcPlayEndpoint'});
    return json;
  }
}

@JsonSerializable()
class WebRtcPublishEndpoint implements Endpoint {
  String id;
  P2pMode p2p;
  bool force_relay = false;
  AudioSettings audio_settings = AudioSettings(PublishPolicy.Optional);
  VideoSettings video_settings = VideoSettings(PublishPolicy.Optional);

  WebRtcPublishEndpoint(this.id, this.p2p); // todo contsr

  factory WebRtcPublishEndpoint.fromJson(Map<String, dynamic> json) =>
      _$WebRtcPublishEndpointFromJson(json);

  @override
  Map<String, dynamic> toJson() {
    var json = _$WebRtcPublishEndpointToJson(this);
    json.addAll({'kind': 'WebRtcPublishEndpoint'});
    return json;
  }
}

enum P2pMode {
  Always,
  Never,
  IfPossible,
}

enum PublishPolicy {
  Optional,
  Required,
  Disabled,
}

@JsonSerializable()
class AudioSettings {
  PublishPolicy publish_policy = PublishPolicy.Optional;
  AudioSettings(this.publish_policy);
  factory AudioSettings.fromJson(Map<String, dynamic> json) =>
      _$AudioSettingsFromJson(json);
  Map<String, dynamic> toJson() => _$AudioSettingsToJson(this);
}

@JsonSerializable()
class VideoSettings {
  PublishPolicy publish_policy = PublishPolicy.Optional;
  VideoSettings(this.publish_policy);
  factory VideoSettings.fromJson(Map<String, dynamic> json) =>
      _$VideoSettingsFromJson(json);
  Map<String, dynamic> toJson() => _$VideoSettingsToJson(this);
}
