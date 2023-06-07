// ignore_for_file: constant_identifier_names, depend_on_referenced_packages, non_constant_identifier_names

import 'package:json_annotation/json_annotation.dart';

part 'endpoint.g.dart';

/// Media element flowing one or more media data streams through itself.
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

/// Media element playing media data for a client via WebRTC.
@JsonSerializable()
class WebRtcPlayEndpoint implements Endpoint {
  /// ID of this [WebRtcPlayEndpoint].
  String id = '';

  /// URI in format `local://{room_id}/{member_id}/{endpoint_id}` pointing to
  /// [WebRtcPublishEndpoint] which this [WebRtcPlayEndpoint] plays.
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

/// Media element receiving media data from a client via WebRTC (allows to
/// publish media data).
@JsonSerializable()
class WebRtcPublishEndpoint implements Endpoint {
  /// ID of this [WebRtcPublishEndpoint].
  String id;

  /// Mode of connection for this [WebRtcPublishEndpoint].
  P2pMode p2p;

  /// Option to relay all media through a TURN server forcibly.
  bool force_relay = false;

  /// Settings for the audio media type of this [WebRtcPublishEndpoint].
  AudioSettings audio_settings = AudioSettings(PublishPolicy.Optional);

  /// Settings for the video media type of this [WebRtcPublishEndpoint].
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

/// Possible peer-to-peer modes of a WebRTC interaction in a
/// [WebRtcPublishEndpoint].
enum P2pMode {
  /// Send media data via peer-to-peer connections only, and never through a
  /// media server.
  Always,

  /// Never use peer-to-peer connections and always send media data through a
  /// media server.
  Never,

  /// Use peer-to-peer connections directly if it's possible, otherwise send
  /// media data through a media server.
  IfPossible,
}

/// Policy of how a video or an audio media type can be published in a
/// [WebRtcPublishEndpoint].
enum PublishPolicy {
  /// Publish this media type, if it's possible.
  Optional,

  /// Don't start call if this media type can't be published.
  Required,

  /// Media type __must__ not be published.
  ///
  /// Media server will not try to initialize publishing.
  Disabled,
}

/// Settings for an audio media type of a [WebRtcPublishEndpoint].
@JsonSerializable()
class AudioSettings {
  /// Publishing policy of the audio media type.
  PublishPolicy publish_policy = PublishPolicy.Optional;

  AudioSettings(this.publish_policy);

  factory AudioSettings.fromJson(Map<String, dynamic> json) =>
      _$AudioSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$AudioSettingsToJson(this);
}

/// Settings for a video media type of a [WebRtcPublishEndpoint].
@JsonSerializable()
class VideoSettings {
  /// Publishing policy of the video media type.
  PublishPolicy publish_policy = PublishPolicy.Optional;

  VideoSettings(this.publish_policy);

  factory VideoSettings.fromJson(Map<String, dynamic> json) =>
      _$VideoSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$VideoSettingsToJson(this);
}
