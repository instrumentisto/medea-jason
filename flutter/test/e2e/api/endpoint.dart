import 'package:json_annotation/json_annotation.dart';

part 'endpoint.g.dart';

/// `Endpoint` element representation.
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

/// [Control API]'s `WebRtcPlayEndpoint` element representation.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
@JsonSerializable()
class WebRtcPlayEndpoint implements Endpoint {
  /// ID of this [WebRtcPlayEndpoint].
  String id = '';

  /// URI in format `local://{room_id}/{member_id}/{endpoint_id}` pointing to
  /// [WebRtcPublishEndpoint] which this [`WebRtcPlayEndpoint`] plays.
  String src;

  /// Option to relay all media through a TURN server forcibly.
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

/// [Control API]'s `WebRtcPublishEndpoint` representation.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
@JsonSerializable()
class WebRtcPublishEndpoint implements Endpoint {
  /// ID of [WebRtcPublishEndpoint].
  String id;

  /// Mode of connection for this [WebRtcPublishEndpoint].
  P2pMode p2p;

  /// Option to relay all media through a TURN server forcibly.
  bool force_relay = false;

  /// Settings for the audio media type of the [WebRtcPublishEndpoint].
  AudioSettings audio_settings = AudioSettings(PublishPolicy.Optional);

  /// Settings for the video media type of the [WebRtcPublishEndpoint].
  VideoSettings video_settings = VideoSettings(PublishPolicy.Optional);

  WebRtcPublishEndpoint(this.id, this.p2p);

  factory WebRtcPublishEndpoint.fromJson(Map<String, dynamic> json) =>
      _$WebRtcPublishEndpointFromJson(json);

  @override
  Map<String, dynamic> toJson() {
    var json = _$WebRtcPublishEndpointToJson(this);
    json.addAll({'kind': 'WebRtcPublishEndpoint'});
    return json;
  }
}

/// P2P mode of [WebRtcPublishEndpoint].
enum P2pMode {
  /// Send media data peer-to-peer only without a media server.
  Always,

  /// Always send media data through a media server.
  Never,

  /// Send media data peer-to-peer directly if it's possible, otherwise
  /// through a media server.
  IfPossible,
}

/// Publishing policy of the video or audio media type in the
/// [WebRtcPublishEndpoint].
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

/// Settings for the audio media type of the [WebRtcPublishEndpoint].
@JsonSerializable()
class AudioSettings {
  /// Publishing policy of the audio media type in the
  /// [WebRtcPublishEndpoint].
  PublishPolicy publish_policy = PublishPolicy.Optional;

  AudioSettings(this.publish_policy);

  factory AudioSettings.fromJson(Map<String, dynamic> json) =>
      _$AudioSettingsFromJson(json);
  Map<String, dynamic> toJson() => _$AudioSettingsToJson(this);
}

/// Settings for the video media type of the [WebRtcPublishEndpoint].
@JsonSerializable()
class VideoSettings {
  /// Publishing policy of the video media type in the
  /// [WebRtcPublishEndpoint].
  PublishPolicy publish_policy = PublishPolicy.Optional;

  VideoSettings(this.publish_policy);
  factory VideoSettings.fromJson(Map<String, dynamic> json) =>
      _$VideoSettingsFromJson(json);
  Map<String, dynamic> toJson() => _$VideoSettingsToJson(this);
}
