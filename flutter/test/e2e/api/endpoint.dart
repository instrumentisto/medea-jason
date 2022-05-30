import 'package:json_annotation/json_annotation.dart';

part 'endpoint.g.dart';


// todo refact
@JsonSerializable()
class Endpoint {
  dynamic data;
  Endpoint(this.data);

  factory Endpoint.fromJson(Map<String, dynamic> json) {
    try {
      return Endpoint(WebRtcPlayEndpoint.fromJson(json));
    } catch (_) {
      return Endpoint(WebRtcPublishEndpoint.fromJson(json));
    }
  }

  Map<String, dynamic> toJson() {
    return data.toJson();
  }
}

@JsonSerializable()
class WebRtcPlayEndpoint {
  String id = ''; // skip deser
  String src;
  bool force_relay = false;

  WebRtcPlayEndpoint(this.id, this.src);
  factory WebRtcPlayEndpoint.fromJson(Map<String, dynamic> json) => _$WebRtcPlayEndpointFromJson(json);

  Map<String, dynamic> toJson() {
    var json = _$WebRtcPlayEndpointToJson(this);
    json.addAll({'kind':'WebRtcPlayEndpoint'});
    return json;
  }
}

@JsonSerializable()
class WebRtcPublishEndpoint {
  String id;
  P2pMode p2p;
  bool force_relay = false;
        @JsonKey(
      toJson: AudioSettings.toJson)
  AudioSettings audio_settings = AudioSettings(PublishPolicy.Optional); 
          @JsonKey(
      toJson: VideoSettings.toJson)
  VideoSettings video_settings = VideoSettings(PublishPolicy.Optional);
  WebRtcPublishEndpoint(this.id, this.p2p); // todo contsr

  factory WebRtcPublishEndpoint.fromJson(Map<String, dynamic> json) => _$WebRtcPublishEndpointFromJson(json);

  Map<String, dynamic> toJson() {
    var json = _$WebRtcPublishEndpointToJson(this) ;
    json.addAll({'kind':'WebRtcPublishEndpoint'});
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
  factory AudioSettings.fromJson(Map<String, dynamic> json) => _$AudioSettingsFromJson(json);
  Map<String, dynamic> _toJson() => _$AudioSettingsToJson(this);
  static Map<String, dynamic> toJson(AudioSettings value) => value._toJson();
}

@JsonSerializable()
class VideoSettings {
  PublishPolicy publish_policy = PublishPolicy.Optional;
  VideoSettings(this.publish_policy);
  factory VideoSettings.fromJson(Map<String, dynamic> json) => _$VideoSettingsFromJson(json);
  Map<String, dynamic> _toJson() => _$VideoSettingsToJson(this);
  static Map<String, dynamic> toJson(VideoSettings value) => value._toJson();
}